use std::sync::Arc;

use prisma_client::{DbModel, PrismaClient, User};

use super::errors::{AuthError, WebError};
use super::{body::AuthBody, claims::Claims, credentials::Credentials, keys::KEYS};

use jsonwebtoken::{encode, Header};

pub fn authorize(user_id: String) -> Result<AuthBody, WebError> {
    let claims = Claims {
        sub: user_id,
        company: "orame".to_owned(),
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| WebError::from(AuthError::TokenCreation))?;

    Ok(AuthBody::new(token))
}

pub async fn login(
    conn: &Arc<PrismaClient>,
    credentials: Credentials,
) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let my_user = User::fetch(credentials.email.clone(), conn).await?;

    if credentials.password != my_user.password {
        return Err(AuthError::WrongCredentials.into());
    }

    println!("USER: {:#?}", my_user);

    authorize(my_user.id)
}

pub async fn register(
    conn: &Arc<PrismaClient>,
    credentials: Credentials,
) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    println!("Pre fetch");
    if let Ok(_) = User::fetch_by_email(credentials.email.clone(), conn).await {
        println!("User already exists");
        return Err(AuthError::UserAlreadyExists.into());
    }

    let mut new_user = User::new(
        "EMPTY_NAME".to_string(),
        credentials.email.clone(),
        credentials.password.clone(),
    );

    println!("Pre save");
    new_user.create(conn).await?;
    println!("Post save");

    authorize(new_user.id)
}
