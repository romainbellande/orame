use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Not enough resources")]
    NotEnoughResources,

    #[error("Not enough ships")]
    NotEnoughShips,

    #[error("Not enough ships")]
    FlightNotArrived,
}

pub type Result<T> = std::result::Result<T, Error>;
