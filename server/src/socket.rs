use std::{fmt::Debug, sync::Arc};

use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::Response,
    routing::get,
    Extension, Router,
};
use tower_http::services::{ServeDir, ServeFile};

use super::db::PrismaClient;
use prisma_client_rust::NewClientError;

use ogame_core::game::Game;
use ogame_core::protocol::Protocol;

use serde::de::DeserializeOwned;
use serde::Serialize;

use futures::stream::StreamExt;
use futures::SinkExt;
use log::*;

async fn handler<P: Serialize + DeserializeOwned + Debug + 'static>(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
    debug!("New connection from {}", addr);
    ws.on_upgrade(move |socket| handle_socket::<P>(socket, addr))
}

fn protocol_from_bytes<P: 'static + Serialize + DeserializeOwned + Debug>(bytes: &[u8]) -> P {
    serde_cbor::from_slice(bytes).unwrap()
}

fn protocol_to_bytes<P: 'static + Serialize + DeserializeOwned + Debug>(packet: P) -> Vec<u8> {
    serde_cbor::to_vec(&packet).unwrap()
}

async fn handle_socket<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
) {
    let (mut tx, mut rx) = socket.split();

    let game_packet = protocol_to_bytes(Protocol::Game(Game::new()));
    tx.send(game_packet.into()).await.unwrap();

    while let Some(msg) = rx.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            break;
        };

        let msg = protocol_from_bytes::<P>(&msg.into_data());
        println!("Received msg: {:#?}", msg);
    }

    // client disconnected
}

pub async fn run<P: Serialize + DeserializeOwned + Debug + 'static>() {
    let db: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let db = Arc::new(db.unwrap());
    let app = Router::new()
        .route("/ws", get(handler::<P>))
        .nest("/auth", crate::auth::router())
        .layer(Extension(db))
        .fallback_service(
            ServeDir::new("client/dist")
                .not_found_service(ServeFile::new("client/dist/index.html")),
        );

    let addr = "0.0.0.0:8080";

    info!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}