use axum::{
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
    UserAlreadyExists,
}

impl From<AuthError> for WebError {
    fn from(e: AuthError) -> WebError {
        match e {
            AuthError::WrongCredentials => WebError {
                code: 1,
                status: HyperStatusCode::UNAUTHORIZED,
                message: "Wrong credentials".to_string(),
            },
            AuthError::MissingCredentials => WebError {
                code: 2,
                status: HyperStatusCode::BAD_REQUEST,
                message: "Missing credentials".to_string(),
            },
            AuthError::TokenCreation => WebError {
                code: 3,
                status: HyperStatusCode::INTERNAL_SERVER_ERROR,
                message: "Token creation error".to_string(),
            },
            AuthError::InvalidToken => WebError {
                code: 4,
                status: HyperStatusCode::BAD_REQUEST,
                message: "Invalid token".to_string(),
            },
            AuthError::UserAlreadyExists => WebError {
                code: 5,
                status: HyperStatusCode::BAD_REQUEST,
                message: "User already exists".to_string(),
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
    pub fn into_json(self) -> Json<Value> {
        Json(json!({
            "code": self.code,
            "message": self.message,
            "status": self.status.as_u16(),
        }))
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (self.status, self.into_json()).into_response()
    }
}

pub enum UserError {
    NotFound { email: String },
}

impl From<UserError> for WebError {
    fn from(e: UserError) -> WebError {
        match e {
            UserError::NotFound { email } => WebError {
                code: 404,
                status: HyperStatusCode::NOT_FOUND,
                message: format!("user with email {} not found", email),
            },
        }
    }
}

impl From<prisma_client::Error> for WebError {
    fn from(e: prisma_client::Error) -> Self {
        WebError {
            code: 500,
            status: HyperStatusCode::INTERNAL_SERVER_ERROR,
            message: format!("{:?}", e.to_string()),
        }
    }
}
