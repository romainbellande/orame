use serde::{Deserialize, Serialize};

use super::{Planet, PositionedEntity, Station};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Entity {
    Planet(Planet),
    Station(Station),
}

impl Entity {
    pub fn id(&self) -> String {
        match self {
            Entity::Planet(planet) => planet.id.clone(),
            Entity::Station(station) => station.id.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Entity::Planet(planet) => planet.name.clone(),
            Entity::Station(station) => station.name.clone(),
        }
    }
}

impl PositionedEntity for Entity {
    fn get_real_position(&self) -> (i32, i32) {
        match self {
            Entity::Planet(planet) => planet.get_real_position(),
            Entity::Station(station) => station.get_real_position(),
        }
    }
}
