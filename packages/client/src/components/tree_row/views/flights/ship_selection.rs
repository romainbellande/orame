use std::collections::BTreeMap;

use leptos::*;

use ogame_core::ship::Ship;
use web_sys::MouseEvent;

use crate::components::{
    context_menu::{views::ShipContextMenu, ContextMenuContext},
    tree_row::{IntoTreeItem, TreeItem},
};

#[derive(Clone, PartialEq)]
pub struct ShipsSelectionTreeItem(pub BTreeMap<String, Ship>, pub RwSignal<Option<Ship>>);

impl IntoTreeItem for ShipsSelectionTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-4">
                <span> Ship id </span>
                <span> Type </span>
                <span> Position </span>
                <span> Storage id </span>
            </div>
        }
        .into_view();

        TreeItem {
            view,
            id: "Ships".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .filter(|(_, ship)| ship.flight_id == None)
                .map(|(_, ship)| ShipTreeItem(ship, self.1).into_tree_item())
                .collect(),
            collapsed: create_rw_signal(false),
        }
    }
}

pub struct ShipTreeItem(pub Ship, pub RwSignal<Option<Ship>>);

impl IntoTreeItem for ShipTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let context_menu = expect_context::<RwSignal<ContextMenuContext>>();

        let self_copy = self.0.clone();
        let context_click = move |ev: MouseEvent| {
            context_menu
                .update(|context_menu| context_menu.show(ShipContextMenu(self_copy.clone()), ev));
        };

        let selected_ship = self.1.clone();
        let ship = self.0.clone();

        let ship2 = ship.clone();
        let select_ship = move |ev: MouseEvent| {
            ev.stop_propagation();
            selected_ship.update(|selected_ship| *selected_ship = Some(ship2.clone()));
        };

        let selected_class = move || {
            if selected_ship()
                .map(|ship| ship.id)
                .unwrap_or("".to_string())
                .eq(&ship.id)
            {
                "grid grid-cols-4 gap-4 hover:bg-green-200 bg-green-400"
            } else {
                "grid grid-cols-4 gap-4 hover:bg-gray-400"
            }
        };

        let position_name = ogame_core::GAME_DATA
            .read()
            .unwrap()
            .get_position_name(&self.0.position_id);

        #[allow(unused_braces)]
        let view = view! {
            <div class=selected_class on:auxclick=context_click on:click=select_ship>
                <span> {self.0.id.clone()} </span>
                <span> {self.0.r#type.to_string()} </span>
                <span> {position_name} </span>
                <span> {self.0.storage_id.clone()} </span>
            </div>

        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: vec![],
            collapsed: create_rw_signal(true),
        }
    }
}
