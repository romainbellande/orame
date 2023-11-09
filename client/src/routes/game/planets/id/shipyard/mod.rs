use crate::{components::window::Window, data::ship::Ship};
use leptos::{html::s, *};
use ogame_core::{planet::Planet, ship_type::ShipType};
mod ship_tile;

use super::ui::PlanetUI;

pub fn create_ship(ship_type: ShipType, planet: Signal<Planet>) -> Ship {
    let amount = planet
        .get()
        .ships
        .ships
        .get(&ship_type)
        .unwrap_or(&0)
        .clone();
    Ship::from(ship_type).set_amount(amount)
}

#[component]
pub fn ShipyardWindow(ui: ReadSignal<PlanetUI>, planet: Signal<Planet>) -> impl IntoView {
    let ships = create_memo(move |_| vec![]);

    view! {
      <Show when=move || ui().shipyard.get()>
        <Window title="Shipyard" on_show=move |value: bool| ui().shipyard.set(value)>
          <ul>
            <li class=format!("sprite sprite_large building {}", building().config.class)></li>
          </ul>
        </Window>
      </Show>
    }
}
