use std::sync::Arc;

use ogame_core::{ship::Ship, storage::Storage};

use crate::{db::*, db_model::DbModel, error::*};

impl From<ship::Data> for Ship {
    fn from(db_ship: ship::Data) -> Self {
        Self {
            id: db_ship.id.clone(),
            flight_id: db_ship.flight_id.clone(),
            r#type: db_ship.r#type.into(),
            user_id: db_ship.user_id.clone(),
            position_id: db_ship.position_id.clone(),
            storage_id: db_ship.storage_id.clone(),
        }
    }
}

impl DbModel for Ship {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        let mut storage = Storage::new(self.user_id.clone());
        storage.create(conn).await?;

        let db_ship = conn
            .ship()
            .create(
                user::id::equals(self.user_id.clone()),
                self.position_id.clone(),
                self.r#type.clone().into(),
                storage::id::equals(storage.id.clone()),
                vec![],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;

        self.id = db_ship.id.clone();

        Ok(self)
    }

    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self> {
        let ship_saved = conn
            .ship()
            .update(
                ship::id::equals(self.id.clone()),
                vec![
                    ship::position_id::set(self.position_id.clone()),
                    ship::flight_id::set(self.flight_id.clone()),
                ],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotSave(e.to_string()))?;

        println!("ship_saved: {:#?}", ship_saved);

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

    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()> {
        conn.ship()
            .delete(ship::id::equals(self.id.clone()))
            .exec()
            .await
            .map_err(|e| Error::CannotDelete(e.to_string()))?;

        Ok(())
    }
}
