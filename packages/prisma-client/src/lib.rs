pub use prisma_client_rust::{queries::QueryError, NewClientError};

#[allow(warnings, unused)]
mod db;
mod db_model;
mod error;
mod flight;
mod ship;
mod storage;
mod user;

pub use db::new_client_with_url;
pub use db::PrismaClient;

pub use db_model::*;
pub use error::*;
pub use flight::*;
pub use ship::*;
pub use storage::*;
pub use user::*;

lazy_static::lazy_static! {
    pub static ref GAME_DATA: ogame_core::GameData = {
        let data = std::fs::read("../../data/game_data.cbor").unwrap();
        serde_cbor::from_slice(&data[..]).unwrap()
    };
}
