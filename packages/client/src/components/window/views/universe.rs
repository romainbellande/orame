use leptos::*;

use crate::components::{
    tree_row::{views::universe::UniverseTreeItem, TreeRow},
    window::Window,
};

#[component]
pub fn UniverseWindow() -> impl IntoView {
    let universe_tree_view =
        move || UniverseTreeItem(ogame_core::GAME_DATA.read().unwrap().clone());

    view! {
        <Window title="Universe">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=universe_tree_view />
            </ul>
        </Window>
    }
}
