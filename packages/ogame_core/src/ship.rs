use std::collections::BTreeMap;

use crate::{flight::Flight, ship_type::ShipType, storage::Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ship {
    pub id: String,
    pub r#type: ShipType,
    pub user_id: String,
    pub position_id: String,
    pub storage: Storage,
    pub flight: Option<Flight>,
}

impl Ship {
    pub fn new(user_id: String, r#type: ShipType) -> Self {
        Self {
            id: "".to_string(),
            r#type,
            storage: Storage::new(user_id.clone()),
            user_id,
            position_id: "".to_string(),
            flight: None,
        }
    }
}
