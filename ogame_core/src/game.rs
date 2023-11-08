use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use web_time::UNIX_EPOCH;

use crate::{
    building_type::BuildingType,
    error::*,
    flight::{Flight, MissionType},
    planet::Planet,
    protocol::Protocol,
    resources::Resources,
    ship_type::ShipType,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub player_id: String,
    pub planets: BTreeMap<String, Planet>,
    pub flights: Vec<Flight>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_id: "".to_string(),
            planets: BTreeMap::new(),
            flights: vec![],
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let now = web_time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        for (_, planet) in &mut self.planets {
            planet.tick(now)?;
        }

        Ok(())
    }

    pub fn process_message(&mut self, msg: Protocol) -> Result<()> {
        self.tick()?;

        match msg {
            Protocol::UpgradeBuilding {
                planet_id,
                building_type,
            } => {
                self.upgrade_building(planet_id, building_type)?;
            }
            Protocol::Game(game) => {
                self.player_id = game.player_id;
                self.planets = game.planets;
            }
        }

        Ok(())
    }

    fn upgrade_building(&mut self, planet_id: String, building_type: BuildingType) -> Result<()> {
        self.tick()?;

        self.planets
            .get_mut(&planet_id)
            .unwrap()
            .upgrade_building(building_type)?;

        Ok(())
    }

    fn process_flights(&mut self) -> Result<()> {
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
    }

    fn buy_ship(&mut self, planet_id: String, ship_type: ShipType, amount: usize) -> Result<()> {
        self.tick()?;

        self.planets
            .get_mut(&planet_id)
            .unwrap()
            .buy_ship(ship_type, amount)?;

        Ok(())
    }

    fn create_flight(
        &mut self,
        from_planet_id: String,
        to_planet_id: String,
        ships: BTreeMap<ShipType, usize>,
        mission: MissionType,
        speed: usize,
    ) -> Result<Flight> {
        self.tick()?;

        self.planets
            .get_mut(&from_planet_id)
            .unwrap()
            .ships
            .assert_ships_amount(&ships)?;

        let flight = Flight {
            player_id: self.player_id.clone(),
            from_planet_id,
            to_planet_id,
            ships,
            arrival_time: 0,      // TODO
            return_time: Some(0), // TODO
            resources: Resources::default(),
            mission,
            speed,
        };

        self.flights.push(flight.clone());

        Ok(flight)
    }

    fn add_planet(&mut self, planet: Planet) {
        self.planets.insert(planet.id.clone(), planet);
    }
}
