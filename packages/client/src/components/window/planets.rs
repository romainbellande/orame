use leptos::*;

use crate::{
    components::{
        tree_row::{views::PlanetsTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn PlanetsWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let planets_tree_view = move || PlanetsTreeItem(state().planets.clone());

    view! {
        <Window title="Planets">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=planets_tree_view />
            </ul>
        </Window>
    }
}