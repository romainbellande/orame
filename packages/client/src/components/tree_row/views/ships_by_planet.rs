use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{planet::Planet, ship_type::ShipType};

use crate::components::tree_row::{IntoTreeItem, TreeItem};

pub struct ShipsByPlanetTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for ShipsByPlanetTreeItem {
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
                .map(|(_, planet)| PlanetWithShipsTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct PlanetWithShipsTreeItem(pub Planet);

impl IntoTreeItem for PlanetWithShipsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            {self.0.id.clone()}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: self
                .0
                .ships
                .clone()
                .ships
                .into_iter()
                .map(|b| ShipTreeItem(b).into_tree_item())
                .collect(),
        }
    }
}

pub struct ShipTreeItem(pub (ShipType, usize));

impl IntoTreeItem for ShipTreeItem {
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
