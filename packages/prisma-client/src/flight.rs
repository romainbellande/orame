use std::sync::Arc;

use ogame_core::flight::Flight;

use crate::{db::*, db_model::DbModel, error::*};

impl From<flight::Data> for Flight {
    fn from(db_flight: flight::Data) -> Self {
        println!("db_flight: {:#?}", db_flight);
        Self {
            id: db_flight.id.clone(),
            user_id: db_flight.user_id.clone(),
            from_id: db_flight.from_id.clone(),
            to_id: db_flight.to_id.clone(),
            ships: db_flight
                .ships
                .unwrap_or(vec![])
                .into_iter()
                .map(|s| s.into())
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

        for ship in self.ships.iter_mut() {
            println!("ship: {:#?}", ship);
            ship.flight_id = Some(self.id.clone());
            ship.position_id = "".to_string();
            ship.save(conn).await?;
            println!("ship after: {:#?}", ship);
        }

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
            .find_unique(flight::id::equals(id.clone()))
            .with(flight::ships::fetch(vec![]))
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("Flight {} not found", id)))?;

        Ok(Self::from(db_flight))
    }

    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()> {
        conn.flight()
            .delete(flight::id::equals(self.id.clone()))
            .exec()
            .await
            .map_err(|e| Error::CannotDelete(e.to_string()))?;

        for ship in self.ships.iter() {
            let mut ship = ship.clone();
            ship.flight_id = None;
            ship.position_id = self.to_id.clone();
            ship.save(conn).await?;
        }

        Ok(())
    }
}
