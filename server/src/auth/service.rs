use std::sync::Arc;

use ogame_core::resources::Resources;
use prisma_client::{
    create_buildings, create_coordinates, create_planet, create_resources, create_ships,
    create_user, planet, user, PrismaClient,
};

use super::errors::{UserError, WebError};
use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};

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

    if credentials.password != my_user.password {
        return Err(AuthError::WrongCredentials.into());
    }

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

    let new_user = create_user(
        "EMPTY_NAME".to_owned(),
        credentials.email.clone(),
        credentials.password.clone(),
        &conn,
    )
    .await;

    let new_coordinates = create_coordinates(0, 0, 0, &conn).await;

    let new_resources = create_resources(
        &Resources {
            id: "".to_owned(),
            metal: 2500.0,
            crystal: 1000.0,
            deuterium: 0.0,
        },
        &conn,
    )
    .await;

    let new_ships = create_ships(
        &vec![
            (ogame_core::ship_type::ShipType::SmallCargo, 0 as usize),
            (ogame_core::ship_type::ShipType::LargeCargo, 0),
            (ogame_core::ship_type::ShipType::ColonyShip, 0),
            (ogame_core::ship_type::ShipType::Recycler, 0),
            (ogame_core::ship_type::ShipType::EspionageProbe, 0),
            (ogame_core::ship_type::ShipType::SolarSatellite, 0),
            (ogame_core::ship_type::ShipType::LightFighter, 0),
            (ogame_core::ship_type::ShipType::HeavyFighter, 0),
            (ogame_core::ship_type::ShipType::Cruiser, 0),
            (ogame_core::ship_type::ShipType::Battleship, 0),
            (ogame_core::ship_type::ShipType::Bomber, 0),
            (ogame_core::ship_type::ShipType::Destroyer, 0),
            (ogame_core::ship_type::ShipType::Battlecruiser, 0),
            (ogame_core::ship_type::ShipType::Deathstar, 0),
        ]
        .into_iter()
        .collect(),
        &conn,
    )
    .await;
    let new_planet = create_planet(
        new_user.id.clone(),
        new_coordinates.id,
        new_resources.id,
        // new_buildings.id,
        new_ships.id,
        &conn,
    )
    .await;

    create_buildings(new_planet.id.clone(), &conn).await;

    Ok(authorize(new_user.id)?)
}
