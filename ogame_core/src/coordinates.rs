use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub galaxy: usize,
    pub system: usize,
    pub position: usize,
}

impl Coordinates {
    pub fn new(galaxy: usize, system: usize, position: usize) -> Self {
        Coordinates {
            galaxy,
            system,
            position,
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        let mut distance = 0;

        if self.galaxy != other.galaxy {
            distance += 20000 * (self.galaxy as i32 - other.galaxy as i32).abs();
        }

        if self.system != other.system {
            distance += 95 * (self.system as i32 - other.system as i32).abs();
        }

        if self.position != other.position {
            distance += (self.position as i32 - other.position as i32).abs();
        }

        distance as usize
    }
}
