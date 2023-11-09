use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::planet::Planet;

use crate::{
    components::window::Window, data::building::Building, routes::game::planets::id::ui::PlanetUI,
    utils::GameWrapper,
};

#[component]
pub fn BuildingWindow(
    building: ReadSignal<Building>,
    planet: Signal<Planet>,
    ui: ReadSignal<PlanetUI>,
) -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();

    let on_upgrade = move |building: Building, planet: Planet| {
        move |_| {
            console_log(format!("upgrade building: {:?}", building.get_type()).as_str());
            state.update(|state| {
                if let Err(e) = state.upgrade_building(planet.id.clone(), building.get_type()) {
                    console_log(format!("Error upgrade building: {:?}", e).as_str());
                }
            });
        }
    };

    let level = move || with!(|building, planet| building.get_level_from_planet(planet));

    view! {
          <Window title="Building" on_show=move |value: bool| ui().set_building_visibility(building().config.building_type, value)>
            <div class="space-y-4">
              <h3><span>{ building().config.name }</span><span>"level " {level}</span></h3>
              <div class=format!("sprite sprite_large building {}", building().config.class)></div>
              <p class="text-xs">{ building().config.description }</p>
              <div class="flex justify-end">

                <button type="button" on:click=on_upgrade(building(), planet()) class="rounded bg-indigo-500 p-2 px-4 text-white hover:bg-orange-500">"upgrade"</button>
              </div>
            </div>
          </Window>
    }
}
