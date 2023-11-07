use std::sync::Arc;

use crate::{buildings, planet, resources, ships};
use ogame_core::game::Game;

use crate::PrismaClient;

pub async fn save_game(game: Game, conn: &Arc<PrismaClient>) {
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
