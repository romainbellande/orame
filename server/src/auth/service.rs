use std::sync::Arc;

use prisma_client::{coordinates, planet, user, PrismaClient};

use super::errors::{UserError, WebError};
use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use hyper::StatusCode;
use jsonwebtoken::{encode, Header};

pub fn authorize(user_id: String) -> Result<AuthBody, WebError> {
    let claims = Claims {
        sub: user_id,
        company: "orame".to_owned(),
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation.into())?;

    Ok(AuthBody::new(token))
}

pub async fn login(
    conn: Arc<PrismaClient>,
    credentials: Credentials,
) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let my_user = conn
        .user()
        .find_first(vec![user::email::equals(credentials.email.clone())])
        .with(
            user::planets::fetch(vec![])
                .with(planet::coordinates::fetch())
                .with(planet::resources::fetch())
                .with(planet::buildings::fetch())
                .with(planet::ships::fetch()),
        )
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

    println!("USER: {:#?}", my_user);

    Ok(authorize(my_user.id)?)
}

pub async fn register(
    conn: Arc<PrismaClient>,
    credentials: Credentials,
) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    if let Ok(Some(_)) = conn
        .user()
        .find_first(vec![user::email::equals(credentials.email.clone())])
        .exec()
        .await
    {
        return Err(AuthError::UserAlreadyExists.into());
    }

    let my_user = conn
        .user()
        .create(
            "EMPTY_NAME".to_owned(),
            credentials.email.clone(),
            credentials.password.clone(),
            vec![],
        )
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    let new_planet = conn
        .planet()
        .create(
            user::id::equals(my_user.id.clone()),
            "[]".to_owned(),
            "[]".to_owned(),
            0,
            vec![],
        )
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    conn.coordinates()
        .create(planet::id::equals(new_planet.id.clone()), 0, 0, 0, vec![])
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    conn.resources()
        .create(
            planet::id::equals(new_planet.id.clone()),
            2500.0,
            1000.0,
            0.0,
            vec![],
        )
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    conn.buildings()
        .create(
            planet::id::equals(new_planet.id.clone()),
            0,
            0,
            0,
            0,
            vec![],
        )
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    conn.ships()
        .create(
            planet::id::equals(new_planet.id.clone()),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            vec![],
        )
        .exec()
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    Ok(authorize(my_user.id)?)
}
