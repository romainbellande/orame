use std::collections::BTreeMap;

use leptos::*;

use ogame_core::{flight::Flight, ship::Ship, ship_type::ShipType};
use web_sys::MouseEvent;

use crate::components::{
    context_menu::{views::ShipContextMenu, ContextMenuContext},
    tree_row::{IntoTreeItem, TreeItem},
};

pub struct FlightsTreeItem(pub BTreeMap<String, Flight>);

impl IntoTreeItem for FlightsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <div class="grid grid-cols-6 gap-4 ml-4">
                <span> From </span>
                <span> To </span>
                <span> Ships </span>
                <span> Mission </span>
                <span> Arrival </span>
                <span> Speed </span>
            </div>
        }
        .into_view();

        TreeItem {
            view,
            id: "Flights".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(_, ship)| FlightTreeItem(ship).into_tree_item())
                .collect(),
            collapsed: create_rw_signal(false),
        }
    }
}

pub struct FlightTreeItem(pub Flight);

impl IntoTreeItem for FlightTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        #[allow(unused_braces)]
        let view = view! {
            <div class="grid grid-cols-4 gap-4">
                <span> {self.0.from_id.clone()} </span>
                <span> {self.0.to_id.clone()} </span>
                <span> {self.0.ships.clone()} </span>
                <span> {self.0.mission.to_string()} </span>
                <span> {self.0.arrival_time.clone()} </span>
                <span> {self.0.speed_ratio.clone()} </span>
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
