use std::collections::BTreeMap;

use ogame_core::{flight::Flight, ship::Ship};

use crate::db::*;

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
