use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode as HyperStatusCode;
use serde_json::{json, Value};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl Into<WebError> for AuthError {
    fn into(self) -> WebError {
        match self {
            Self::WrongCredentials => WebError {
                code: 1,
                status: HyperStatusCode::UNAUTHORIZED,
                message: "Wrong credentials".to_string(),
            },
            Self::MissingCredentials => WebError {
                code: 2,
                status: HyperStatusCode::BAD_REQUEST,
                message: "Missing credentials".to_string(),
            },
            Self::TokenCreation => WebError {
                code: 3,
                status: HyperStatusCode::INTERNAL_SERVER_ERROR,
                message: "Token creation error".to_string(),
            },
            Self::InvalidToken => WebError {
                code: 4,
                status: HyperStatusCode::BAD_REQUEST,
                message: "Invalid token".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebError {
    pub code: u16,
    pub status: HyperStatusCode,
    pub message: String,
}

impl WebError {
    pub fn into_json(&self) -> Json<Value> {
        Json(json!({
            "code": self.code,
            "message": self.message,
            "status": self.status.as_u16(),
        }))
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let file = std::file!();
        (self.status, self.into_json()).into_response()
    }
}

pub enum UserError {
    CouldNotSaveUser,
    NotFound { email: String },
}

impl Into<WebError> for UserError {
    fn into(self) -> WebError {
        match self {
            Self::CouldNotSaveUser => WebError {
                code: 500,
                status: HyperStatusCode::INTERNAL_SERVER_ERROR,
                message: "could not save user".to_string(),
            },
            Self::NotFound { email } => WebError {
                code: 404,
                status: HyperStatusCode::NOT_FOUND,
                message: format!("user with email {} not found", email),
            },
        }
    }
}
