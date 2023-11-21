use crate::{
    game_data::{GameData, PositionedEntity},
    ship_type::ShipType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Ship {
    pub id: String,
    pub r#type: ShipType,
    pub user_id: String,
    pub position_id: String,
    pub storage_id: String,
    pub flight_id: Option<String>,
}

impl Ship {
    pub fn new(user_id: String, r#type: ShipType) -> Self {
        Self {
            id: "".to_string(),
            flight_id: None,
            r#type,
            user_id,
            position_id: "".to_string(),
            storage_id: "".to_string(),
        }
    }
}

impl PositionedEntity for Ship {
    fn get_real_position(&self, game_data: &GameData) -> (i32, i32) {
        let item = game_data.get_item_at_position(&self.position_id).unwrap();

        item.get_real_position(game_data)
    }
}
