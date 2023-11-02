use crate::components::window::Window;
use leptos::wasm_bindgen::JsCast;
use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{building_type::BuildingType, game::Game};
use wasm_bindgen::prelude::Closure;

#[component]
pub fn Toolbar() -> impl IntoView {
    let (show_buildings, set_show_buildings) = create_signal(false);
    let state = expect_context::<RwSignal<Game>>();

    state.update(|state| {
        state.add_planet(ogame_core::planet::Planet::new(0));
    });

    let upgrade_cb = move |planet: ogame_core::planet::Planet, building: BuildingType| {
        move |_| {
            state.update(|state| {
                state
                    .upgrade_building(planet.id(), building.clone())
                    .unwrap();
            });
            console_log(format!("{:#?}", state().planets).as_str());
        }
    };

    let build_queue = move |id: usize| move || state().planets[id].build_queue.items.clone();
    let buildings = move |id: usize| move || state().planets[id].buildings.clone();

    let planets = move || state().planets;

    let now = move || {
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    };

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
        <For
            each=planets
            key=|planet| planet.last_update()
            let:planet
        >
            <div>
                <li>Metal: {state().planets[planet.id()].resources().metal}</li>
                <li>Crystal: {state().planets[planet.id()].resources().crystal}</li>
                <li>Deuterium: {state().planets[planet.id()].resources().deuterium}</li>
            </div>
            {
                let planet2 = planet.clone();
                view! {
                    <For
                        each=buildings(planet2.id())
                        key=|building| building.1
                        let:building
                    >
                        {
                            let planet3 = planet2.clone();
                            view! {
                                <li>
                                    Building {building.0.to_string()}: lvl {state().planets[planet3.id()].building_level(building.0.clone())}
                                    <button on:click=upgrade_cb(planet3.clone(), building.clone().0.clone())>
                                        "(upgrade)"
                                    </button>
                                </li>
                            }
                        }
                        </For>
                    <For
                        each=build_queue(planet.id())
                        key=|item| item.finish_date
                        let:item
                    >
                        <li>
                            {item.r#type.to_string()} {item.finish_date - now()}s
                        </li>
                    </For>
                }
            }
        </For>
      </div>
    }
}
