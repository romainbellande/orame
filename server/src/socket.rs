use std::{fmt::Debug, net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    middleware,
    response::Response,
    routing::get,
    Extension, Router,
};
use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use log::*;
use prisma_client::*;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::Receiver;
use tower_http::services::{ServeDir, ServeFile};

use ogame_core::protocol::Protocol;

use crate::{
    auth::{middleware::auth_bearer_middleware, Claims},
    client,
    connected_users::ConnectedUsers,
    error::*,
};

pub async fn run<P: Serialize + DeserializeOwned + Debug + 'static>() -> Result<()> {
    let db = PrismaClient::_builder().build().await?;

    let db = Arc::new(db);

    let ws_router = Router::new()
        .route("/", get(ws_handler::<P>))
        .route_layer(middleware::from_fn(auth_bearer_middleware));

    let app = Router::new()
        .nest("/ws", ws_router)
        .nest("/auth", crate::auth::router())
        .layer(Extension(db))
        .layer(Extension(ConnectedUsers::empty()))
        .fallback_service(
            ServeDir::new("public").not_found_service(ServeFile::new("public/index.html")),
        );

    let addr = "0.0.0.0:8080";

    info!("Listening on {}", addr);

    Ok(axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?)
}

async fn ws_handler<P: Serialize + DeserializeOwned + Debug + 'static>(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(conn): Extension<Arc<PrismaClient>>,
    Extension(connected_users): Extension<ConnectedUsers>,
) -> Response {
    ws.on_upgrade(move |socket| {
        handle_client_with_error::<P>(socket, addr, claims.sub, connected_users, conn)
    })
}

async fn handle_client_with_error<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    addr: SocketAddr,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: Arc<PrismaClient>,
) {
    if let Err(e) =
        handle_client::<P>(socket, addr, user_id.clone(), connected_users.clone(), conn).await
    {
        error!("Error handling client: {:?}", e);
    }

    connected_users.remove(user_id).await;
}

async fn handle_client<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: Arc<PrismaClient>,
) -> Result<()> {
    let (tx, rx) = socket.split();

    let user_rx = connected_users.add(user_id.clone()).await;

    tokio::task::spawn(listen_to_others_messages(user_rx, tx));

    client::send_initial_game(user_id.clone(), connected_users.clone(), &conn).await?;

    message_loop(rx, user_id.clone(), connected_users.clone(), &conn).await?;

    Ok(())
}

async fn listen_to_others_messages(
    mut user_rx: Receiver<Protocol>,
    mut tx: SplitSink<WebSocket, Message>,
) -> Result<()> {
    while let Some(msg) = user_rx.recv().await {
        let msg = protocol_to_bytes(msg)?;
        tx.send(msg.into()).await?;
    }

    Ok(())
}

async fn message_loop(
    mut rx: SplitStream<WebSocket>,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    while let Some(msg) = rx.next().await {
        let protocol = msg_to_protocol(msg)?;

        client::handle_msg(user_id.clone(), protocol, connected_users.clone(), conn).await?;
    }

    Ok(())
}

fn msg_to_protocol(msg: std::result::Result<Message, axum::Error>) -> Result<Protocol> {
    let msg = match msg {
        Ok(msg) => msg,
        Err(_) => return Err(Error::ClientDisconnected),
    };

    let msg_tmp = msg.into_data();

    if msg_tmp.is_empty() {
        return Err(Error::ClientDisconnected);
    }

    protocol_from_bytes(&msg_tmp)
}

fn protocol_from_bytes(bytes: &[u8]) -> Result<Protocol> {
    Ok(serde_cbor::from_slice(bytes)?)
}

fn protocol_to_bytes(packet: Protocol) -> Result<Vec<u8>> {
    Ok(serde_cbor::to_vec(&packet)?)
}
