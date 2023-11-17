use leptos::*;

use crate::{
    components::{
        tree_row::{views::universe::UniverseTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn UniverseWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let universe_tree_view = move || UniverseTreeItem(state().game_data.clone());

    view! {
        <Window title="Universe">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=universe_tree_view />
            </ul>
        </Window>
    }
}
