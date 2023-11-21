use serde::{Deserialize, Serialize};

use super::SystemId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct System {
    pub id: SystemId,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub links: Vec<SystemId>,
}
