/* pub mod build_cost_trait;
pub mod build_queue;
pub mod build_time_trait;
pub mod building_type;
pub mod coordinates; */
pub mod error;
// pub mod fleet;
pub mod flight;
pub mod game;
pub mod storage;
// pub mod planet;
pub mod protocol;
// pub mod resources;
pub mod ship_type;

pub mod ship;

pub use universe_gen::GameData;

const UNIVERSE_SPEED: usize = 8;
