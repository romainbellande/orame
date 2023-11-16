use ogame_core::building_type::BuildingType;

use crate::components::context_menu::{Action, ContextMenuRows, IntoContextMenuRows};

pub struct BuildingContextMenu(pub (BuildingType, usize));

impl IntoContextMenuRows for BuildingContextMenu {
    fn into_context_menu(&self) -> ContextMenuRows {
        let building_type = self.0 .0.to_string();
        let level = self.0 .1;

        let rows = vec![
            Action::OpenWindow("Planets".to_string()),
            Action::UpgradeBuilding(building_type, level),
        ];

        ContextMenuRows { rows }
    }
}
