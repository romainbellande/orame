use super::errors::WebError;
use super::{errors::AuthError, keys::KEYS};
use axum::{
    async_trait,
    extract::{FromRequest, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Request,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for Claims
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state)
                .await
                .map_err(|_| AuthError::InvalidToken.into())?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken.into())?;

        Ok(token_data.claims)
    }
}