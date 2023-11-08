use serde::{Deserialize, Serialize};

use crate::{building_type::BuildingType, game::Game};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    // Server -> Client
    Game(Game),

    // Client -> Server
    UpgradeBuilding {
        planet_id: String,
        building_type: BuildingType,
    },
}
