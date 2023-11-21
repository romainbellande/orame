use serde::{Deserialize, Serialize};

use crate::{
    flight::{Flight, MissionType},
    game::Game,
    ship::Ship,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    // Server -> Client
    Game(Game),
    Flight(Flight),

    // Client -> Server
    SendShips {
        from_id: String,
        to_id: String,
        ships: Vec<Ship>,
        mission: MissionType,
        speed_ratio: usize,
    },
}
