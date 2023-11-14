use std::collections::BTreeMap;

use leptos::*;
use ogame_core::{building_type::BuildingType, ship_type::ShipType};
use web_sys::wasm_bindgen::UnwrapThrowExt;

use crate::data::ship::get_all_ships;

#[derive(Clone)]
pub struct PlanetUI {
    pub buildings: BTreeMap<BuildingType, RwSignal<bool>>,
    pub ships: BTreeMap<ShipType, RwSignal<bool>>,
    pub shipyard: RwSignal<bool>,
}

impl PlanetUI {
    pub fn new() -> Self {
        let mut buildings = BTreeMap::new();
        buildings.insert(BuildingType::Metal, create_rw_signal(false));
        buildings.insert(BuildingType::Crystal, create_rw_signal(false));
        buildings.insert(BuildingType::Deuterium, create_rw_signal(false));
        buildings.insert(BuildingType::Shipyard, create_rw_signal(false));

        let mut ships = BTreeMap::new();
        let _ = get_all_ships()
            .into_iter()
            .map(|ship| ships.insert(ship, create_rw_signal(false)))
            .collect::<Vec<_>>();

        let shipyard = create_rw_signal(false);

        Self {
            buildings,
            shipyard,
            ships,
        }
    }

    pub fn is_building_visible(&self, building_type: BuildingType) -> ReadSignal<bool> {
        self.buildings
            .get(&building_type)
            .expect_throw("building doesn't exist")
            .read_only()
    }

    pub fn toggle_building_window(&self, building_type: BuildingType) {
        let building = self.buildings.get(&building_type).unwrap();
        building.set(!building.get());
    }

    pub fn set_building_visibility(&self, building_type: BuildingType, visible: bool) {
        let building = self.buildings.get(&building_type).unwrap();
        building.set(visible);
    }

    pub fn is_ship_visible(&self, ship_type: ShipType) -> ReadSignal<bool> {
        self.ships
            .get(&ship_type)
            .expect_throw(format!("ship {ship_type} doesn't exist").as_str())
            .read_only()
    }

    pub fn toggle_ship_window(&self, ship_type: ShipType) {
        let ship = self.ships.get(&ship_type).unwrap();
        ship.set(!ship.get());
    }

    pub fn set_ship_visibility(&self, ship_type: ShipType, visible: bool) {
        let ship = self.ships.get(&ship_type).unwrap();
        ship.set(visible);
    }

    pub fn toggle_shipyard_window(&self) {
        self.shipyard.set(!self.shipyard.get());
    }
}
