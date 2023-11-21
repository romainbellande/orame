use leptos::*;

use crate::{
    components::{
        tree_row::{views::ships::ShipsTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn ShipsWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let ships_tree_view = move || ShipsTreeItem(state().ships.clone());

    view! {
        <Window title="Ships">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=ships_tree_view() />
            </ul>
        </Window>
    }
}
