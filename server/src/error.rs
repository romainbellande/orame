use std::net::AddrParseError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    #[error("Ogame core error: {0}")]
    Core(#[from] ogame_core::error::Error),

    #[error("Client disconnected")]
    ClientDisconnected,

    #[error("Db error: {0}")]
    DbNewClient(String),

    #[error("Axum error: {0}")]
    AxumServe(String),

    #[error("Db error: {0}")]
    DbError(String),

    #[error("Not found")]
    NotFound,

    #[error("Parse error")]
    ParseError(String),

    #[error("Send error")]
    SendError(String),
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

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Self::ParseError(e.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for Error {
    fn from(e: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        Self::SendError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
