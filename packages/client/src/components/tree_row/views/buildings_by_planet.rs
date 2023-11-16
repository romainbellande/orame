use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{building_type::BuildingType, planet::Planet};

use crate::components::tree_row::{IntoTreeItem, TreeItem};

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
        let view = view! {
            {self.0.id.clone()}
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
        let view = view! {
            {self.0.0.to_string()} {self.0.1}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0 .0.to_string(),
            children: vec![],
        }
    }
}
