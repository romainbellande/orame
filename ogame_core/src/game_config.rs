use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameConfig {
    pub nb_galaxies: usize,
    pub nb_systems: usize,
    pub nb_planets: usize,
}
