use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{ship::Ship, ship_type::ShipType};
use web_sys::MouseEvent;

use crate::components::{
    context_menu::{views::ShipContextMenu, ContextMenuContext},
    tree_row::{IntoTreeItem, TreeItem},
};

pub struct ShipsTreeItem(pub BTreeMap<String, Ship>);

impl IntoTreeItem for ShipsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <div class="grid grid-cols-4 gap-4 ml-4">
                <span> Ship id </span>
                <span> Type </span>
                <span> Structure id </span>
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
                .map(|(_, ship)| ShipTreeItem(ship).into_tree_item())
                .collect(),
        }
    }
}

pub struct ShipTreeItem(pub Ship);

impl IntoTreeItem for ShipTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let context_menu = expect_context::<RwSignal<ContextMenuContext>>();

        let self_copy = self.0.clone();
        let context_click = move |ev: MouseEvent| {
            context_menu
                .update(|context_menu| context_menu.show(ShipContextMenu(self_copy.clone()), ev));
        };
        #[allow(unused_braces)]
        let view = view! {
            <div class="grid grid-cols-4 gap-4" on:auxclick=context_click>
                <span> {self.0.id.clone()} </span>
                <span> {self.0.r#type.to_string()} </span>
                <span> {self.0.position_id.clone()} </span>
                <span> {self.0.storage_id.clone()} </span>
            </div>

        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: vec![],
        }
    }
}
