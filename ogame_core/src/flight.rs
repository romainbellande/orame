use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

use crate::{
    coordinates::Coordinates, planet::Planet, resources::Resources, ship_hangar::ShipHangar,
    ship_type::ShipType,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MissionType {
    Attack,
    Transport,
    Colonize,
    Espionage,
    Station,
}

impl Display for MissionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionType::Attack => write!(f, "Attack"),
            MissionType::Transport => write!(f, "Transport"),
            MissionType::Colonize => write!(f, "Colonize"),
            MissionType::Espionage => write!(f, "Espionage"),
            MissionType::Station => write!(f, "Station"),
        }
    }
}

impl From<String> for MissionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Attack" => MissionType::Attack,
            "Transport" => MissionType::Transport,
            "Colonize" => MissionType::Colonize,
            "Espionage" => MissionType::Espionage,
            "Station" => MissionType::Station,
            _ => panic!("Invalid mission type"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Flight {
    pub id: String,
    pub player_id: String,
    pub from_planet_id: String,
    pub to_planet_id: String,
    pub ships: ShipHangar,
    pub resources: Resources,
    pub mission: MissionType,
    pub speed: usize, // between 0 and 100,
    pub arrival_time: usize,
    pub return_time: Option<usize>,
}

impl Flight {
    pub fn new(
        id: String,
        player_id: String,
        from_planet_id: String,
        to_planet_id: String,
        ships: ShipHangar,
        resources: Resources,
        mission: MissionType,
        speed: usize,
        arrival_time: usize,
        return_time: Option<usize>,
    ) -> Self {
        Flight {
            id,
            player_id,
            from_planet_id,
            to_planet_id,
            ships,
            resources,
            mission,
            speed,
            arrival_time,
            return_time,
        }
    }
    pub fn create(
        id: String,
        player_id: String,
        from_planet: &Planet,
        to_planet_id: String,
        to_coordinates: &Coordinates,
        ships: ShipHangar,
        resources: Resources,
        mission: MissionType,
        speed: usize,
    ) -> Self {
        let duration = Self::calc_flight_duration(&from_planet.coordinates, to_coordinates, speed);
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let arrival_time = duration + now;
        let return_time = match mission {
            MissionType::Station => None,
            _ => Some(duration * 2 + now),
        };

        Flight {
            id,
            player_id,
            from_planet_id: from_planet.id.clone(),
            to_planet_id,
            ships,
            arrival_time,
            return_time,
            mission,
            resources,
            speed,
        }
    }

    fn calc_flight_duration(from: &Coordinates, to: &Coordinates, speed: usize) -> usize {
        let distance = from.distance(to);

        // FIXME: the 1 is a placeholder for the speed of the slowest ship
        ((10 + (3500 / speed) * (10 * distance / 1)) as f64)
            .sqrt()
            .floor() as usize
    }
}
