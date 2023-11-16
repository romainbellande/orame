use leptos::*;

use crate::{
    components::{
        tree_row::{views::ShipsByPlanetTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn ShipsByPlanetWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let ships_by_planet = move || ShipsByPlanetTreeItem(state().planets.clone());

    view! {
        <Window title="Ships">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=ships_by_planet />
            </ul>
        </Window>
    }
}
