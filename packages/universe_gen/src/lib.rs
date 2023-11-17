use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod consts;

pub type SystemId = String;
pub type PlanetId = String;
pub type StationId = String;
pub type ItemId = String;
pub type RecipeId = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    pub id: SystemId,
    pub x: i32,
    pub y: i32,
    pub links: Vec<SystemId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planet {
    pub id: PlanetId,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub id: StationId,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameData {
    pub systems: BTreeMap<SystemId, System>,
    pub planets: BTreeMap<PlanetId, Planet>,
    pub stations: BTreeMap<StationId, Station>,
    pub recipes: BTreeMap<RecipeId, Recipe>,
}

impl GameData {
    pub fn get_items_list(&self) -> Vec<String> {
        self.recipes.keys().cloned().collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub ticks: usize,
    pub inputs: BTreeMap<ItemId, usize>,
    pub outputs: BTreeMap<ItemId, usize>,
}
