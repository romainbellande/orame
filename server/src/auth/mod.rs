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
use prisma_client::PrismaClient;

use axum::http::{header::SET_COOKIE, HeaderMap};
use axum::response::{Redirect, Response};
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
) -> Result<(HeaderMap, Redirect), WebError> {
    let body: body::AuthBody = service::authorize(conn, credentials).await?;

    let cookie = format!("access_token={}; SameSite=Lax; Path=/", body.access_token);
    let mut headers = HeaderMap::new();

    headers.insert(SET_COOKIE, cookie.parse().expect("failed to parse cookie"));

    Ok((headers, Redirect::to("/")))
}

pub fn router() -> Router {
    Router::new().route("/login", post(authorize))
}
