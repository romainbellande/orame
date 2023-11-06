use std::{fmt::Debug, sync::Arc};

use std::net::SocketAddr;

use axum::middleware;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::Response,
    routing::get,
    Extension, Router,
};
use ogame_core::planet::Planet;
use ogame_core::resources::Resources;
use tower_http::services::{ServeDir, ServeFile};

use crate::auth::middleware::auth_bearer_middleware;
use crate::auth::Claims;

use prisma_client::*;
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
    Extension(claims): Extension<Claims>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(conn): Extension<Arc<PrismaClient>>,
) -> Response {
    println!("New connection from {}", addr);
    println!("Claims: {:#?}", claims);
    debug!("New connection from {}", addr);
    ws.on_upgrade(move |socket| handle_socket::<P>(socket, addr, claims.sub, conn))
}

fn protocol_from_bytes<P: 'static + Serialize + DeserializeOwned + Debug>(bytes: &[u8]) -> P {
    serde_cbor::from_slice(bytes).unwrap()
}

fn protocol_to_bytes<P: 'static + Serialize + DeserializeOwned + Debug>(packet: P) -> Vec<u8> {
    serde_cbor::to_vec(&packet).unwrap()
}

/* fn translate_db_into_game(db: Planet) -> Planet {
    /* let mut planet = Planet::new(db.name, db.coordinates.into(), db.resources.into());
    planet.buildings = db.buildings.into();
    planet.build_queue = db.build_queue.into();
    planet.ships = db.ships.into();
    planet.ship_queue = db.ship_queue.into();
    planet */
} */

async fn fetch_game(user_id: String, conn: Arc<PrismaClient>) -> Game {
    use prisma_client::{coordinates, planet, user, PrismaClient};

    let user_game = conn
        .user()
        .find_first(vec![user::id::equals(user_id)])
        .with(
            user::planets::fetch(vec![])
                .with(planet::coordinates::fetch())
                .with(planet::resources::fetch())
                .with(planet::buildings::fetch())
                .with(planet::ships::fetch()),
        )
        .exec()
        .await
        .unwrap()
        .unwrap();

    println!("GOT USER GAME: {:#?}", user_game);

    user_game.into()
}

async fn handle_socket<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
    user_id: String,
    conn: Arc<PrismaClient>,
) {
    let (mut tx, mut rx) = socket.split();

    let game = fetch_game(user_id, conn).await;
    println!("Game fetched: {:#?}", game);

    let game_packet = protocol_to_bytes(Protocol::Game(game));
    tx.send(game_packet.into()).await.unwrap();

    while let Some(msg) = rx.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            break;
        };
        let msg_tmp = msg.into_data();

        if msg_tmp.is_empty() {
            return;
        }

        println!("{:#?}", msg_tmp);

        let msg = protocol_from_bytes::<P>(&msg_tmp);
        println!("Received msg: {:#?}", msg);
    }

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
