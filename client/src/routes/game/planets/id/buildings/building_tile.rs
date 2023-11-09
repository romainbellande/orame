use leptos::*;
use web_sys::MouseEvent;

use crate::data::building::Building;

#[component]
pub fn BuildingTile(
    building: ReadSignal<Building>,
    #[prop(into)] on_toggle: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
      <li class=format!("w-24 h-24 relative hover:brightness-125 cursor-pointer icon sprite sprite_medium medium {}", building.get().config.class) on:click=on_toggle >
      </li>
    }
}
