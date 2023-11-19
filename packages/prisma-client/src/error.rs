use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    CannotCreate(String),
    CannotSave(String),
    CannotFetch(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CannotCreate(msg) => write!(f, "Cannot create: {}", msg),
            Error::CannotSave(msg) => write!(f, "Cannot save: {}", msg),
            Error::CannotFetch(msg) => write!(f, "Cannot fetch: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
