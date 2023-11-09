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
use ogame_core::{fleet::Fleet, game::Game, protocol::Protocol};
use prisma_client::*;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::Receiver;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    apply_to_game::{apply_msg_to_game, apply_to_game_with},
    auth::{middleware::auth_bearer_middleware, Claims},
    connected_users::ConnectedUsers,
};

async fn handler<P: Serialize + DeserializeOwned + Debug + 'static>(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(conn): Extension<Arc<PrismaClient>>,
    Extension(connected_users): Extension<ConnectedUsers>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket::<P>(socket, addr, claims.sub, connected_users, conn))
}

fn protocol_from_bytes(bytes: &[u8]) -> Protocol {
    serde_cbor::from_slice(bytes).unwrap()
}

fn protocol_to_bytes(packet: Protocol) -> Vec<u8> {
    serde_cbor::to_vec(&packet).unwrap()
}

pub async fn handle_flight(
    game: &mut Game,
    message: Protocol,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) {
    // special case for flight
    if let Protocol::SendShips {
        from_planet_id,
        to_planet_id,
        ships,
        mission,
        resources,
        speed_ratio,
    } = message
    {
        // fetch to_planet from db
        let to_planet = conn
            .planet()
            .find_unique(prisma_client::planet::id::equals(to_planet_id.clone()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        // create flight
        let flight = game
            .create_flight(
                "".to_string(),
                from_planet_id,
                to_planet_id.clone(),
                &((*to_planet.coordinates.unwrap()).into()),
                Fleet::new("".to_string(), ships),
                resources,
                mission,
                speed_ratio,
            )
            .unwrap();

        // save it in db
        let ships = create_ships(&flight.ships.ships, conn).await;
        let resources = create_resources(&flight.resources, conn).await;
        let db_flight = create_flight(flight, ships.id, resources.id, conn).await;

        // send it to the two players
        let msg = Protocol::InboundFleet(db_flight.into());

        // get player id from to_planet
        let target_planet = fetch_planet(to_planet_id, conn).await;

        connected_users
            .send(game.player_id.clone(), msg.clone())
            .await;

        game.process_message(msg.clone()).unwrap();

        // if target player is different from current player then send it to him
        if target_planet.user_id != game.player_id {
            connected_users
                .send(target_planet.user_id.clone(), msg.clone())
                .await;

            apply_to_game_with(target_planet.user_id.clone(), conn, move |game| {
                game.process_message(msg.clone()).unwrap();
            })
            .await;
        }
    }
}

async fn listen_to_others_messages(
    mut user_rx: Receiver<Protocol>,
    mut tx: SplitSink<WebSocket, Message>,
) {
    while let Some(msg) = user_rx.recv().await {
        let msg = protocol_to_bytes(msg);
        tx.send(msg.into()).await.unwrap();
    }
}

async fn send_initial_game(
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) {
    let mut game = fetch_game(user_id.clone(), &conn).await;

    game.tick().unwrap();

    connected_users
        .send(user_id.clone(), Protocol::Game(game.clone()))
        .await;

    save_game(game, &conn).await;
}

fn msg_to_protocol(msg: Result<Message, axum::Error>) -> Result<Protocol, ()> {
    let msg = match msg {
        Ok(msg) => msg,
        Err(_) => return Err(()),
    };

    let msg_tmp = msg.into_data();

    if msg_tmp.is_empty() {
        return Err(());
    }

    Ok(protocol_from_bytes(&msg_tmp))
}

async fn message_loop(
    mut rx: SplitStream<WebSocket>,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(msg) = rx.next().await {
        let protocol = msg_to_protocol(msg).unwrap();

        apply_msg_to_game(user_id.clone(), protocol, connected_users.clone(), conn).await;
    }

    Ok(())
}

async fn handle_socket<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: Arc<PrismaClient>,
) {
    let (tx, rx) = socket.split();

    let user_rx = connected_users.add(user_id.clone()).await;

    tokio::task::spawn(listen_to_others_messages(user_rx, tx));

    send_initial_game(user_id.clone(), connected_users.clone(), &conn).await;

    message_loop(rx, user_id.clone(), connected_users.clone(), &conn)
        .await
        .unwrap();

    // client disconnected
}

pub async fn run<P: Serialize + DeserializeOwned + Debug + 'static>() {
    let db: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let db = Arc::new(db.unwrap());
    let ws_router = Router::new()
        .route("/", get(handler::<P>))
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

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
