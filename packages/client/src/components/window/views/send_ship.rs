use leptos::*;
use ogame_core::{flight::MissionType, protocol::Protocol, PositionedEntity};

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
        DestinationSelectionTreeItem(ogame_core::GAME_DATA.read().unwrap().clone(), selected_dest)
            .into_tree_item(),
    );

    let send_ships = move |_| {
        state()
            .action(Protocol::SendShips {
                from_id: selected_ship().unwrap().position_id.clone(),
                to_id: selected_dest().unwrap().id().clone(),
                ships: vec![selected_ship().unwrap().clone()],
                mission: MissionType::Transport,
                speed_ratio: 1,
            })
            .unwrap();
    };

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
                            None
                        }.into_view()
                    }
                }
            </div>
            <div>
                <span class="float-left"> Distance:</span>
                { move ||
                    if let Some(ship) = selected_ship() {
                        if let Some(dest) = selected_dest() {
                            let distance = ship.distance_to(&dest);
                            view! {
                                #[allow_unused(braces)]
                                { distance }
                            }.into_view()
                        } else {
                            view! {
                                None
                            }.into_view()
                        }
                    } else {
                        view! {
                            None
                        }.into_view()
                    }
                }

            </div>

            <div class="h-40 m-4">
                <div> Ship selection: </div>
                <div class="overflow-auto h-40 border border-gray-600">
                    <TreeRow tree_item=ship_selection() />
                </div>
            </div>
            <div class="h-40 m-4 mt-4">
                <div> Destination selection: </div>
                <div class="overflow-auto h-40 border border-gray-600">
                    <TreeRow tree_item=destination_selection />
                </div>
            </div>
            <div>
                <button on:click=send_ships>
                    Send
                </button>
            </div>
        </Window>
    }
}
