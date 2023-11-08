use std::collections::HashMap;
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
use ogame_core::ship_hangar::ShipHangar;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use tower_http::services::{ServeDir, ServeFile};

use crate::apply_to_game::{apply_msg_to_game, apply_to_game_with_async};
use crate::auth::middleware::auth_bearer_middleware;
use crate::auth::Claims;

use prisma_client::*;

use ogame_core::game::Game;
use ogame_core::protocol::Protocol;

use serde::de::DeserializeOwned;
use serde::Serialize;

use futures::stream::StreamExt;
use futures::{Future, SinkExt};
use log::*;

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
        speed,
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
                ShipHangar::new("".to_string(), ships),
                resources,
                mission,
                speed,
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

            apply_to_game_with_async(target_planet.user_id.clone(), conn, move |mut game| {
                game.process_message(msg.clone()).unwrap();
                async move { game }
            })
            .await;
        }
    }
}

async fn handle_socket<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
    user_id: String,
    connected_users: ConnectedUsers,
    conn: Arc<PrismaClient>,
) {
    let (mut tx, mut rx) = socket.split();

    let mut user_rx = connected_users.add(user_id.clone()).await;

    let connected_users2 = connected_users.clone();
    let user_id2 = user_id.clone();
    let conn2 = conn.clone();
    tokio::task::spawn(async move {
        while let Some(msg) = user_rx.recv().await {
            let msg = protocol_to_bytes(msg);
            tx.send(msg.into()).await.unwrap();
        }
    });

    {
        let mut game = fetch_game(user_id.clone(), &conn).await;
        game.tick().unwrap();

        connected_users
            .send(user_id.clone(), Protocol::Game(game.clone()))
            .await;

        save_game(game, &conn).await;
    }

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

        let msg = protocol_from_bytes(&msg_tmp);

        apply_msg_to_game(user_id.clone(), msg, connected_users.clone(), &conn).await;
    }

    // client disconnected
}

#[derive(Clone)]
pub struct ConnectedUsers {
    pub users: Arc<RwLock<HashMap<String, Sender<Protocol>>>>,
}

impl ConnectedUsers {
    pub fn empty() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&self, user_id: String) -> Receiver<Protocol> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        self.users.write().await.insert(user_id, tx);
        rx
    }

    pub async fn remove(&self, user_id: String) {
        self.users.write().await.remove(&user_id);
    }

    pub async fn send(&self, user_id: String, message: Protocol) {
        if let Some(sender) = self.users.write().await.get_mut(&user_id) {
            sender.try_send(message).unwrap();
        }
    }
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
