use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    build_cost_trait::BuildCost,
    build_queue::BuildQueue,
    building_type::BuildingType,
    coordinates::Coordinates,
    error::*,
    fleet::Fleet,
    flight::{Flight, MissionType},
    resources::{ResourceType, Resources},
    ship_type::ShipType,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Planet {
    pub id: String,
    pub coordinates: Coordinates,
    pub resources: Resources,
    pub buildings: BTreeMap<BuildingType, usize>,
    pub ships: Fleet,
    pub build_queue: BuildQueue<BuildingType>,
    pub ship_queue: BuildQueue<ShipType>,
    pub last_update: usize,
    pub flights: Vec<Flight>,
}

impl Planet {
    pub fn new(
        id: String,
        coordinates: Coordinates,
        resources: Resources,
        buildings: BTreeMap<BuildingType, usize>,
        ships: Fleet,
        build_queue: BuildQueue<BuildingType>,
        ship_queue: BuildQueue<ShipType>,
        last_update: usize,
        flights: Vec<Flight>,
    ) -> Self {
        Planet {
            id,
            coordinates,
            resources,
            buildings,
            ships,
            build_queue,
            ship_queue,
            last_update,
            flights,
        }
    }

    pub fn upgrade_building(&mut self, building_type: BuildingType) -> Result<()> {
        let mut current_level = *self.buildings.get(&building_type).unwrap_or(&0);

        for building in self.build_queue.items.iter() {
            if building.r#type == building_type {
                current_level += 1;
            }
        }

        let cost = building_type.cost(current_level + 1);

        self.pay(cost)?;

        self.build_queue.push(building_type, current_level + 1);

        Ok(())
    }

    pub fn buy_ship(&mut self, ship_type: ShipType, amount: usize) -> Result<()> {
        let cost = ship_type.cost(0) * amount as f64;

        self.pay(cost)?;

        self.ship_queue.push(ship_type, amount);

        Ok(())
    }

    pub fn pay(&mut self, cost: Resources) -> Result<()> {
        if !self.resources.has_enough(&cost) {
            return Err(Error::NotEnoughResources);
        }

        self.resources -= cost;

        Ok(())
    }

    pub fn tick(&mut self, now: usize) -> Result<()> {
        let new_tick = self.build_queue.calc_tick_until_first_completion(now);

        self.update_resources(new_tick)?;
        self.process_build_queue(new_tick)?;
        self.process_ship_queue(new_tick)?;

        self.last_update = new_tick;

        if now > new_tick {
            self.tick(now)?;
        }

        Ok(())
    }

    fn process_build_queue(&mut self, now: usize) -> Result<()> {
        let buildings_update = self.build_queue.tick(now)?;

        for building in buildings_update {
            let current_level = self.buildings.get_mut(&building).unwrap(); // Should always exist
            *current_level += 1;
        }

        Ok(())
    }

    fn process_ship_queue(&mut self, now: usize) -> Result<()> {
        let ships_update = self.ship_queue.tick(now)?;

        for ship in ships_update {
            let current_amount = self.ships.ships.get_mut(&ship).unwrap();
            *current_amount += 1;
        }

        Ok(())
    }

    fn update_resources(&mut self, now: usize) -> Result<()> {
        for (building, level) in &self.buildings {
            let produced = building.produced(*level, now - self.last_update);

            self.resources += produced;
        }

        Ok(())
    }

    pub fn get_produced_resource(&self, resource: ResourceType, ticks: usize) -> f64 {
        let mut produced = 0.0;

        for (building, level) in &self.buildings {
            produced += building.produced(*level, ticks).get(resource.clone());
        }

        produced
    }

    pub fn building_level(&self, building_type: BuildingType) -> usize {
        *self.buildings.get(&building_type).unwrap_or(&0)
    }

    pub fn ships(&self) -> &Fleet {
        &self.ships
    }
    pub fn received_flight(&mut self, flight: Flight) -> Result<()> {
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if let Some(return_time) = flight.return_time {
            if return_time < now {
                self.ships.add_ships(&flight.ships.ships)?;

                return Ok(());
            }
        }

        if flight.arrival_time > now {
            return Err(Error::FlightNotArrived);
        }

        match flight.mission {
            MissionType::Transport => {
                self.resources += flight.resources;
            }
            MissionType::Station => {
                self.ships.add_ships(&flight.ships.ships)?;
                self.resources += flight.resources;
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}
