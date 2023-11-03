use crate::components::window::Window;
use leptos::*;

#[derive(Clone, PartialEq)]
enum Building {
  MetalMine,
  CrystalMine,
  None,
}

#[component]
pub fn Toolbar() -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);
    let (selected_building, set_selected_building) = create_signal(Building::None);
    let (show_building, set_show_building) = create_signal(false);

    let set_building = move |building: Building| {
      if selected_building() == building {
        set_show_building(false);
        set_selected_building(Building::None);
      } else {
        set_show_building(true);
        set_selected_building(building);
      }
    };

    // let show_building = create_memo(move |_| {
    //   match selected_building() {
    //     Building::None => false,
    //     _ => true,
    //   }
    // });

    let get_building_name = create_memo(move |_| {
      match selected_building.get() {
        Building::MetalMine => "Metal Mine",
        Building::CrystalMine => "Crystal Mine",
        _ => "",
      }
    });

    view! {
      <div class="bg-black text-white">
        <Show when=show_building>
          <Window title="Building" on_show=set_show_building>
            <div>
              {
                get_building_name
              }
            </div>
          </Window>
        </Show>
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <div class="text-white flex space-x-4">
              <div class="w-24 h-24 relative hover:brightness-125 cursor-pointer" on:click=move |_| { set_building(Building::MetalMine)  }>
                <img class="h-full w-full" src="/public/images/metal-mine.png" />
              </div>
              <div class="w-24 h-24 relative hover:brightness-125 cursor-pointer"  on:click=move |_| { set_building(Building::CrystalMine) }>
                <img class="h-full w-full" src="/public/images/crystal-mine.png" />
              </div>
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
