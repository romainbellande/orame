use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{build_cost_trait::BuildCost, resources::Resources};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ShipType {
    // Utility
    SmallCargo,
    LargeCargo,
    ColonyShip,
    Recycler,
    EspionageProbe,
    SolarSatellite,

    // Battle
    LightFighter,
    HeavyFighter,
    Cruiser,
    Battleship,
    Bomber,
    Destroyer,
    Battlecruiser,
    Deathstar,
}

impl Display for ShipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipType::SmallCargo => write!(f, "Small Cargo"),
            ShipType::LargeCargo => write!(f, "Large Cargo"),
            ShipType::ColonyShip => write!(f, "Colony Ship"),
            ShipType::Recycler => write!(f, "Recycler"),
            ShipType::EspionageProbe => write!(f, "Espionage Probe"),
            ShipType::SolarSatellite => write!(f, "Solar Satellite"),
            ShipType::LightFighter => write!(f, "Light Fighter"),
            ShipType::HeavyFighter => write!(f, "Heavy Fighter"),
            ShipType::Cruiser => write!(f, "Cruiser"),
            ShipType::Battleship => write!(f, "Battleship"),
            ShipType::Bomber => write!(f, "Bomber"),
            ShipType::Destroyer => write!(f, "Destroyer"),
            ShipType::Battlecruiser => write!(f, "Battlecruiser"),
            ShipType::Deathstar => write!(f, "Deathstar"),
        }
    }
}

impl BuildCost for ShipType {
    fn cost(&self, _level: usize) -> Resources {
        match self {
            ShipType::SmallCargo => Resources {
                metal: 2000.0,
                crystal: 2000.0,
                deuterium: 0.0,
            },
            ShipType::LargeCargo => Resources {
                metal: 6000.0,
                crystal: 6000.0,
                deuterium: 0.0,
            },
            ShipType::ColonyShip => Resources {
                metal: 10000.0,
                crystal: 20000.0,
                deuterium: 10000.0,
            },
            ShipType::Recycler => Resources {
                metal: 10000.0,
                crystal: 6000.0,
                deuterium: 2000.0,
            },
            ShipType::EspionageProbe => Resources {
                metal: 0.0,
                crystal: 1000.0,
                deuterium: 0.0,
            },
            ShipType::SolarSatellite => Resources {
                metal: 0.0,
                crystal: 2000.0,
                deuterium: 500.0,
            },
            ShipType::LightFighter => Resources {
                metal: 3000.0,
                crystal: 1000.0,
                deuterium: 0.0,
            },
            ShipType::HeavyFighter => Resources {
                metal: 6000.0,
                crystal: 4000.0,
                deuterium: 0.0,
            },
            ShipType::Cruiser => Resources {
                metal: 20000.0,
                crystal: 7000.0,
                deuterium: 2000.0,
            },
            ShipType::Battleship => Resources {
                metal: 45000.0,
                crystal: 15000.0,
                deuterium: 0.0,
            },
            ShipType::Bomber => Resources {
                metal: 50000.0,
                crystal: 25000.0,
                deuterium: 15000.0,
            },
            ShipType::Destroyer => Resources {
                metal: 60000.0,
                crystal: 50000.0,
                deuterium: 15000.0,
            },
            ShipType::Battlecruiser => Resources {
                metal: 30000.0,
                crystal: 40000.0,
                deuterium: 15000.0,
            },
            ShipType::Deathstar => Resources {
                metal: 5000000.0,
                crystal: 4000000.0,
                deuterium: 1000000.0,
            },
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
}
