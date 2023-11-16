use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Storage {
    pub id: String,
    pub user_id: String,
    pub structure_id: String,
    pub items: BTreeMap<String, usize>,
}

impl Storage {
    pub fn new(user_id: String) -> Self {
        Self {
            id: "".to_string(),
            user_id,
            structure_id: "".to_string(),
            items: BTreeMap::new(),
        }
    }
}
