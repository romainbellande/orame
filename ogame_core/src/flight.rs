use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    coordinates::Coordinates, error::*, fleet::Fleet, planet::Planet, resources::Resources,
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
    pub ships: Fleet,
    pub resources: Resources,
    pub mission: MissionType,
    pub speed_ratio: usize, // between 0 and 100,
    pub arrival_time: usize,
    pub return_time: Option<usize>,
}

impl Flight {
    pub fn new(
        id: String,
        player_id: String,
        from_planet_id: String,
        to_planet_id: String,
        ships: Fleet,
        resources: Resources,
        mission: MissionType,
        speed_ratio: usize,
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
            speed_ratio,
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
        ships: Fleet,
        resources: Resources,
        mission: MissionType,
        speed_ratio: usize,
    ) -> Result<Self> {
        let duration =
            Self::calc_flight_duration(&from_planet.coordinates, to_coordinates, speed_ratio);
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)?
            // .ok_or(Error::SystemTime("SystemTime::now() failed".to_string()))?
            .as_secs() as usize;

        let arrival_time = duration + now;
        let return_time = match mission {
            MissionType::Station => None,
            _ => Some(duration * 2 + now),
        };

        Ok(Flight {
            id,
            player_id,
            from_planet_id: from_planet.id.clone(),
            to_planet_id,
            ships,
            arrival_time,
            return_time,
            mission,
            resources,
            speed_ratio,
        })
    }

    fn calc_flight_duration(from: &Coordinates, to: &Coordinates, speed_ratio: usize) -> usize {
        let distance = from.distance(to);

        // FIXME: the 1 is a placeholder for the speed of the slowest ship
        ((10 + (3500 / speed_ratio) * 10 * distance) as f64)
            .sqrt()
            .floor() as usize
    }
}
