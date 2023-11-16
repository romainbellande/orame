use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{planet::Planet, resources::ResourceType};

use crate::components::tree_row::{IntoTreeItem, TreeItem};

pub struct PlanetsTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for PlanetsTreeItem {
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
                .map(|(_, planet)| PlanetTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct PlanetTreeItem(pub Planet);

impl IntoTreeItem for PlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <span>{self.0.id.clone()}</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Metal).floor()} metal</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Crystal).floor()} crystal</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Deuterium).floor()} deuterium</span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: vec![],
        }
    }
}
