use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    flight::{Flight, MissionType},
    game::Game,
    ship_type::ShipType,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    // Server -> Client
    Game(Game),
    InboundFleet(Flight),

    // Client -> Server
    /* UpgradeBuilding {
        planet_id: String,
        building_type: BuildingType,
    }, */
    /* BuyShip {
        planet_id: String,
        ship_type: ShipType,
        amount: usize,
    }, */
    SendShips {
        from_id: String,
        to_id: String,
        ships: Vec<String>,
        mission: MissionType,
        speed_ratio: usize,
        resources: BTreeMap<String, usize>,
    },
}
