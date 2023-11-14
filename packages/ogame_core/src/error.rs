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

    #[error("System time error: {0}")]
    SystemTime(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<web_time::SystemTimeError> for Error {
    fn from(e: web_time::SystemTimeError) -> Self {
        Self::SystemTime(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
