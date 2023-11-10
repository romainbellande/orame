use std::{collections::BTreeMap, sync::Arc};

use crate::{
    buildings, coordinates,
    flight::{self},
    planet, resources, ships, user,
};
use ogame_core::{
    building_type::BuildingType,
    flight::Flight,
    game::Game,
    planet::Planet,
    resources::{ResourceType, Resources},
};

use crate::PrismaClient;

pub async fn save_planet(planet: &Planet, conn: &Arc<PrismaClient>) {
    conn.planet()
        .update(
            planet::id::equals(planet.id.clone()),
            vec![
                planet::build_queue::set(serde_json::to_string(&planet.build_queue.items).unwrap()),
                planet::ship_queue::set(serde_json::to_string(&planet.ship_queue.items).unwrap()),
                planet::last_update::set(planet.last_update as i32),
            ],
        )
        .exec()
        .await
        .unwrap();
}

pub async fn save_resources(resources_id: String, resources: &Resources, conn: &Arc<PrismaClient>) {
    conn.resources()
        .update(
            resources::id::equals(resources_id.clone()),
            vec![
                resources::metal::set(resources.get(ResourceType::Metal)),
                resources::crystal::set(resources.get(ResourceType::Crystal)),
                resources::deuterium::set(resources.get(ResourceType::Deuterium)),
            ],
        )
        .exec()
        .await
        .unwrap();
}

pub async fn save_buildings(
    planet_id: String,
    buildings: &BTreeMap<BuildingType, usize>,
    conn: &Arc<PrismaClient>,
) {
    conn.buildings()
        .update(
            buildings::planet_id::equals(planet_id.clone()),
            vec![
                buildings::metal::set(
                    buildings[&ogame_core::building_type::BuildingType::Metal] as i32,
                ),
                buildings::crystal::set(
                    buildings[&ogame_core::building_type::BuildingType::Crystal] as i32,
                ),
                buildings::deuterium::set(
                    buildings[&ogame_core::building_type::BuildingType::Deuterium] as i32,
                ),
                buildings::shipyard::set(
                    buildings[&ogame_core::building_type::BuildingType::Shipyard] as i32,
                ),
            ],
        )
        .exec()
        .await
        .unwrap();
}

pub async fn save_ships(
    ships_id: String,
    ships: &BTreeMap<ogame_core::ship_type::ShipType, usize>,
    conn: &Arc<PrismaClient>,
) {
    conn.ships()
        .update(
            ships::id::equals(ships_id.clone()),
            vec![
                ships::small_cargo::set(ships[&ogame_core::ship_type::ShipType::SmallCargo] as i32),
                ships::large_cargo::set(ships[&ogame_core::ship_type::ShipType::LargeCargo] as i32),
                ships::colony_ship::set(ships[&ogame_core::ship_type::ShipType::ColonyShip] as i32),
                ships::recycler::set(ships[&ogame_core::ship_type::ShipType::Recycler] as i32),
                ships::espionage_probe::set(
                    ships[&ogame_core::ship_type::ShipType::EspionageProbe] as i32,
                ),
                ships::solar_satellite::set(
                    ships[&ogame_core::ship_type::ShipType::SolarSatellite] as i32,
                ),
                ships::light_fighter::set(
                    ships[&ogame_core::ship_type::ShipType::LightFighter] as i32,
                ),
                ships::heavy_fighter::set(
                    ships[&ogame_core::ship_type::ShipType::HeavyFighter] as i32,
                ),
                ships::cruiser::set(ships[&ogame_core::ship_type::ShipType::Cruiser] as i32),
                ships::battleship::set(ships[&ogame_core::ship_type::ShipType::Battleship] as i32),
                ships::bomber::set(ships[&ogame_core::ship_type::ShipType::Bomber] as i32),
                ships::destroyer::set(ships[&ogame_core::ship_type::ShipType::Destroyer] as i32),
                ships::battlecruiser::set(
                    ships[&ogame_core::ship_type::ShipType::Battlecruiser] as i32,
                ),
                ships::deathstar::set(ships[&ogame_core::ship_type::ShipType::Deathstar] as i32),
            ],
        )
        .exec()
        .await
        .unwrap();
}

