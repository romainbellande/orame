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
    ws.on_upgrade(move |socket| handle_socket::<P>(socket, addr, claims.sub, conn))
}

fn protocol_from_bytes(bytes: &[u8]) -> Protocol {
    serde_cbor::from_slice(bytes).unwrap()
}

fn protocol_to_bytes(packet: Protocol) -> Vec<u8> {
    serde_cbor::to_vec(&packet).unwrap()
}

async fn fetch_game(user_id: String, conn: &Arc<PrismaClient>) -> Game {
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

async fn save_game(game: Game, conn: &Arc<PrismaClient>) {
    use prisma_client::{coordinates, planet, user, PrismaClient};

    for (_, planet) in game.planets {
        conn.planet()
            .update(
                planet::id::equals(planet.id.clone()),
                vec![
                    planet::build_queue::set(
                        serde_json::to_string(&planet.build_queue.items).unwrap(),
                    ),
                    planet::ship_queue::set(
                        serde_json::to_string(&planet.ship_queue.items).unwrap(),
                    ),
                    planet::last_update::set(planet.last_update as i32),
                ],
            )
            .exec()
            .await
            .unwrap();

        conn.resources()
            .update(
                resources::planet_id::equals(planet.id.clone()),
                vec![
                    resources::metal::set(planet.resources.metal),
                    resources::crystal::set(planet.resources.crystal),
                    resources::deuterium::set(planet.resources.deuterium),
                ],
            )
            .exec()
            .await
            .unwrap();

        conn.buildings()
            .update(
                buildings::planet_id::equals(planet.id.clone()),
                vec![
                    buildings::metal::set(
                        planet.buildings[&ogame_core::building_type::BuildingType::Metal] as i32,
                    ),
                    buildings::crystal::set(
                        planet.buildings[&ogame_core::building_type::BuildingType::Crystal] as i32,
                    ),
                    buildings::deuterium::set(
                        planet.buildings[&ogame_core::building_type::BuildingType::Deuterium]
                            as i32,
                    ),
                    buildings::shipyard::set(
                        planet.buildings[&ogame_core::building_type::BuildingType::Shipyard] as i32,
                    ),
                ],
            )
            .exec()
            .await
            .unwrap();

        conn.ships()
            .update(
                ships::planet_id::equals(planet.id.clone()),
                vec![
                    ships::small_cargo::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::SmallCargo] as i32,
                    ),
                    ships::large_cargo::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::LargeCargo] as i32,
                    ),
                    ships::colony_ship::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::ColonyShip] as i32,
                    ),
                    ships::recycler::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Recycler] as i32,
                    ),
                    ships::espionage_probe::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::EspionageProbe] as i32,
                    ),
                    ships::solar_satellite::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::SolarSatellite] as i32,
                    ),
                    ships::light_fighter::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::LightFighter] as i32,
                    ),
                    ships::heavy_fighter::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::HeavyFighter] as i32,
                    ),
                    ships::cruiser::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Cruiser] as i32,
                    ),
                    ships::battleship::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Battleship] as i32,
                    ),
                    ships::bomber::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Bomber] as i32,
                    ),
                    ships::destroyer::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Destroyer] as i32,
                    ),
                    ships::battlecruiser::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Battlecruiser] as i32,
                    ),
                    ships::deathstar::set(
                        planet.ships.ships[&ogame_core::ship_type::ShipType::Deathstar] as i32,
                    ),
                ],
            )
            .exec()
            .await
            .unwrap();
    }
}

async fn handle_socket<P: Serialize + DeserializeOwned + Debug + 'static>(
    socket: WebSocket,
    _addr: SocketAddr,
    user_id: String,
    conn: Arc<PrismaClient>,
) {
    let (mut tx, mut rx) = socket.split();

    {
        let mut game = fetch_game(user_id.clone(), &conn).await;
        game.tick().unwrap();
        println!("Game fetched: {:#?}", game);

        let game_packet = protocol_to_bytes(Protocol::Game(game));
        tx.send(game_packet.into()).await.unwrap();
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
        println!("Received msg: {:#?}", msg);

        apply_to_game(user_id.clone(), msg, &conn).await;
    }

    // client disconnected
}

async fn apply_to_game(user_id: String, message: Protocol, conn: &Arc<PrismaClient>) {
    let mut game = fetch_game(user_id, conn).await;

    game.process_message(message).unwrap();

    save_game(game, conn).await;
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
