use leptos::*;

use crate::{
    components::{
        tree_row::{views::BuildingsByPlanetTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn BuildingsByPlanetWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let buildings_by_planet = move || BuildingsByPlanetTreeItem(state().planets.clone());

    view! {
        <Window title="Buildings">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=buildings_by_planet />
            </ul>
        </Window>
    }
}
