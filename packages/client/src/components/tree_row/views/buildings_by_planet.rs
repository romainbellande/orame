use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{building_type::BuildingType, planet::Planet};
use web_sys::MouseEvent;

use crate::components::{
    context_menu::{views::BuildingContextMenu, ContextMenuContext},
    tree_row::{IntoTreeItem, TreeItem},
};

pub struct BuildingsByPlanetTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for BuildingsByPlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            "Planets"
        }
        .into_view();

        TreeItem {
            view,
            id: "Planets".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(_id, planet)| PlanetWithBuildingsTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct PlanetWithBuildingsTreeItem(pub Planet);

impl IntoTreeItem for PlanetWithBuildingsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            { self.0.id.clone() }
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: self
                .0
                .buildings
                .clone()
                .into_iter()
                .map(|b| BuildingTreeItem(b).into_tree_item())
                .collect(),
        }
    }
}

pub struct BuildingTreeItem(pub (BuildingType, usize));

impl IntoTreeItem for BuildingTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let context_menu = expect_context::<RwSignal<ContextMenuContext>>();
        let building_type = self.0 .0.to_string();
        let level = self.0 .1;
        let self_copy = self.0.clone();
        let context_click = move |ev: MouseEvent| {
            context_menu.update(|context_menu| {
                context_menu.show(BuildingContextMenu(self_copy.clone()), ev)
            });
        };

        let view = view! {
            <span on:auxclick=context_click>
                <span> {building_type} </span>
                <span> {level} </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0 .0.to_string(),
            children: vec![],
        }
    }
}
