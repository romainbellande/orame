use leptos::*;

use crate::{
    components::{
        tree_row::{views::flights::FlightsTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn FlightsWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let flights_tree_view = move || FlightsTreeItem(state().flights.clone());

    view! {
        <Window title="Flights">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=flights_tree_view() />
            </ul>
        </Window>
    }
}
