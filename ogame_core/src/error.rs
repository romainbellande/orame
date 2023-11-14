use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    NotEnoughResources,
    NotEnoughShips,
    FlightNotArrived,
}

pub type Result<T> = std::result::Result<T, Error>;
