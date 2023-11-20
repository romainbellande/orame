use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod consts;

pub type SystemId = String;
pub type PlanetId = String;
pub type StationId = String;
pub type ItemId = String;
pub type RecipeId = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct System {
    pub id: SystemId,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub links: Vec<SystemId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Planet {
    pub id: PlanetId,
    pub name: String,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
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

    pub fn get_position_name(&self, id: &str) -> String {
        let mut name = String::new();

        if let Some(system) = self.systems.get(id) {
            name += &system.name;
        } else if let Some(planet) = self.planets.get(id) {
            name += &planet.name;
        } else if let Some(station) = self.stations.get(id) {
            name += &station.name;
        }

        name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub ticks: usize,
    pub inputs: BTreeMap<ItemId, usize>,
    pub outputs: BTreeMap<ItemId, usize>,
}
