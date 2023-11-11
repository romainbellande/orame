use leptos::leptos_dom::logging::console_log;
use ogame_core::game::Game;

use leptos::wasm_bindgen::UnwrapThrowExt;
use leptos::*;
use leptos_router::use_params_map;
mod toolbar;
use ogame_core::planet::Planet;
use toolbar::Toolbar;

mod resource_bar;
use resource_bar::ResourceBar;

use crate::components::galaxy::GalaxyView;
use crate::components::planets::Planets;
use crate::utils::GameWrapper;

#[component]
pub fn PlanetIDPage() -> impl IntoView {
    let game_wrapper = expect_context::<RwSignal<GameWrapper>>();

    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let planet: Signal<Option<Planet>> = Signal::derive(move || {
        game_wrapper.with(|state| {
            console_log(format!("id: {} planets: {:?}", &id(), state.planets).as_str());
            match state.planets.clone().get(&id()) {
                Some(planet) => Some(planet.clone()),
                None => None,
            }
        })
    });

    view! {
      <Show when=move || planet().is_some()>
        <div class="flex-grow flex flex-col justify-between bg-slate-800">
          <section>
            <div class="w-full flex justify-center">
              <ResourceBar planet=Signal::derive(move || planet.get().unwrap().clone()) />
            </div>
            <div>"My Planet"</div>
            <GalaxyView />
            <Planets />
          </section>
          <Toolbar planet=Signal::derive(move || planet.get().unwrap().clone())  />
        </div>

      </ Show>
    }
}
