use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod consts;

pub type SystemId = i32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    pub x: i32,
    pub y: i32,
    pub links: Vec<SystemId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planet {
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameData {
    pub systems: Vec<System>,
    pub planets: BTreeMap<SystemId, Vec<(i32, i32)>>,
}
