use std::collections::BTreeMap;

use crate::error::*;
use crate::ship_type::ShipType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShipHangar {
    pub ships: BTreeMap<ShipType, usize>,
}

impl ShipHangar {
    pub fn new(ships: BTreeMap<ShipType, usize>) -> Self {
        Self { ships }
    }
    pub(crate) fn assert_ships_amount(&mut self, ships: &BTreeMap<ShipType, usize>) -> Result<()> {
        // TODO: implement deuterium consumption based on distance

        // check if enough ships
        for (ship_type, amount) in ships {
            let current_amount = self.ships.get(ship_type).unwrap_or(&0);

            if current_amount < amount {
                return Err(Error::NotEnoughShips);
            }
        }

        Ok(())
    }

    pub fn remove_ships(&mut self, ships: &BTreeMap<ShipType, usize>) -> Result<()> {
        self.assert_ships_amount(ships)?;
        for (ship_type, amount) in ships {
            let current_amount = self.ships.get_mut(ship_type).unwrap();
            *current_amount -= amount;
        }

        Ok(())
    }

    pub fn add_ships(&mut self, ships: &BTreeMap<ShipType, usize>) -> Result<()> {
        for (ship_type, amount) in ships {
            let current_amount = self.ships.get_mut(ship_type).unwrap();
            *current_amount += amount;
        }

        Ok(())
    }
}
