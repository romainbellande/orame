#[allow(warnings, unused)]
mod db;
// mod fetch_game;
// mod save_game;

use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    sync::Arc,
};

pub use db::PrismaClient;
use db::*;
// pub use fetch_game::*;
use ogame_core::{flight::Flight, ship::Ship, ship_type::ShipType, storage::Storage};
pub use prisma_client_rust::{queries::QueryError, NewClientError};
// pub use save_game::*;
//

#[derive(Debug, Clone)]
pub enum Error {
    CannotCreate(String),
    CannotSave(String),
    CannotFetch(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CannotCreate(msg) => write!(f, "Cannot create: {}", msg),
            Error::CannotSave(msg) => write!(f, "Cannot save: {}", msg),
            Error::CannotFetch(msg) => write!(f, "Cannot fetch: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub ships: BTreeMap<String, Ship>,
    pub flights: BTreeMap<String, Flight>,
    pub storages: BTreeMap<String, Storage>,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: "".to_string(),
            name,
            email,
            password,
            ships: BTreeMap::new(),
            flights: BTreeMap::new(),
            storages: BTreeMap::new(),
        }
    }

    pub async fn fetch_by_email(email: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_user = conn
            .user()
            .find_first(vec![user::email::equals(email.clone())])
            .with(user::ships::fetch(vec![]).with(ship::storage::fetch()))
            .with(user::flights::fetch(vec![]))
            .with(user::storages::fetch(vec![]))
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("User {} not found", email)))?;

        Ok(Self::from(db_user))
    }
}

impl From<user::Data> for User {
    fn from(db_user: user::Data) -> Self {
        println!("USER {:#?}", db_user);
        Self {
            id: db_user.id.clone(),
            name: db_user.name.clone(),
            email: db_user.email.clone(),
            password: db_user.password.clone(),
            ships: db_user
                .ships
                .unwrap()
                .into_iter()
                .map(|ship| (ship.id.clone(), ship.into()))
                .collect(),
            flights: db_user
                .flights
                .unwrap()
                .into_iter()
                .map(|flight| (flight.id.clone(), flight.into()))
                .collect(),
            storages: db_user
                .storages
                .unwrap()
                .into_iter()
                .map(|storage| (storage.id.clone(), storage.into()))
                .collect(),
        }
    }
}

impl From<User> for ogame_core::game::Game {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            ships: user.ships,
            flights: user.flights,
            storages: user.storages,
        }
    }
}

impl From<ogame_core::game::Game> for User {
    fn from(game: ogame_core::game::Game) -> Self {
        Self {
            id: game.user_id,
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            ships: game.ships,
            flights: game.flights,
            storages: game.storages,
        }
    }
}

pub trait DbModel {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self>
    where
        Self: Sized;
    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self>
    where
        Self: Sized;
    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self>
    where
        Self: Sized;
}

impl DbModel for User {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        let db_user = conn
            .user()
            .create(
                self.name.clone(),
                self.email.clone(),
                self.password.clone(),
                vec![],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;

        self.id = db_user.id.clone();

        let mut ship = Ship::new(self.id.clone(), ShipType::Basic);
        ship.create(conn).await?;

        Ok(self)
    }

    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self> {
        for flight in self.flights.values() {
            flight.save(conn).await?;
        }

        for ship in self.ships.values() {
            ship.save(conn).await?;
        }

        for storage in self.storages.values() {
            storage.save(conn).await?;
        }

        conn.user()
            .update(user::id::equals(self.id.clone()), vec![])
            .exec()
            .await
            .map_err(|e| Error::CannotSave(e.to_string()))?;

        Ok(self)
    }

    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_user = conn
            .user()
            .find_first(vec![user::id::equals(id.clone())])
            .with(user::ships::fetch(vec![]).with(ship::storage::fetch()))
            .with(user::flights::fetch(vec![]))
            .with(user::storages::fetch(vec![]))
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("User {} not found", id)))?;

        Ok(Self::from(db_user))
    }
}

impl From<ship::Data> for Ship {
    fn from(db_ship: ship::Data) -> Self {
        Self {
            id: db_ship.id.clone(),
            r#type: db_ship.r#type.into(),
            user_id: db_ship.user_id.clone(),
            position_id: db_ship.position_id.clone(),
            storage: (*db_ship.storage.unwrap()).into(),
            flight: db_ship.flight.map(|flight| {
                flight
                    .into_iter()
                    .map(|f| (*f.flight.unwrap()).into())
                    .collect::<Vec<Flight>>()
                    .first()
                    .unwrap()
                    .clone()
            }),
        }
    }
}

