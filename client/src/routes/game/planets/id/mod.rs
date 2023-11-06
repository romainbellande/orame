use ogame_core::game::Game;

use leptos::*;
use leptos_router::use_params_map;

mod toolbar;
use toolbar::Toolbar;

mod resource_bar;
use resource_bar::ResourceBar;
use web_sys::wasm_bindgen::UnwrapThrowExt;

mod building;

use crate::components::galaxy::Galaxy;
use crate::components::planets::Planets;

#[component]
pub fn PlanetIDPage() -> impl IntoView {
    let state = expect_context::<RwSignal<Game>>();

    state.update(|state| {
        state.add_planet(ogame_core::planet::Planet::new(0));
    });

    let params = use_params_map();
    let _id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let planets = move || state().planets;

    let planet = Signal::derive(move || state.with(|state| state.planets.get(0).expect("must have at least one planet").clone()));

    view! {
      <div class="flex-grow flex flex-col justify-between bg-slate-800">
        <section>
          <div class="w-full flex justify-center">
            <ResourceBar planet=planet />
          </div>
          <div>"My Planet"</div>
          <Galaxy />
          <Planets />
        </section>
        <Toolbar planet=planet />
      </div>
    }
}
