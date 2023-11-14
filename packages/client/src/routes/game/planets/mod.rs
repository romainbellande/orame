use leptos::*;
use leptos_router::*;
mod id;
pub use id::PlanetIDPage;

use crate::utils::GameWrapper;

fn get_planet_style(dimensions: usize, color: &'static str) -> String {
    format!(
        "
    width:{dimensions}px;
    height:{dimensions}px;
    border-radius:50%;
    box-shadow: 0 0 10px {color};
    background:linear-gradient(to bottom right, {color}, white);
    margin-top:-{dimensions}px/2;
    margin-left:-{dimensions}px/2;
  "
    )
}

#[component]
pub fn PlanetsPage() -> impl IntoView {
    let game_wrapper = expect_context::<RwSignal<GameWrapper>>();

    let planets = Signal::derive(move || game_wrapper.with(|state| state.planets.clone()));

    let navigate_to_planet = |planet_id: String| {
        move |_| {
            let navigate = use_navigate();
            navigate(format!("/planets/{planet_id}").as_str(), Default::default())
        }
    };

    view! {
      <div class="flex items-center h-full flex-grow justify-center">
        <For
          each=planets
          key=|planet| planet.0.clone()
          let:planet
        >
          <div style=get_planet_style(100, "#66b2ff") class="hover:opacity-70 cursor-pointer" on:click=navigate_to_planet(planet.1.id.clone())></div>
        </For>
      </div>
    }
}
