use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{GameData, PlanetId, SystemId};
use web_sys::MouseEvent;

use crate::components::tree_row::{IntoTreeItem, TreeItem};

pub struct UniverseTreeItem(pub GameData);

impl IntoTreeItem for UniverseTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-8">
                <span> System id </span>
                <span> x </span>
                <span> y </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: "Universe".to_string(),
            children: self
                .0
                .systems
                .clone()
                .into_iter()
                .map(|(id, system)| {
                    SystemTreeItem(system.id.clone(), self.0.clone()).into_tree_item()
                })
                .collect(),
        }
    }
}

pub struct SystemTreeItem(pub SystemId, pub GameData);

impl IntoTreeItem for SystemTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let system = self.1.systems.get(&self.0).unwrap();

        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-4">
                <span> {system.id.clone()} </span>
                <span> {system.x.clone()} </span>
                <span> {system.y.clone()} </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: vec![
                PlanetsTreeItem(self.0.clone(), self.1.clone()).into_tree_item(),
                StationsTreeItem(self.0.clone(), self.1.clone()).into_tree_item(),
            ],
        }
    }
}

pub struct PlanetsTreeItem(pub SystemId, pub GameData);

impl IntoTreeItem for PlanetsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-8">
                <span> Planet id </span>
                <span> x </span>
                <span> y </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: self
                .1
                .planets
                .clone()
                .into_iter()
                .filter(|(_, planet)| planet.system_id == self.0)
                .map(|(id, planet)| {
                    PlanetTreeItem(planet.id.clone(), self.1.clone()).into_tree_item()
                })
                .collect(),
        }
    }
}

pub struct PlanetTreeItem(pub PlanetId, pub GameData);

impl IntoTreeItem for PlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let planet = self.1.planets.get(&self.0).unwrap();

        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-4 hover:bg-gray-400">
                <span> {planet.id.clone()} </span>
                <span> {planet.x.clone()} </span>
                <span> {planet.y.clone()} </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.to_string(),
            children: vec![],
        }
    }
}

pub struct StationsTreeItem(pub SystemId, pub GameData);

impl IntoTreeItem for StationsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-8">
                <span> Station id </span>
                <span> x </span>
                <span> y </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: self
                .1
                .planets
                .clone()
                .into_iter()
                .filter(|(_, planet)| planet.system_id == self.0)
                .map(|(id, planet)| {
                    StationTreeItem(planet.id.clone(), self.1.clone()).into_tree_item()
                })
                .collect(),
        }
    }
}

pub struct StationTreeItem(pub PlanetId, pub GameData);

impl IntoTreeItem for StationTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let planet = self.1.planets.get(&self.0).unwrap();

        let view = view! {
            <span class="grid grid-cols-3 gap-4 ml-4 hover:bg-gray-400">
                <span> {planet.id.clone()} </span>
                <span> {planet.x.clone()} </span>
                <span> {planet.y.clone()} </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.to_string(),
            children: vec![],
        }
    }
}
