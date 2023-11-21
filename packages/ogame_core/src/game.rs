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

    pub fn tick(&mut self) -> Result<()> {
        let now = web_time::SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;

        Ok(())
    }

    pub fn process_message(&mut self, msg: Protocol) -> Result<()> {
        self.tick()?;

        match msg {
            // Server -> Client
            Protocol::Game(game) => {
                *self = game;
            }

            // Client -> Server
            Protocol::SendShips {
                from_id,
                ships,
                resources,
                ..
            } => {}
        }

        Ok(())
    }

    /* fn process_flights(&mut self) -> Result<()> {
        self.tick()?;

        let now = web_time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let mut flights = vec![];

        for flight in &mut self.flights {
            if flight.arrival_time <= now {
                self.planets
                    .get_mut(&flight.to_planet_id)
                    .unwrap()
                    .ships
                    .add_ships(&flight.ships)?;
            } else {
                flights.push(flight.clone());
            }
        }

        self.flights = flights;

        Ok(())
    } */

    pub fn create_flight(
        &self,
        id: String,
        from_id: String,
        to_id: String,
        ships: Vec<String>,
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
