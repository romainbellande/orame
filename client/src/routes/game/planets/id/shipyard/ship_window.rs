use leptos::*;
use ogame_core::planet::Planet;

use crate::{
    components::window::Window, data::ship::Ship, routes::game::planets::id::ui::PlanetUI,
};

#[component]
pub fn ShipWindow(
    ui: ReadSignal<PlanetUI>,
    ship: ReadSignal<Ship>,
    planet: Signal<Planet>,
) -> impl IntoView {
    let amount =
        move || with!(|ship, planet| *planet.ships.ships.get(&ship.ship_type).unwrap_or(&0));

    view! {
      <Show when=move || ui().is_ship_visible(ship().get_type()).get()>
        <Window title="Shipyard" on_show=move |value: bool| ui().set_ship_visibility(ship().get_type(), value)>
          <div class="space-y-4">
            <h3 class="flex justify-between items-center"><span>{ ship().name }</span><span>"amount " {amount}</span></h3>
            <div class=format!("sprite sprite_large building {}", ship().class)></div>
            <p class="text-xs">{ ship().description }</p>
            <div class="flex justify-end">

              <button type="button" class="rounded bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"upgrade"</button>
            </div>
          </div>
        </Window>
      </Show>
    }
}
