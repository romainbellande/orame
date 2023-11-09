use crate::data::ship::Ship;
use leptos::*;

#[component]
pub fn ShipTile(ship: ReadSignal<Ship>) -> impl IntoView {
    view! {
      <li class=format!("icon sprite sprite_small small  {}", ship().class)></li>
    }
}
