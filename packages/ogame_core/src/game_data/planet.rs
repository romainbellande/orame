use serde::{Deserialize, Serialize};

use super::{GameData, PlanetId, PositionedEntity, SystemId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Planet {
    pub id: PlanetId,
    pub name: String,
    pub system_id: SystemId,
    pub x: i32,
    pub y: i32,
}

impl PositionedEntity for Planet {
    fn get_real_position(&self) -> (i32, i32) {
        let system = crate::GAME_DATA
            .read()
            .unwrap()
            .systems
            .get(&self.system_id)
            .unwrap()
            .clone();

        (system.x + self.x, system.y + self.y)
    }
}
