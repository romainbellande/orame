use std::sync::Arc;

use crate::db::{user, PrismaClient};

use super::errors::WebError;
use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};
// use crate::modules::user::errors::UserError;
// use entity::user::{self, CreateUser, UserResponse};
use hyper::StatusCode;
use jsonwebtoken::{encode, Header};
// use sea_orm::DatabaseConnection;

pub async fn authorize(
    conn: Arc<PrismaClient>,
    credentials: Credentials,
) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let my_user = conn
        .user()
        .find_first(vec![user::email::equals(credentials.email)])
        .exec()
        .await;

    let my_user = my_user.map_err(|err| WebError {
        code: 1,
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: err.to_string(),
    })?;

    let my_user = my_user.ok_or_else(|| {
        UserError::NotFound {
            email: credentials.email.clone(),
        }
        .into()
    })?;

    // Here you can check the user credentials from a database
    if !my_user.verify_password(credentials.password) {
        return Err(AuthError::WrongCredentials.into());
    }

    let claims = Claims {
        sub: my_user.id.to_string().to_owned(),
        company: "ACME".to_owned(),
        // TODO: add roles here
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation.into())?;

    Ok(AuthBody::new(token))
}