pub async fn create_user(
    name: String,
    email: String,
    password: String,
    conn: &Arc<PrismaClient>,
) -> user::Data {
    conn.user()
        .create(name, email, password, vec![])
        .exec()
        .await
        .unwrap()
}

pub async fn create_planet(
    player_id: String,
    coordinates_id: String,
    resources_id: String,
    ships_id: String,
    conn: &Arc<PrismaClient>,
) -> planet::Data {
    conn.planet()
        .create(
            user::id::equals(player_id),
            coordinates::id::equals(coordinates_id),
            resources::id::equals(resources_id),
            // buildings::id::equals(buildings_id),
            ships::id::equals(ships_id),
            "[]".to_string(),
            "[]".to_string(),
            0,
            vec![],
        )
        .exec()
        .await
        .unwrap()
}

pub async fn create_buildings(planet_id: String, conn: &Arc<PrismaClient>) -> buildings::Data {
    conn.buildings()
        .create(0, 0, 0, 0, planet::id::equals(planet_id), vec![])
        .exec()
        .await
        .unwrap()
}

pub async fn create_coordinates(
    galaxy: i32,
    system: i32,
    position: i32,
    conn: &Arc<PrismaClient>,
) -> coordinates::Data {
    conn.coordinates()
        .create(galaxy, system, position, vec![])
        .exec()
        .await
        .unwrap()
}

pub async fn create_resources(resources: &Resources, conn: &Arc<PrismaClient>) -> resources::Data {
    conn.resources()
        .create(
            resources.get(ResourceType::Metal),
            resources.get(ResourceType::Crystal),
            resources.get(ResourceType::Deuterium),
            vec![],
        )
        .exec()
        .await
        .unwrap()
}

pub async fn create_ships(
    ships: &BTreeMap<ogame_core::ship_type::ShipType, usize>,
    conn: &Arc<PrismaClient>,
) -> ships::Data {
    conn.ships()
        .create(
            ships[&ogame_core::ship_type::ShipType::SmallCargo] as i32,
            ships[&ogame_core::ship_type::ShipType::LargeCargo] as i32,
            ships[&ogame_core::ship_type::ShipType::ColonyShip] as i32,
            ships[&ogame_core::ship_type::ShipType::Recycler] as i32,
            ships[&ogame_core::ship_type::ShipType::EspionageProbe] as i32,
            ships[&ogame_core::ship_type::ShipType::SolarSatellite] as i32,
            ships[&ogame_core::ship_type::ShipType::LightFighter] as i32,
            ships[&ogame_core::ship_type::ShipType::HeavyFighter] as i32,
            ships[&ogame_core::ship_type::ShipType::Cruiser] as i32,
            ships[&ogame_core::ship_type::ShipType::Battleship] as i32,
            ships[&ogame_core::ship_type::ShipType::Bomber] as i32,
            ships[&ogame_core::ship_type::ShipType::Destroyer] as i32,
            ships[&ogame_core::ship_type::ShipType::Battlecruiser] as i32,
            ships[&ogame_core::ship_type::ShipType::Deathstar] as i32,
            vec![],
        )
        .exec()
        .await
        .unwrap()
}

pub async fn create_flight(
    flight: Flight,
    ships_id: String,
    resources_id: String,
    conn: &Arc<PrismaClient>,
) -> flight::Data {
    conn.flight()
        .create(
            flight.player_id.clone(),
            planet::id::equals(flight.from_planet_id.clone()),
            planet::id::equals(flight.to_planet_id.clone()),
            ships::id::equals(ships_id.clone()),
            resources::id::equals(resources_id.clone()),
            flight.arrival_time as i32,
            flight.mission.to_string(),
            flight.speed_ratio as i32,
            vec![],
        )
        .exec()
        .await
        .unwrap()
}

pub async fn save_game(game: Game, conn: &Arc<PrismaClient>) {
    for (_, planet) in game.planets {
        save_planet(&planet, conn).await;
        save_resources(planet.resources.id.clone(), &planet.resources, conn).await;
        save_buildings(planet.id.clone(), &planet.buildings, conn).await;
        save_ships(planet.ships.id.clone(), &planet.ships.ships, conn).await;
    }
}
