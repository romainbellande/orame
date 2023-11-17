use std::collections::BTreeMap;

use crate::{flight::Flight, ship_type::ShipType, storage::Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
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
