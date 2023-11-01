use crate::components::window::Window;
use leptos::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);

    view! {
      <div>
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <div class="text-white">
              "buildings"
            </div>
          </Window>
        </Show>
        <ul>
          <li>
            <button on:click=move |_| set_show_buildings(!show_buildings())>"buildings"</button>
          </li>
        </ul>
      </div>
    }
}