impl DbModel for Ship {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        println!("pre-storage_create");
        self.storage.create(conn).await?;

        println!("post-storage_create {:#?}", self.storage);

        println!("pre-ship_create {:#?}", self);
        let db_ship = conn
            .ship()
            .create(
                user::id::equals(self.user_id.clone()),
                self.position_id.clone(),
                self.r#type.clone().into(),
                storage::id::equals(self.storage.id.clone()),
                vec![],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;
        println!("post-ship_create");

        self.id = db_ship.id.clone();

        Ok(self)
    }

    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self> {
        conn.ship()
            .update(
                ship::id::equals(self.id.clone()),
                vec![ship::position_id::set(self.position_id.clone())],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotSave(e.to_string()))?;

        Ok(self)
    }

    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_ship = conn
            .ship()
            .find_first(vec![ship::id::equals(id.clone())])
            .with(ship::storage::fetch())
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("Ship {} not found", id)))?;

        println!("db_ship {:#?}", db_ship);

        Ok(Self::from(db_ship))
    }
}

impl From<flight::Data> for Flight {
    fn from(db_flight: flight::Data) -> Self {
        Self {
            id: db_flight.id.clone(),
            user_id: db_flight.user_id.clone(),
            from_id: db_flight.from_id.clone(),
            to_id: db_flight.to_id.clone(),
            ships: db_flight
                .ships
                .unwrap()
                .into_iter()
                .map(|s| (*s.ship.unwrap().unwrap()).into())
                .collect(),
            arrival_time: db_flight.arrival_time as usize,
            return_time: db_flight
                .return_time
                .map(|return_time| return_time as usize),
            mission: db_flight.mission.into(),
            speed_ratio: db_flight.speed_ratio as usize,
        }
    }
}

impl DbModel for Flight {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        let db_flight = conn
            .flight()
            .create(
                user::id::equals(self.user_id.clone()),
                self.from_id.clone(),
                self.to_id.clone(),
                self.arrival_time as i32,
                self.mission.clone().into(),
                self.speed_ratio as i32,
                vec![flight::return_time::set(
                    self.return_time.map(|return_time| return_time as i32),
                )],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;

        self.id = db_flight.id.clone();

        Ok(self)
    }

    async fn save(&self, _conn: &Arc<PrismaClient>) -> Result<&Self> {
        /* conn.flight()
        .update(flight::id::equals(self.id.clone()), vec![])
        .exec()
        .await
        .map_err(|e| Error::CannotSave(e.to_string()))?; */

        Ok(self)
    }

    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_flight = conn
            .flight()
            .find_first(vec![flight::id::equals(id.clone())])
            .with(flight::ships::fetch(vec![]))
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("Flight {} not found", id)))?;

        Ok(Self::from(db_flight))
    }
}

impl From<storage::Data> for Storage {
    fn from(db_storage: storage::Data) -> Self {
        Self {
            id: db_storage.id.clone(),
            user_id: db_storage.user_id.clone(),
            structure_id: db_storage.structure_id.clone(),
            items: serde_json::from_str(&db_storage.items).unwrap(),
        }
    }
}

impl DbModel for Storage {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        let db_storage = conn
            .storage()
            .create(
                user::id::equals(self.user_id.clone()),
                self.structure_id.clone(),
                serde_json::to_string(&self.items).unwrap(),
                vec![],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;

        self.id = db_storage.id.clone();

        Ok(self)
    }

    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self> {
        conn.storage()
            .update(
                storage::id::equals(self.id.clone()),
                vec![storage::items::set(
                    serde_json::to_string(&self.items).unwrap(),
                )],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotSave(e.to_string()))?;

        Ok(self)
    }

    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_storage = conn
            .storage()
            .find_first(vec![storage::id::equals(id.clone())])
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("Storage {} not found", id)))?;

        Ok(Self::from(db_storage))
    }
}

pub struct ShipsInFlight {
    pub ships: BTreeMap<String, Ship>,
    pub flight: Flight,
}

impl From<Vec<ships_in_flight::Data>> for ShipsInFlight {
    fn from(db_ships_in_flight: Vec<ships_in_flight::Data>) -> Self {
        Self {
            ships: db_ships_in_flight
                .iter()
                .map(|db_ship_in_flight| {
                    (
                        db_ship_in_flight.ship_id.clone(),
                        (*db_ship_in_flight.ship.clone().unwrap().unwrap()).into(),
                    )
                })
                .collect(),
            flight: Flight::from(*db_ships_in_flight[0].clone().flight.unwrap()),
        }
    }
}
