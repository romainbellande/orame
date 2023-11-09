use crate::data::ship::Ship;
use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn ShipTile(
    ship: ReadSignal<Ship>,
    #[prop(into)] on_toggle: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
      <li on:click=on_toggle class=format!("hover:brightness-125 cursor-pointer icon sprite sprite_small small m-2  {}", ship().class)></li>
    }
}
