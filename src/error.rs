pub enum Error {
    InvalidMessage(String),
}

pub type Result<T> = std::result::Result<T, Error>;
