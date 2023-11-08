use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use web_time::UNIX_EPOCH;

use crate::{
    building_type::BuildingType,
    coordinates::Coordinates,
    error::*,
    flight::{Flight, MissionType},
    planet::Planet,
    protocol::Protocol,
    resources::Resources,
    ship_hangar::ShipHangar,
    ship_type::ShipType,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub player_id: String,
    pub planets: BTreeMap<String, Planet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_id: "".to_string(),
            planets: BTreeMap::new(),
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
            // Servec -> Client
            Protocol::Game(game) => {
                self.player_id = game.player_id;
                self.planets = game.planets;
            }
            Protocol::InboundFleet(flight) => {
                if let Some(ref mut planet) = self.planets.get_mut(&flight.from_planet_id) {
                    planet.flights.push(flight.clone());
                }
                if let Some(ref mut planet) = self.planets.get_mut(&flight.to_planet_id) {
                    planet.flights.push(flight);
                }
            }

            // Client -> Server
            Protocol::UpgradeBuilding {
                planet_id,
                building_type,
            } => {
                self.upgrade_building(planet_id, building_type)?;
            }
            Protocol::BuyShip {
                planet_id,
                ship_type,
                amount,
            } => {
                self.buy_ship(planet_id, ship_type, amount)?;
            }
            Protocol::SendShips {
                from_planet_id,
                ships,
                resources,
                ..
            } => {
                self.pay_for_flight(from_planet_id, ships, resources)?;
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

    fn buy_ship(&mut self, planet_id: String, ship_type: ShipType, amount: usize) -> Result<()> {
        self.planets
            .get_mut(&planet_id)
            .unwrap()
            .buy_ship(ship_type, amount)?;

        Ok(())
    }

    fn pay_for_flight(
        &mut self,
        planet_id: String,
        ships: BTreeMap<ShipType, usize>,
        resources: Resources,
    ) -> Result<()> {
        let origin_planet = self.planets.get_mut(&planet_id).unwrap();

        // we first assert the ship amount so that if we cannot pay the resources price, we dont
        // have to add the ships back to the planet hangar
        origin_planet.ships.assert_ships_amount(&ships)?;
        origin_planet.pay(resources.clone())?;
        origin_planet.ships.remove_ships(&ships)?;
        // TODO: add deuterium consumption

        Ok(())
    }

    pub fn create_flight(
        &self,
        id: String,
        from_planet_id: String,
        to_planet_id: String,
        to_planet_coordinates: &Coordinates,
        ships: ShipHangar,
        resources: Resources,
        mission: MissionType,
        speed: usize,
    ) -> Result<Flight> {
        let flight = Flight::create(
            id,
            self.player_id.clone(),
            self.planets.get(&from_planet_id).unwrap(),
            to_planet_id.clone(),
            to_planet_coordinates,
            ships,
            resources,
            mission,
            speed,
        );

        Ok(flight)
    }
}
