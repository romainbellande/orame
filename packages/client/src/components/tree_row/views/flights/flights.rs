use std::collections::BTreeMap;

use leptos::*;

use ogame_core::flight::Flight;

use crate::{
    components::tree_row::{IntoTreeItem, TreeItem},
    utils::GameWrapper,
};

#[derive(Clone, PartialEq)]
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
        let state = expect_context::<RwSignal<GameWrapper>>();
        let self_copy = self.0.clone();
        let duration = create_memo(move |_| {
            state();
            self_copy.get_formated_duration()
        });

        let from = ogame_core::GAME_DATA
            .read()
            .unwrap()
            .get_item_at_position(&self.0.from_id)
            .unwrap();

        let to = ogame_core::GAME_DATA
            .read()
            .unwrap()
            .get_item_at_position(&self.0.to_id)
            .unwrap();

        #[allow(unused_braces)]
        let view = view! {
            <div class="grid grid-cols-6 gap-4">
                <span> {from.name()} </span>
                <span> {to.name()} </span>
                <span> {self.0.ships.len()} </span>
                <span> {self.0.mission.to_string()} </span>
                <span> {
                    { duration }
                } </span>
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
