use super::building::{Building, BuildingTile, BuildingWindow};
use crate::components::window::Window;
use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{building_type::BuildingType, game::Game, planet::Planet};
use uuid::Uuid;

#[component]
pub fn Toolbar(planet: Signal<Planet>) -> impl IntoView {
    let state = expect_context::<RwSignal<Game>>();
    let (show_buildings, set_show_buildings) = create_signal(false);

    let initial_buildings = Signal::derive(move || {
        planet.with(|planet| {
            vec![
                Building::from(BuildingType::Metal)
                    .set_level(planet.building_level(BuildingType::Metal)),
                Building::from(BuildingType::Crystal)
                    .set_level(planet.building_level(BuildingType::Crystal)),
                Building::from(BuildingType::Deuterium)
                    .set_level(planet.building_level(BuildingType::Deuterium)),
            ]
            .into_iter()
            .map(|building| (Uuid::new_v4(), create_signal(building)))
            .collect::<Vec<_>>()
        })
    });

    let (buildings, _) = create_signal(initial_buildings);

    // let on_upgrade = move |building: ReadSignal<Building>| {

    //   Callback::new(move |_| {
    //     console_log(format!("upgrade building: {:?}", building().get_type()).as_str());
    //       if let Err(e) = state().upgrade_building(planet.get().id(), building().get_type()) {
    //         console_log(format!("Error upgrade building: {:?}", e).as_str());
    //     }
    //   })
    // };

    view! {
      <div class="bg-black text-white">
      <For
          each=initial_buildings
          key=|building| building.0
          children=move |(id, (building, _))| {
            view! {
              <BuildingWindow building=building planet=planet />
            }
          }
        />
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <ul class="text-white flex space-x-4">
              <For
                each=initial_buildings
                key=|building| building.0
                children=move |(id, (building, _))| {
                  view! {
                    <BuildingTile building=building />
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
