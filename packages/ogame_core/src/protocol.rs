use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{flight::MissionType, game::Game};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    // Server -> Client
    Game(Game),

    // Client -> Server
    SendShips {
        from_id: String,
        to_id: String,
        ships: Vec<String>,
        mission: MissionType,
        speed_ratio: usize,
        resources: BTreeMap<String, usize>,
    },
}
