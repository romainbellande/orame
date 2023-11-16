pub mod body;
mod claims;
pub mod credentials;
mod errors;
mod keys;
pub mod middleware;
mod service;

use std::sync::Arc;

use axum::{routing::post, Extension, Json, Router};
pub use claims::Claims;
use credentials::Credentials;
use prisma_client::PrismaClient;

use axum::http::{header::SET_COOKIE, HeaderMap};
use axum::response::Redirect;
use errors::WebError;

pub async fn login(
    Extension(conn): Extension<Arc<PrismaClient>>,
    Json(credentials): Json<Credentials>,
) -> Result<(HeaderMap, Redirect), WebError> {
    let body: body::AuthBody = service::login(&conn, credentials).await?;

    let cookie = format!("access_token={}; SameSite=Lax; Path=/", body.access_token);
    let mut headers = HeaderMap::new();

    headers.insert(SET_COOKIE, cookie.parse().expect("failed to parse cookie"));

    Ok((headers, Redirect::to("/")))
}

pub async fn register(
    Extension(conn): Extension<Arc<PrismaClient>>,
    Json(credentials): Json<Credentials>,
) -> Result<(HeaderMap, Redirect), WebError> {
    let body: body::AuthBody = service::register(&conn, credentials).await?;

    let cookie = format!("access_token={}; SameSite=Lax; Path=/", body.access_token);
    let mut headers = HeaderMap::new();

    headers.insert(SET_COOKIE, cookie.parse().expect("failed to parse cookie"));

    Ok((headers, Redirect::to("/")))
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
