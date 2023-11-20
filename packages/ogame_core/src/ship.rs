use std::collections::BTreeMap;

use crate::{flight::Flight, ship_type::ShipType, storage::Storage};
use serde::{Deserialize, Serialize};
use universe_gen::{GameData, PositionedItem, System};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Ship {
    pub id: String,
    pub r#type: ShipType,
    pub user_id: String,
    pub position_id: String,
    pub storage_id: String,
}

impl Ship {
    pub fn new(user_id: String, r#type: ShipType) -> Self {
        Self {
            id: "".to_string(),
            r#type,
            user_id,
            position_id: "".to_string(),
            storage_id: "".to_string(),
        }
    }
}

impl PositionedItem for Ship {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32) {
        let item = game_data.get_item_at_position(&self.position_id).unwrap();

        item.get_real_position(game_data)
    }
}
