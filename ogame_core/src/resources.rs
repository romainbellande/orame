use std::ops::{AddAssign, Mul, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Resources {
    pub metal: f64,
    pub crystal: f64,
    pub deuterium: f64,
}

impl Resources {
    pub fn new(metal: f64, crystal: f64, deuterium: f64) -> Self {
        Resources {
            metal,
            crystal,
            deuterium,
        }
    }

    pub fn has_enough(&self, other: &Self) -> bool {
        self.metal >= other.metal
            && self.crystal >= other.crystal
            && self.deuterium >= other.deuterium
    }

    pub fn pay(&mut self, other: &Self) {
        self.metal -= other.metal;
        self.crystal -= other.crystal;
        self.deuterium -= other.deuterium;
    }

    pub fn gain(&mut self, other: &Self) {
        self.metal += other.metal;
        self.crystal += other.crystal;
        self.deuterium += other.deuterium;
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.metal -= rhs.metal;
        self.crystal -= rhs.crystal;
        self.deuterium -= rhs.deuterium;
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.metal += rhs.metal;
        self.crystal += rhs.crystal;
        self.deuterium += rhs.deuterium;
    }
}

impl Mul<f64> for Resources {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Resources {
            metal: self.metal * rhs,
            crystal: self.crystal * rhs,
            deuterium: self.deuterium * rhs,
        }
    }
}
