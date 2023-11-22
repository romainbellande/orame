use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

pub mod error;
pub mod flight;
pub mod game;
pub mod game_data;
pub mod protocol;
pub mod ship;
pub mod ship_type;
pub mod storage;

pub use game_data::*;

lazy_static! {
    pub static ref GAME_DATA: Arc<RwLock<GameData>> = Arc::new(RwLock::new(GameData::default()));
}
