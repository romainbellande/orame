use std::fmt::{Display, Formatter};

use crate::{error::*, ship::Ship, PositionedEntity};

use serde::{Deserialize, Serialize};

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
    pub ships: Vec<Ship>,
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
        ships: Vec<Ship>,
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
        ships: Vec<Ship>,
        mission: MissionType,
        speed_ratio: usize,
    ) -> Result<Self> {
        let duration = Self::duration_of_flight(&from_id, &to_id, speed_ratio);

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

    pub fn duration_of_flight(id1: &str, id2: &str, speed_ratio: usize) -> usize {
        let distance = crate::GAME_DATA
            .read()
            .unwrap()
            .distance_between_ids(id1, id2);

        ((10 + (3500 / speed_ratio) * 10 * distance as usize) as f64)
            .sqrt()
            .floor() as usize
    }

    pub fn get_formated_duration(&self) -> String {
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let duration = self.arrival_time.clone() as u64 - now;
        Self::formated_duration(duration as usize)
    }

    pub fn formated_duration(duration: usize) -> String {
        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60) / 60;

        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds).to_string()
    }
}
