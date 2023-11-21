use std::{collections::BTreeMap, sync::Arc};

use ogame_core::{flight::Flight, ship::Ship};

use crate::{db::*, error::*, DbModel};

#[derive(Clone, Debug)]
pub struct ShipsInFlight {
    pub ships: Vec<Ship>,
    pub flight: Flight,
}

/* impl From<Vec<ships_in_flight::Data>> for ShipsInFlight {
    fn from(db_ships_in_flight: Vec<ships_in_flight::Data>) -> Self {
        Self {
            ships: db_ships_in_flight
                .iter()
                .map(|db_ship_in_flight| (*db_ship_in_flight.ship.clone().unwrap()).into())
                .collect(),
            flight: Flight::from(*db_ships_in_flight[0].clone().flight.unwrap()),
        }
    }
}

impl DbModel for ShipsInFlight {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        for ship in self.ships.iter() {
            let ship_in_flight = conn
                .ships_in_flight()
                .create(
                    flight::id::equals(self.flight.id.clone()),
                    ship::id::equals(ship.id.clone()),
                    vec![],
                )
                .exec()
                .await
                .map_err(|e| Error::CannotCreate(e.to_string()))?;
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
            .ships_in_flight()
            .find_many(vec![ships_in_flight::flight_id::equals(id.clone())])
            .with(ships_in_flight::ship::fetch())
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?;

        Ok(Self::from(db_flight))
    }

    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()> {
        conn.ships_in_flight()
            .delete_many(vec![ships_in_flight::flight_id::equals(
                self.flight.id.clone(),
            )])
            .exec()
            .await
            .map_err(|e| Error::CannotDelete(e.to_string()))?;

        Ok(())
    }
} */
