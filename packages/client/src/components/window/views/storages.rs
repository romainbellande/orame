use leptos::*;

use crate::{
    components::{
        tree_row::{views::storages::StoragesTreeItem, TreeRow},
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn StoragesWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let storages_tree_view = move || StoragesTreeItem(state().storages.clone());

    view! {
        <Window title="Storages">
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=storages_tree_view />
            </ul>
        </Window>
    }
}
