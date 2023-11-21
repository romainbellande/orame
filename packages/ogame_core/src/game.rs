use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use web_time::UNIX_EPOCH;

use crate::{
    error::*,
    flight::{Flight, MissionType},
    protocol::Protocol,
    ship::Ship,
    storage::Storage,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub user_id: String,
    pub ships: BTreeMap<String, Ship>,
    pub flights: BTreeMap<String, Flight>,
    pub storages: BTreeMap<String, Storage>,
    pub game_data: super::game_data::GameData,
}

impl Game {
    pub fn new() -> Self {
        Game {
            user_id: "".to_string(),
            ships: BTreeMap::new(),
            flights: BTreeMap::new(),
            storages: BTreeMap::new(),
            game_data: Default::default(),
        }
    }

    // Returns the data that have to be deleted from db
    pub fn tick(&mut self) -> Result<Vec<Flight>> {
        let now = web_time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;

        let flights_to_delete = self.process_flights(now)?;

        Ok(flights_to_delete)
    }

    pub fn process_message(&mut self, msg: Protocol) -> Result<()> {
        self.tick()?;

        match msg {
            // Server -> Client
            Protocol::Game(game) => {
                *self = game;
            }
            Protocol::Flight(flight) => {
                self.flights.insert(flight.id.clone(), flight.clone());
                for ship in flight.ships {
                    self.ships.insert(ship.id.clone(), ship.clone());
                }
            }

            // Client -> Server
            Protocol::SendShips { from_id, ships, .. } => {
                // TODO: do various checks
            }
        }

        Ok(())
    }

    fn process_flights(&mut self, now: usize) -> Result<Vec<Flight>> {
        let mut flights_to_delete = vec![];

        for (_, flight) in &mut self.flights {
            if flight.arrival_time <= now {
                flights_to_delete.push(flight.clone());
                self.ships
                    .values_mut()
                    .find(|ship| ship.flight_id == Some(flight.id.clone()))
                    .map(|ship| {
                        ship.flight_id = None;
                        ship.position_id = flight.to_id.clone();
                    });
            }
        }

        self.flights.retain(|_, flight| flight.arrival_time > now);

        Ok(flights_to_delete)
    }

    pub fn create_flight(
        &self,
        id: String,
        from_id: String,
        to_id: String,
        ships: Vec<Ship>,
        mission: MissionType,
        speed_ratio: usize,
    ) -> Result<Flight> {
        Flight::create(
            id,
            self.user_id.clone(),
            from_id,
            to_id,
            ships,
            mission,
            speed_ratio,
        )
    }
}
