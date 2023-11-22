use leptos::*;
use ogame_core::{
    flight::{Flight, MissionType},
    protocol::Protocol,
    PositionedEntity,
};

use crate::{
    components::{
        tree_row::{
            views::flights::{
                DestinationSelectionTreeItem, FlightsTreeItem, ShipsSelectionTreeItem,
            },
            IntoTreeItem, TreeRow,
        },
        window::Window,
    },
    utils::GameWrapper,
};

#[component]
pub fn FlightsWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let selected_ship = create_rw_signal(None);
    let selected_dest = create_rw_signal(None);

    let ship_selection =
        create_memo(move |_| ShipsSelectionTreeItem(state().ships.clone(), selected_ship));

    let destination_selection = store_value(
        DestinationSelectionTreeItem(ogame_core::GAME_DATA.read().unwrap().clone(), selected_dest)
            .into_tree_item(),
    );

    let flights_tree_view = create_memo(move |_| FlightsTreeItem(state().flights.clone()));

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
        <Window title="Flights">
            <div class="space-x-4 mb-4">
                <TreeRow tree_item=flights_tree_view />
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
                <div class="float-left"> Distance:
                { move ||
                    if let Some(ship) = selected_ship() {
                        if let Some(dest) = selected_dest() {
                            let distance = ship.distance_to(&dest);
                            view! {
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
                <div> Duration:
                { move ||
                    if let Some(ship) = selected_ship() {
                        if let Some(dest) = selected_dest() {
                            let duration = Flight::formated_duration(Flight::duration_of_flight(&ship.position_id, &dest.id(), 1));
                            view! {
                                { duration }
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

            </div>
            <div>
                <button on:click=send_ships>
                    Send
                </button>
            </div>

        </Window>
    }
}
