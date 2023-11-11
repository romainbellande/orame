use crate::coordinates::Coordinates;
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Clone)]
pub enum PlanetSubtype {
    Ocean,
    Terran,
    GasGiant,
    Ice,
    Volcanic,
    Desert,
    Hollow,
    Artifical,
    Dead,
}

const PLANET_SUBTYPES: [PlanetSubtype; 9] = [
    PlanetSubtype::Ocean,
    PlanetSubtype::Terran,
    PlanetSubtype::GasGiant,
    PlanetSubtype::Ice,
    PlanetSubtype::Volcanic,
    PlanetSubtype::Desert,
    PlanetSubtype::Hollow,
    PlanetSubtype::Artifical,
    PlanetSubtype::Dead,
];

impl PlanetSubtype {
    pub fn rand() -> Self {
        let items = PLANET_SUBTYPES.to_vec();

        let weights: Vec<u32> = items.iter().map(|p| p.get_density()).collect();

        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();

        items[dist.sample(&mut rng)].clone()
    }

    pub fn get_type(&self) -> PlanetType {
        match self {
            PlanetSubtype::Ocean => PlanetType::Habitable,
            PlanetSubtype::Terran => PlanetType::Habitable,
            PlanetSubtype::GasGiant => PlanetType::Hostile,
            PlanetSubtype::Ice => PlanetType::Hostile,
            PlanetSubtype::Volcanic => PlanetType::Hostile,
            PlanetSubtype::Desert => PlanetType::Hostile,
            PlanetSubtype::Hollow => PlanetType::Unusal,
            PlanetSubtype::Artifical => PlanetType::Unusal,
            PlanetSubtype::Dead => PlanetType::Hostile,
        }
    }

    pub fn get_density(&self) -> u32 {
        match self {
            PlanetSubtype::Ocean => 5000,
            PlanetSubtype::Terran => 5000,
            PlanetSubtype::GasGiant => 2000,
            PlanetSubtype::Ice => 3000,
            PlanetSubtype::Volcanic => 1000,
            PlanetSubtype::Desert => 2000,
            PlanetSubtype::Hollow => 100,
            PlanetSubtype::Artifical => 1,
            PlanetSubtype::Dead => 100,
        }
    }
}

pub enum PlanetType {
    Habitable,
    Hostile,
    Unusal,
}

pub struct PlanetConfig {
    coordinates: Coordinates,
    planet_subtype: PlanetSubtype,
}

impl PlanetConfig {
    pub fn is_hostile() -> bool {
        false
    }
}
