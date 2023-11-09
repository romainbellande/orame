use leptos::*;
use ogame_core::{ship_type::ShipType, planet::Planet};
use crate::{components::window::Window, data::ship::{Ship, get_all_ships}};
mod ship_tile;
use ship_tile::ShipTile;
mod ship_window;
use ship_window::ShipWindow;

use super::ui::PlanetUI;

pub fn create_ship(ship_type: ShipType, planet: Signal<Planet>) -> (ShipType, (ReadSignal<Ship>, WriteSignal<Ship>)) {
  let amount = planet.get().ships.ships.get(&ship_type.clone()).unwrap_or(&0).clone();
  let ship = Ship::from(ship_type.clone()).set_amount(amount);
  (ship_type, create_signal(ship))
}

#[component]
pub fn ShipyardWindow(ui: ReadSignal<PlanetUI>, planet: Signal<Planet>) -> impl IntoView {
  let ships = create_memo(move |_| {
    get_all_ships().into_iter().map(|ship_type| create_ship(ship_type, planet.clone())).collect::<Vec<_>>()
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
