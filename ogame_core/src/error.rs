#[derive(Debug)]
pub enum Error {
    NotEnoughResources,
    NotEnoughShips,
    FlightNotArrived,
}

pub type Result<T> = std::result::Result<T, Error>;
