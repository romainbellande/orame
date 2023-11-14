use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    building_type::BuildingType,
    flight::{Flight, MissionType},
    game::Game,
    resources::Resources,
    ship_type::ShipType,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    // Server -> Client
    Game(Game),
    InboundFleet(Flight),

    // Client -> Server
    UpgradeBuilding {
        planet_id: String,
        building_type: BuildingType,
    },
    BuyShip {
        planet_id: String,
        ship_type: ShipType,
        amount: usize,
    },
    SendShips {
        from_planet_id: String,
        to_planet_id: String,
        ships: BTreeMap<ShipType, usize>,
        mission: MissionType,
        resources: Resources,
        speed_ratio: usize,
    },
}
