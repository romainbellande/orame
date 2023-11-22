use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::PositionedEntity;

use super::{Entity, Planet, PlanetId, Recipe, RecipeId, Station, StationId, System, SystemId};

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
        if let Some(system) = self.systems.get(id) {
            system.name.clone()
        } else if let Some(planet) = self.planets.get(id) {
            planet.name.clone()
        } else if let Some(station) = self.stations.get(id) {
            station.name.clone()
        } else {
            panic!("Invalid position id");
        }
    }

    pub fn get_item_at_position(&self, id: &str) -> Option<Entity> {
        if let Some(planet) = self.planets.get(id) {
            Some(Entity::Planet(planet.clone()))
        } else if let Some(station) = self.stations.get(id) {
            Some(Entity::Station(station.clone()))
        } else {
            None
        }
    }

    pub fn distance_between_ids(&self, id1: &str, id2: &str) -> i64 {
        let item1 = self.get_item_at_position(id1).unwrap();
        let item2 = self.get_item_at_position(id2).unwrap();

        item1.distance_to(&item2)
    }
}
