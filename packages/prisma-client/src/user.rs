use std::{collections::BTreeMap, sync::Arc};

use ogame_core::{flight::Flight, ship::Ship, ship_type::ShipType, storage::Storage};

use crate::{db::*, db_model::DbModel, error::*, GAME_DATA};

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
            .find_unique(user::email::equals(email.clone()))
            .with(user::ships::fetch(vec![]))
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
        ship.position_id = GAME_DATA.stations.iter().nth(0).unwrap().0.clone();
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
            .with(user::ships::fetch(vec![]))
            .with(user::flights::fetch(vec![]).with(flight::ships::fetch(vec![])))
            .with(user::storages::fetch(vec![]))
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("User {} not found", id)))?;

        Ok(Self::from(db_user))
    }

    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()> {
        conn.user()
            .delete(user::id::equals(self.id.clone()))
            .exec()
            .await
            .map_err(|e| Error::CannotDelete(e.to_string()))?;

        Ok(())
    }
}
