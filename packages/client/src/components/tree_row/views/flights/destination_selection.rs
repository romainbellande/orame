use leptos::*;

use ogame_core::{Entity, GameData, PlanetId, PositionedEntity, StationId, SystemId};
use web_sys::MouseEvent;

use crate::components::tree_row::{IntoTreeItem, TreeItem};

#[derive(Clone, PartialEq)]
pub struct Destination(pub Entity);

impl Destination {
    pub fn id(&self) -> PlanetId {
        self.0.id().clone()
    }
}

impl PositionedEntity for Destination {
    fn get_real_position(&self) -> (i32, i32) {
        self.0.get_real_position()
    }
}

#[derive(Clone, PartialEq)]
pub struct DestinationSelectionTreeItem(pub GameData, pub RwSignal<Option<Destination>>);

impl IntoTreeItem for DestinationSelectionTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <span class="grid grid-cols-2 gap-4 ml-8">
                <span> System name </span>
                <span> (x, y) </span>
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
                .map(|(_id, system)| {
                    SystemTreeItem(system.id.clone(), self.0.clone(), self.1).into_tree_item()
                })
                .collect(),
            collapsed: create_rw_signal(false),
        }
    }
}

pub struct SystemTreeItem(
    pub SystemId,
    pub GameData,
    pub RwSignal<Option<Destination>>,
);

impl IntoTreeItem for SystemTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let system = self.1.systems.get(&self.0).unwrap();

        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-2 gap-4">
                <span> {system.name.clone()} </span>
                <span> "(" {system.x.clone()}, {system.y.clone()} ")" </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: vec![
                PlanetsTreeItem(self.0.clone(), self.1.clone(), self.2).into_tree_item(),
                StationsTreeItem(self.0.clone(), self.1.clone(), self.2).into_tree_item(),
            ],
            collapsed: create_rw_signal(true),
        }
    }
}

pub struct PlanetsTreeItem(
    pub SystemId,
    pub GameData,
    pub RwSignal<Option<Destination>>,
);

impl IntoTreeItem for PlanetsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-2 gap-4">
                <span> Planet name </span>
                <span> (x, y) </span>
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
                .map(|(_id, planet)| {
                    PlanetTreeItem(planet.id.clone(), self.1.clone(), self.2).into_tree_item()
                })
                .collect(),
            collapsed: create_rw_signal(true),
        }
    }
}

pub struct PlanetTreeItem(
    pub PlanetId,
    pub GameData,
    pub RwSignal<Option<Destination>>,
);

impl IntoTreeItem for PlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let planet = self.1.planets.get(&self.0).unwrap().clone();

        let selected_destination = self.2.clone();

        let planet2 = planet.clone();
        let select_ship = move |ev: MouseEvent| {
            ev.stop_propagation();
            selected_destination.update(|selected_destination| {
                *selected_destination = Some(Destination(Entity::Planet(planet2.clone())))
            });
        };

        let planet2 = planet.clone();
        let selected_class = move || {
            if selected_destination()
                .map(|dest| dest.id().clone())
                .unwrap_or("".to_string())
                .eq(&planet2.id)
            {
                "grid grid-cols-2 gap-4 hover:bg-green-200 bg-green-400"
            } else {
                "grid grid-cols-2 gap-4 hover:bg-gray-400"
            }
        };

        let view = view! {
            <span class={selected_class} on:click=select_ship>
                <span> {planet.name.clone()} </span>
                <span> "(" {planet.x.clone()}, {planet.y.clone()} ")" </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.to_string(),
            children: vec![],
            collapsed: create_rw_signal(false),
        }
    }
}

pub struct StationsTreeItem(
    pub SystemId,
    pub GameData,
    pub RwSignal<Option<Destination>>,
);

impl IntoTreeItem for StationsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <span class="grid grid-cols-2 gap-4">
                <span> Station name </span>
                <span> (x, y) </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.clone(),
            children: self
                .1
                .stations
                .clone()
                .into_iter()
                .filter(|(_, station)| station.system_id == self.0)
                .map(|(_id, station)| {
                    StationTreeItem(station.id.clone(), self.1.clone(), self.2).into_tree_item()
                })
                .collect(),
            collapsed: create_rw_signal(true),
        }
    }
}

pub struct StationTreeItem(
    pub StationId,
    pub GameData,
    pub RwSignal<Option<Destination>>,
);

impl IntoTreeItem for StationTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let station = self.1.stations.get(&self.0).unwrap().clone();

        let selected_destination = self.2.clone();

        let station2 = station.clone();
        let select_dest = move |ev: MouseEvent| {
            ev.stop_propagation();
            selected_destination.update(|selected_destination| {
                *selected_destination = Some(Destination(Entity::Station(station2.clone())))
            });
        };

        let station2 = station.clone();
        let selected_class = move || {
            if selected_destination()
                .map(|dest| dest.id().clone())
                .unwrap_or("".to_string())
                .eq(&station2.id)
            {
                "grid grid-cols-2 gap-4 hover:bg-green-200 bg-green-400"
            } else {
                "grid grid-cols-2 gap-4 hover:bg-gray-400"
            }
        };

        let view = view! {
            <span class=selected_class on:click=select_dest>
                <span> {station.name.clone()} </span>
                <span> "(" {station.x.clone()}, {station.y.clone()} ")" </span>
            </span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.to_string(),
            children: vec![],
            collapsed: create_rw_signal(true),
        }
    }
}
