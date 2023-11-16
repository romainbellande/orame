use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ShipType {
    Basic,
}

impl Display for ShipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic"),
        }
    }
}

impl From<ShipType> for String {
    fn from(s: ShipType) -> Self {
        match s {
            ShipType::Basic => "Basic".to_string(),
        }
    }
}

impl From<String> for ShipType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Basic" => ShipType::Basic,
            _ => panic!("Unknown ship type: {}", s),
        }
    }
}

/* impl BuildCost for ShipType {
    fn cost(&self, _level: usize) -> Resources {
        match self {
            ShipType::SmallCargo => Resources::from([
                (ResourceType::Metal, 2000.0),
                (ResourceType::Crystal, 2000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::LargeCargo => Resources::from([
                (ResourceType::Metal, 6000.0),
                (ResourceType::Crystal, 6000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::ColonyShip => Resources::from([
                (ResourceType::Metal, 10000.0),
                (ResourceType::Crystal, 20000.0),
                (ResourceType::Deuterium, 10000.0),
            ]),
            ShipType::Recycler => Resources::from([
                (ResourceType::Metal, 10000.0),
                (ResourceType::Crystal, 6000.0),
                (ResourceType::Deuterium, 2000.0),
            ]),
            ShipType::EspionageProbe => Resources::from([
                (ResourceType::Metal, 0.0),
                (ResourceType::Crystal, 1000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::SolarSatellite => Resources::from([
                (ResourceType::Metal, 0.0),
                (ResourceType::Crystal, 2000.0),
                (ResourceType::Deuterium, 500.0),
            ]),
            ShipType::LightFighter => Resources::from([
                (ResourceType::Metal, 3000.0),
                (ResourceType::Crystal, 1000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::HeavyFighter => Resources::from([
                (ResourceType::Metal, 6000.0),
                (ResourceType::Crystal, 4000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::Cruiser => Resources::from([
                (ResourceType::Metal, 20000.0),
                (ResourceType::Crystal, 7000.0),
                (ResourceType::Deuterium, 2000.0),
            ]),
            ShipType::Battleship => Resources::from([
                (ResourceType::Metal, 45000.0),
                (ResourceType::Crystal, 15000.0),
                (ResourceType::Deuterium, 0.0),
            ]),
            ShipType::Bomber => Resources::from([
                (ResourceType::Metal, 50000.0),
                (ResourceType::Crystal, 25000.0),
                (ResourceType::Deuterium, 15000.0),
            ]),
            ShipType::Destroyer => Resources::from([
                (ResourceType::Metal, 60000.0),
                (ResourceType::Crystal, 50000.0),
                (ResourceType::Deuterium, 15000.0),
            ]),
            ShipType::Battlecruiser => Resources::from([
                (ResourceType::Metal, 30000.0),
                (ResourceType::Crystal, 40000.0),
                (ResourceType::Deuterium, 15000.0),
            ]),
            ShipType::Deathstar => Resources::from([
                (ResourceType::Metal, 5000000.0),
                (ResourceType::Crystal, 4000000.0),
                (ResourceType::Deuterium, 1000000.0),
            ]),
        }
    }
}

impl From<String> for ShipType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Small Cargo" => ShipType::SmallCargo,
            "Large Cargo" => ShipType::LargeCargo,
            "Colony Ship" => ShipType::ColonyShip,
            "Recycler" => ShipType::Recycler,
            "Espionage Probe" => ShipType::EspionageProbe,
            "Solar Satellite" => ShipType::SolarSatellite,
            "Light Fighter" => ShipType::LightFighter,
            "Heavy Fighter" => ShipType::HeavyFighter,
            "Cruiser" => ShipType::Cruiser,
            "Battleship" => ShipType::Battleship,
            "Bomber" => ShipType::Bomber,
            "Destroyer" => ShipType::Destroyer,
            "Battlecruiser" => ShipType::Battlecruiser,
            "Deathstar" => ShipType::Deathstar,
            _ => panic!("Unknown ship type: {}", s),
        }
    }
} */
