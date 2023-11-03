use crate::components::window::Window;
use leptos::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);
    view! {
      <div class="bg-black text-white">
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <div class="text-white">
              "buildings"
            </div>
          </Window>
        </Show>
        <ul class="space-x-4 flex item-center">
          <li class="px-2 py-4 hover:bg-slate-400 hover:text-slate-900 cursor-pointer" >
            <button on:click=move |_| set_show_buildings(!show_buildings())>"buildings"</button>
          </li>
          <li class="px-2 py-4 hover:bg-slate-400 hover:text-slate-900 cursor-pointer" >
          "El Jamon"
          </li>
        </ul>
      </div>
    }
}
