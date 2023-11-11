use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Hash, Ord, PartialOrd)]
pub enum ResourceType {
    Metal,
    Crystal,
    Deuterium,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Resources {
    pub id: String,
    pub items: BTreeMap<ResourceType, f64>,
}

impl<const N: usize> From<[(ResourceType, f64); N]> for Resources {
    fn from(items: [(ResourceType, f64); N]) -> Self {
        Self {
            id: "".to_string(),
            items: items.into(),
        }
    }
}

impl Resources {
    pub fn new(id: String, items: BTreeMap<ResourceType, f64>) -> Self {
        Resources { id, items }
    }

    pub fn has_enough(&self, other: &Self) -> bool {
        other.items.iter().all(|(resource_type, amount)| {
            self.items
                .get(resource_type)
                .map_or(false, |self_amount| self_amount >= amount)
        })
    }

    pub fn pay(&mut self, other: &Self) {
        *self -= other.clone();
    }

    pub fn gain(&mut self, other: &Self) {
        *self += other.clone();
    }

    pub fn contains(&self, resource_type: ResourceType) -> bool {
        self.items.contains_key(&resource_type)
    }

    pub fn get(&self, resource_type: ResourceType) -> f64 {
        *self.items.get(&resource_type).map_or(&0.0, |amount| amount)
    }

    pub fn build_time(&self) -> usize {
        let build_time = (self.get(ResourceType::Metal) + self.get(ResourceType::Crystal))
            / (2500.0 * crate::UNIVERSE_SPEED as f64)
            * 3600.0;

        build_time.ceil() as usize
    }

    pub fn set_id(&mut self, id: String) -> Self {
        self.id = id;
        self.clone()
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let items = self
            .items
            .iter()
            .map(|(resource_type, resource)| {
                let rhs_resource = rhs.items.get(resource_type).unwrap_or(&0.0);

                (resource_type.clone(), *resource - *rhs_resource)
            })
            .collect();

        Resources { id: self.id, items }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        for (resource_type, resource) in rhs.items.iter() {
            if let Some(self_resource) = self.items.get_mut(resource_type) {
                *self_resource -= resource;
            }
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let items = self
            .items
            .iter()
            .map(|(resource_type, resource)| {
                let rhs_resource = rhs.items.get(resource_type).unwrap_or(&0.0);

                (resource_type.clone(), *resource + *rhs_resource)
            })
            .collect();

        Resources { id: self.id, items }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        for (resource_type, resource) in rhs.items.iter() {
            if let Some(self_resource) = self.items.get_mut(resource_type) {
                *self_resource += resource;
            }
        }
    }
}

impl Mul<f64> for Resources {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let items = self
            .items
            .iter()
            .map(|(resource_type, resource)| (resource_type.clone(), *resource * rhs))
            .collect();

        Resources { id: self.id, items }
    }
}
