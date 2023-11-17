use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

use crate::error::*;

use serde::{Deserialize, Serialize};

use crate::ship::Ship;

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

impl From<MissionType> for String {
    fn from(m: MissionType) -> Self {
        match m {
            MissionType::Attack => "Attack".to_string(),
            MissionType::Transport => "Transport".to_string(),
            MissionType::Colonize => "Colonize".to_string(),
            MissionType::Espionage => "Espionage".to_string(),
            MissionType::Station => "Station".to_string(),
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
    pub user_id: String,
    pub from_id: String,
    pub to_id: String,
    pub ships: Vec<String>,
    pub mission: MissionType,
    pub speed_ratio: usize, // between 0 and 100,
    pub arrival_time: usize,
    pub return_time: Option<usize>,
}

impl Flight {
    pub fn new(
        id: String,
        user_id: String,
        from_id: String,
        to_id: String,
        ships: Vec<String>,
        mission: MissionType,
        speed_ratio: usize,
        arrival_time: usize,
        return_time: Option<usize>,
    ) -> Self {
        Flight {
            id,
            user_id,
            from_id,
            to_id,
            ships,
            mission,
            speed_ratio,
            arrival_time,
            return_time,
        }
    }
    pub fn create(
        id: String,
        user_id: String,
        from_id: String,
        to_id: String,
        ships: Vec<String>,
        mission: MissionType,
        speed_ratio: usize,
    ) -> Result<Self> {
        /* let duration =
        Self::calc_flight_duration(&from_planet.coordinates, to_coordinates, speed_ratio); */
        let duration = 0;
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)?
            .as_secs() as usize;

        let arrival_time = duration + now;
        let return_time = match mission {
            MissionType::Station => None,
            _ => Some(duration * 2 + now),
        };

        Ok(Flight {
            id,
            user_id,
            from_id,
            to_id,
            ships,
            arrival_time,
            return_time,
            mission,
            speed_ratio,
        })
    }

    /* fn calc_flight_duration(from: String, to: String, speed_ratio: usize) -> usize {
        let distance = from.distance(to);

        // FIXME: the 1 is a placeholder for the speed of the slowest ship
        ((10 + (3500 / speed_ratio) * 10 * distance) as f64)
            .sqrt()
            .floor() as usize
    } */
}
