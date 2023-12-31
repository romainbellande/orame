use super::errors::WebError;
use super::{errors::AuthError, keys::KEYS};
use axum::extract::rejection::TypedHeaderRejectionReason;
use axum::extract::FromRequestParts;
// use axum::http::request::Parts;
use axum::{async_trait, extract::TypedHeader, headers::Cookie, RequestPartsExt};
use http::{header::COOKIE, request::Parts};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync + 'static,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies =
            parts
                .extract::<TypedHeader<Cookie>>()
                .await
                .map_err(|e| match *e.name() {
                    COOKIE => match e.reason() {
                        TypedHeaderRejectionReason::Missing => {
                            WebError::from(AuthError::MissingCredentials)
                        }
                        _ => AuthError::InvalidToken.into(),
                    },
                    _ => AuthError::InvalidToken.into(),
                })?;

        let access_token = cookies
            .get("access_token")
            .ok_or_else(|| WebError::from(AuthError::MissingCredentials))?;

        let token_data = decode::<Claims>(access_token, &KEYS.decoding, &Validation::default())
            .map_err(|_| WebError::from(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}
