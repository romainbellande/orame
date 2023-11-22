use serde::{Deserialize, Serialize};

use super::{GameData, Planet, PositionedEntity, Station};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Entity {
    Planet(Planet),
    Station(Station),
}

impl PositionedEntity for Entity {
    fn get_real_position(&self) -> (i32, i32) {
        match self {
            Entity::Planet(planet) => planet.get_real_position(),
            Entity::Station(station) => station.get_real_position(),
        }
    }
}
