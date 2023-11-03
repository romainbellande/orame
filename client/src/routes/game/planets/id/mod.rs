use leptos::*;
use leptos_router::use_params_map;

mod toolbar;
use toolbar::Toolbar;

use crate::components::galaxy::Galaxy;
use crate::components::planets::Planets;

#[component]
pub fn PlanetIDPage() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! {
      <div class="flex-grow flex flex-col justify-between">
        <section>
          <div>"My Planet"</div>
          <Galaxy />
          <Planets />
        </section>
        <Toolbar />
      </div>
    }
}
