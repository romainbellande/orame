use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{resources::Resources, ship_type::ShipType};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MissionType {
    Attack,
    Transport,
    Colonize,
    Espionage,
    Station,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Flight {
    pub player_id: String,
    pub from_planet_id: String,
    pub to_planet_id: String,
    pub ships: BTreeMap<ShipType, usize>,
    pub arrival_time: usize,
    pub return_time: Option<usize>,
    pub mission: MissionType,
    pub resources: Resources,
    pub speed: usize, // between 0 and 100,
}
