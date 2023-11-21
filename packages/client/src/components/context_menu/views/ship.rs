use ogame_core::ship::Ship;

use crate::components::context_menu::{Action, ContextMenuRows, IntoContextMenuRows};

pub struct ShipContextMenu(pub Ship);

impl IntoContextMenuRows for ShipContextMenu {
    fn into_context_menu(&self) -> ContextMenuRows {
        let rows = vec![Action::OpenWindow("Storages".to_string())];

        ContextMenuRows { rows }
    }
}
