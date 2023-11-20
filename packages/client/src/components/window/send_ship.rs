use leptos::*;

use crate::{
    components::{
        tree_row::{
            views::send_ship::{
                destination_selection::DestinationSelectionTreeItem,
                ship_selection::ShipsSelectionTreeItem,
            },
            IntoTreeItem, TreeRow,
        },
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn SendShipWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let selected_ship = create_rw_signal(None);
    let selected_dest = create_rw_signal(None);

    let ship_selection = create_memo(move |_| {
        ShipsSelectionTreeItem(state().ships.clone(), selected_ship).into_tree_item()
    });

    let destination_selection = store_value(
        DestinationSelectionTreeItem(crate::GAME_DATA.read().unwrap().clone(), selected_dest)
            .into_tree_item(),
    );

    view! {
        <Window title="SendShip">
            <div>
                <span class="float-left"> Selected ship:</span>
                {move || if let Some(ship) = selected_ship() { ship.id.clone() } else { "None".to_string() }}
            </div>
            <div>
                <span class="float-left"> Selected destination:</span>
                { move ||
                    if let Some(dest) = selected_dest() {
                        dest.view().into_view()
                    } else {
                        view! {
                            { "None".to_string() }
                        }.into_view()
                    }
                }
            </div>

            <div class="h-40 m-4">
                <div> Ship selection: </div>
                <div class="overflow-auto h-40 border border-gray-600">
                    <TreeRow tree_item=ship_selection />
                </div>
            </div>
            <div class="h-40 m-4 mt-4">
                <div> Destination selection: </div>
                <div class="overflow-auto h-40 border border-gray-600">
                    <TreeRow tree_item=destination_selection />
                </div>
            </div>
            <div>
                <button> Send </button>
            </div>
        </Window>
    }
}
