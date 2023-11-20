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

impl PositionedItem for Planet {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32) {
        let system = game_data.systems.get(&self.system_id).unwrap();

        (system.x + self.x, system.y + self.y)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

impl PositionedItem for Station {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32) {
        let system = game_data.systems.get(&self.system_id).unwrap();

        (system.x + self.x, system.y + self.y)
    }
}

pub trait PositionedItem {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32);
    fn distance_to<T: PositionedItem>(&self, item: &T, game_data: &GameData) -> i64 {
        let item1_real_coords = self.get_real_position(game_data);
        let item2_real_coords = item.get_real_position(game_data);

        ((item1_real_coords.0 as i64 - item2_real_coords.0 as i64).pow(2) as f64
            + (item1_real_coords.1 as i64 - item2_real_coords.1 as i64).pow(2) as f64)
            .sqrt() as i64
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Item {
    Planet(Planet),
    Station(Station),
}

impl PositionedItem for Item {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32) {
        match self {
            Item::Planet(planet) => planet.get_real_position(game_data),
            Item::Station(station) => station.get_real_position(game_data),
        }
    }
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

    pub fn get_item_at_position(&self, id: &str) -> Option<Item> {
        if let Some(planet) = self.planets.get(id) {
            return Some(Item::Planet(planet.clone()));
        } else if let Some(station) = self.stations.get(id) {
            return Some(Item::Station(station.clone()));
        }

        None
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
