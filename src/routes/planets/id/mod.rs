use leptos::*;
use leptos_router::use_params_map;

mod toolbar;
use toolbar::Toolbar;

#[component]
pub fn PageID() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! {
      <div>
        <Toolbar />
      </div>
    }
}
