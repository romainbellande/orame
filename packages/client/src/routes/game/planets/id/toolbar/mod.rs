use web_sys::MouseEvent;

use crate::{
    components::window::Window,
    data::building::Building,
    routes::game::planets::id::{
        buildings::{BuildingTile, BuildingWindow},
        shipyard::ShipyardWindow,
        ui::PlanetUI,
    },
};
use leptos::*;
use ogame_core::{building_type::BuildingType, planet::Planet};

pub fn create_building(building_type: BuildingType, planet: Signal<Planet>) -> Building {
    Building::from(building_type.clone())
        .set_level(planet.get_untracked().building_level(building_type))
}

#[component]
pub fn ToolbarItem(
    #[prop(into)] on_click: Callback<MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
      <li class="px-2 py-4 hover:bg-slate-400 hover:text-slate-900 cursor-pointer" >
          <button on:click=on_click>{children()}</button>
      </li>
    }
}

#[component]
pub fn Toolbar(planet: Signal<Planet>) -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);

    let (planet_ui, _) = create_signal(PlanetUI::new());

    let buildings = create_memo(move |_| {
        vec![
            create_building(BuildingType::Metal, planet),
            create_building(BuildingType::Crystal, planet),
            create_building(BuildingType::Deuterium, planet),
            create_building(BuildingType::Shipyard, planet),
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
          children=move |(_id, (building, _))| {
            view! {
              <Show when=move || planet_ui().is_building_visible(building().get_type()).get()>
                <BuildingWindow building=building planet=planet ui=planet_ui/>
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
                children=move |(_id, (building, _))| {
                  view! {
                      <BuildingTile building=building on_toggle=move |_| { planet_ui().toggle_building_window(building().get_type()); } />
                  }
                }
              />
            </ul>
          </Window>
        </Show>

        <ShipyardWindow ui=planet_ui planet=planet />

        <ul class="space-x-4 flex item-center">
          <ToolbarItem on_click=move |_| set_show_buildings(!show_buildings())>"buildings"</ToolbarItem>
          <ToolbarItem on_click=move |_| planet_ui().toggle_shipyard_window()>"shipyard"</ToolbarItem>
        </ul>
      </div>
    }
}
