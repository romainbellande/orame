use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    Core(ogame_core::error::Error),
    ClientDisconnected,
    DbNewClient(String),
    AxumServe(String),
    DbError(String),
    NotFound,
}

impl From<ogame_core::error::Error> for Error {
    fn from(e: ogame_core::error::Error) -> Self {
        Self::Core(e)
    }
}

impl From<prisma_client::NewClientError> for Error {
    fn from(e: prisma_client::NewClientError) -> Self {
        Self::DbNewClient(e.to_string())
    }
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Self::AxumServe(e.to_string())
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::AxumServe(e.to_string())
    }
}

impl From<prisma_client::QueryError> for Error {
    fn from(e: prisma_client::QueryError) -> Self {
        Self::DbError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
