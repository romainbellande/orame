pub mod body;
mod claims;
pub mod credentials;
mod errors;
mod keys;
pub mod middleware;
mod service;

use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
pub use claims::Claims;
use credentials::Credentials;
// use sea_orm::{DatabaseConnection, EntityTrait};
use crate::db::PrismaClient;

use axum::response::Response;
use errors::WebError;
use hyper::StatusCode;
use serde::Serialize;

pub fn map_response<T: Serialize>(
    response: Result<T, WebError>,
    status: Option<StatusCode>,
) -> Response {
    match response {
        Ok(result) => {
            let final_status = match status {
                Some(status) => status,
                None => StatusCode::OK,
            };

            (final_status, Json(result)).into_response()
        }
        Err(web_error) => web_error.into_response(),
    }
}
pub async fn authorize(
    Extension(conn): Extension<Arc<PrismaClient>>,
    Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
    let result = service::authorize(conn, credentials).await;
    map_response(result, None)
}

pub fn router() -> Router {
    Router::new().route("/login", post(authorize))
}
