use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::EntityId;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub ticks: usize,
    pub inputs: BTreeMap<EntityId, usize>,
    pub outputs: BTreeMap<EntityId, usize>,
}
