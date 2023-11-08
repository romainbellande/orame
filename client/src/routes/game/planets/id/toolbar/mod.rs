mod ui;
use ui::ToolbarUI;

mod building;
use crate::components::window::Window;
use building::{Building, BuildingTile, BuildingWindow};
use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{building_type::BuildingType, planet::Planet};

#[component]
pub fn Toolbar(planet: Memo<Planet>) -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);

    let (toolbar_ui, _) = create_signal(ToolbarUI::new());

    let buildings = create_memo(move |_| {
        console_log("triggered");
        vec![
            Building::from(BuildingType::Metal)
                .set_level(planet.get_untracked().building_level(BuildingType::Metal)),
            Building::from(BuildingType::Crystal)
                .set_level(planet.get_untracked().building_level(BuildingType::Crystal)),
            Building::from(BuildingType::Deuterium).set_level(
                planet
                    .get_untracked()
                    .building_level(BuildingType::Deuterium),
            ),
        ]
        .into_iter()
        .map(|building| (building.id, create_signal(building)))
        .collect::<Vec<_>>()
    });

    view! {
      <div class="bg-black text-white">
      <For
          each=buildings
          key=|building| building.0
          children=move |(id, (building, _))| {
            view! {
              <Show when=move || toolbar_ui().is_building_visible(building().get_type()).get()>
                <BuildingWindow building=building planet=planet />
              </Show>
            }
          }
        />
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <ul class="text-white flex space-x-4">
              <For
                each=buildings
                key=|building| building.0
                children=move |(id, (building, _))| {
                  view! {
                      <BuildingTile building=building on_toggle=move |_| { toolbar_ui().toggle_building_window(building().get_type()); } />
                  }
                }
              />
            </ul>
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
