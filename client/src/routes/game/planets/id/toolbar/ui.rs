use std::collections::{BTreeMap, HashMap};

use leptos::*;
use ogame_core::building_type::BuildingType;

#[derive(Clone)]
pub struct ToolbarUI {
    pub buildings: BTreeMap<BuildingType, RwSignal<bool>>,
}

impl ToolbarUI {
    pub fn new() -> Self {
        let mut buildings = BTreeMap::new();
        buildings.insert(BuildingType::Metal, create_rw_signal(false));
        buildings.insert(BuildingType::Crystal, create_rw_signal(false));
        buildings.insert(BuildingType::Deuterium, create_rw_signal(false));

        Self { buildings }
    }

    pub fn is_building_visible(&self, building_type: BuildingType) -> ReadSignal<bool> {
        self.buildings.get(&building_type).unwrap().read_only()
    }

    pub fn toggle_building_window(&self, building_type: BuildingType) {
        let building = self.buildings.get(&building_type).unwrap();
        building.set(!building.get());
    }
}
