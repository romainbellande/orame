#[allow(warnings, unused)]
mod db;

use std::collections::BTreeMap;

pub use db::*;

impl From<user::Data> for ogame_core::game::Game {
    fn from(db_user: user::Data) -> Self {
        let mut game = ogame_core::game::Game::new();
        game.player_id = db_user.id;
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
            (*db_planet.coordinates.unwrap().unwrap()).into(),
            (*db_planet.resources.unwrap().unwrap()).into(),
            (*db_planet.buildings.unwrap().unwrap()).into(),
            (*db_planet.ships.unwrap().unwrap()).into(),
            ogame_core::build_queue::BuildQueue::new(
                serde_json::from_str(&db_planet.build_queue).unwrap(),
            ),
            ogame_core::build_queue::BuildQueue::new(
                serde_json::from_str(&db_planet.ship_queue).unwrap(),
            ),
            db_planet.last_update as usize,
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
        ogame_core::resources::Resources::new(
            db_resources.metal,
            db_resources.crystal,
            db_resources.deuterium,
        )
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

impl From<ships::Data> for ogame_core::ship_hangar::ShipHangar {
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

        ogame_core::ship_hangar::ShipHangar::new(ships)
    }
}
