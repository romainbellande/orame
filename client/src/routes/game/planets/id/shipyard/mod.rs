use crate::{
    components::window::Window,
    data::ship::{get_all_ships, Ship},
};
use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{planet::Planet, ship_type::ShipType};
mod ship_tile;
use ship_tile::ShipTile;
mod ship_window;
use ship_window::ShipWindow;

use super::ui::PlanetUI;

pub fn create_ship(ship_type: ShipType, planet: Signal<Planet>) -> Ship {
    Ship::from(ship_type.clone())
}

#[component]
pub fn ShipyardWindow(ui: ReadSignal<PlanetUI>, planet: Signal<Planet>) -> impl IntoView {
    let ships = create_memo(move |_| {
        console_log("triggered");
        get_all_ships()
            .into_iter()
            .map(|ship_type| {
                let ship = create_ship(ship_type, planet);
                (ship.id, create_signal(ship))
            })
            .collect::<Vec<_>>()
    });

    view! {
      <For
        each=ships
        key=|ship| ship.0.clone()
        children=move |(_id, (ship, _))| {
          view! {
            <ShipWindow ship=ship ui=ui planet=planet />
          }
        }
      />

      <Show when=move || ui().shipyard.get()>
        <Window title="Shipyard" on_show=move |value: bool| ui().shipyard.set(value) width=26>
          <ul class="flex flex-wrap">
            <For
              each=ships
              key=|ship| ship.0.clone()
              children=move |(_id, (ship, _))| {
                view! {
                  <ShipTile ship=ship on_toggle=move |_| {  ui().toggle_ship_window(ship().get_type()); } />
                }
              }
            />

          </ul>
        </Window>
      </Show>
    }
}
