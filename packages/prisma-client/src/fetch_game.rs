use std::{collections::BTreeMap, sync::Arc};

use ogame_core::{fleet::Fleet, game::Game};

use crate::{buildings, coordinates, flight, planet, resources, ships, user, PrismaClient};

pub async fn fetch_game(user_id: String, conn: &Arc<PrismaClient>) -> Game {
    let user_game = conn
        .user()
        .find_first(vec![user::id::equals(user_id)])
        .with(
            user::planets::fetch(vec![])
                .with(planet::coordinates::fetch())
                .with(planet::resources::fetch())
                .with(planet::buildings::fetch())
                .with(planet::ships::fetch())
                .with(planet::out_flights::fetch(vec![]).with(flight::ships::fetch()))
                .with(planet::in_flights::fetch(vec![]).with(flight::ships::fetch())),
        )
        .exec()
        .await
        .unwrap()
        .unwrap();

    user_game.into()
}

pub async fn fetch_planet(planet_id: String, conn: &Arc<PrismaClient>) -> planet::Data {
    conn.planet()
        .find_first(vec![planet::id::equals(planet_id.clone())])
        .with(planet::coordinates::fetch())
        .with(planet::resources::fetch())
        .with(planet::buildings::fetch())
        .with(planet::ships::fetch())
        .with(planet::out_flights::fetch(vec![]).with(flight::ships::fetch()))
        .with(planet::in_flights::fetch(vec![]).with(flight::ships::fetch()))
        .exec()
        .await
        .unwrap()
        .unwrap()
}

impl From<user::Data> for ogame_core::game::Game {
    fn from(db_user: user::Data) -> Self {
        let mut game = ogame_core::game::Game::new();
        game.user_id = db_user.id;
        game.planets = db_user
            .planets
            .unwrap()
            .into_iter()
            .map(|p| (p.id.clone(), p.into()))
            .collect();
        game
    }
}

impl From<planet::Data> for ogame_core::planet::Planet {
    fn from(db_planet: planet::Data) -> Self {
        ogame_core::planet::Planet::new(
            db_planet.id,
            (*db_planet.coordinates.unwrap()).into(),
            (*db_planet.resources.unwrap()).into(),
            (*db_planet.buildings.unwrap().unwrap()).into(),
            (*db_planet.ships.unwrap()).into(),
            ogame_core::build_queue::BuildQueue::new(
                serde_json::from_str(&db_planet.build_queue).unwrap(),
            ),
            ogame_core::build_queue::BuildQueue::new(
                serde_json::from_str(&db_planet.ship_queue).unwrap(),
            ),
            db_planet.last_update as usize,
            db_planet
                .out_flights
                .unwrap()
                .into_iter()
                .chain(db_planet.in_flights.unwrap())
                .map(|f| f.into())
                .collect(),
        )
    }
}

impl From<flight::Data> for ogame_core::flight::Flight {
    fn from(flight: flight::Data) -> Self {
        ogame_core::flight::Flight::new(
            flight.id,
            flight.user_id,
            flight.from_planet_id,
            flight.to_planet_id,
            (*flight.ships.unwrap()).into(),
            (*flight.resources.unwrap()).into(),
            flight.mission.into(),
            flight.speed_ratio as usize,
            flight.arrival_time as usize,
            flight.return_time.map(|t| t as usize),
        )
    }
}

impl From<coordinates::Data> for ogame_core::coordinates::Coordinates {
    fn from(db_coordinates: coordinates::Data) -> Self {
        ogame_core::coordinates::Coordinates::new(
            db_coordinates.galaxy as usize,
            db_coordinates.system as usize,
            db_coordinates.position as usize,
        )
    }
}

impl From<resources::Data> for ogame_core::resources::Resources {
    fn from(db_resources: resources::Data) -> Self {
        ogame_core::resources::Resources::from([
            (
                ogame_core::resources::ResourceType::Metal,
                db_resources.metal,
            ),
            (
                ogame_core::resources::ResourceType::Crystal,
                db_resources.crystal,
            ),
            (
                ogame_core::resources::ResourceType::Deuterium,
                db_resources.deuterium,
            ),
        ])
        .set_id(db_resources.id)
    }
}

impl From<buildings::Data> for BTreeMap<ogame_core::building_type::BuildingType, usize> {
    fn from(db_buildings: buildings::Data) -> Self {
        let mut buildings = BTreeMap::new();
        buildings.insert(
            ogame_core::building_type::BuildingType::Metal,
            db_buildings.metal as usize,
        );
        buildings.insert(
            ogame_core::building_type::BuildingType::Crystal,
            db_buildings.crystal as usize,
        );
        buildings.insert(
            ogame_core::building_type::BuildingType::Deuterium,
            db_buildings.deuterium as usize,
        );
        buildings.insert(
            ogame_core::building_type::BuildingType::Shipyard,
            db_buildings.shipyard as usize,
        );

        buildings
    }
}

impl From<ships::Data> for Fleet {
    fn from(db_ships: ships::Data) -> Self {
        let mut ships = BTreeMap::new();
        ships.insert(
            ogame_core::ship_type::ShipType::SmallCargo,
            db_ships.small_cargo as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::LargeCargo,
            db_ships.large_cargo as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::ColonyShip,
            db_ships.colony_ship as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Recycler,
            db_ships.recycler as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::EspionageProbe,
            db_ships.espionage_probe as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::SolarSatellite,
            db_ships.solar_satellite as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::LightFighter,
            db_ships.light_fighter as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::HeavyFighter,
            db_ships.heavy_fighter as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Cruiser,
            db_ships.cruiser as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Battleship,
            db_ships.battleship as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Bomber,
            db_ships.bomber as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Destroyer,
            db_ships.destroyer as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Battlecruiser,
            db_ships.battlecruiser as usize,
        );
        ships.insert(
            ogame_core::ship_type::ShipType::Deathstar,
            db_ships.deathstar as usize,
        );

        Fleet::new(db_ships.id, ships)
    }
}
