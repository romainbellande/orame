use leptos::{leptos_dom::logging::console_log, *};
use ogame_core::{
    build_cost_trait::BuildCost, build_time_trait::BuildTime, building_type::BuildingType,
};

use crate::utils::GameWrapper;

#[component]
pub fn Planets() -> impl IntoView {
    let (_show_buildings, _set_show_buildings) = create_signal(false);
    let state = expect_context::<RwSignal<GameWrapper>>();

    let upgrade_cb = move |planet: ogame_core::planet::Planet, building: BuildingType| {
        move |_| {
            state.update(|state| {
                if let Err(e) = state.upgrade_building(planet.id.clone(), building.clone()) {
                    console_log(format!("Error upgrade building: {:?}", e).as_str());
                }
            });
        }
    };

    let build_queue =
        move |id: String| move || state().planets.get(&id).unwrap().build_queue.items.clone();
    let buildings = move |id: String| move || state().planets.get(&id).unwrap().buildings.clone();

    let planets = move || state().planets.values().cloned().collect::<Vec<_>>();

    let now = move || {
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    };

    view! {
        <For
            each=planets
            key=|planet| planet.last_update
            let:planet
        >
            {
                let available_resources = planet.resources.clone();

                let metal = available_resources.metal.floor();
                let crystal = available_resources.crystal.floor();
                let deuterium = available_resources.deuterium.floor();

                let metal_production = BuildingType::Metal.produced(planet.building_level(BuildingType::Metal), 3600).metal.floor();
                let crystal_production = BuildingType::Crystal.produced(planet.building_level(BuildingType::Crystal), 3600).crystal.floor();
                let deuterium_production = BuildingType::Deuterium.produced(planet.building_level(BuildingType::Deuterium), 3600).deuterium.floor();

                view ! {
                    <div>
                        <li> Metal: {metal} + {metal_production}/h </li>
                        <li> Crystal: {crystal} + {crystal_production}/h </li>
                        <li> Deuterium: {deuterium} + {deuterium_production}/h </li>
                    </div>
                    {
                        let planet2 = planet.clone();
                        view! {
                            <For
                                each=buildings(planet2.id.clone())
                                key=|building| building.1
                                let:building
                            >
                                {
                                    let planet3 = planet2.clone();
                                    let extra_levels_in_build_queue = planet2.build_queue.items.iter().filter(|item| item.r#type == building.0).count() + 1;
                                    let cost = building.0.cost(building.1 + extra_levels_in_build_queue);

                                    let metal_cost = cost.metal.floor();
                                    let crystal_cost = cost.crystal.floor();
                                    let deuterium_cost = cost.deuterium.floor();

                                    let build_time = building.0.build_time(building.1 + extra_levels_in_build_queue);

                                    view! {
                                        <li>
                                            Building {building.0.to_string()}: lvl {state().planets.get(&planet3.id).unwrap().building_level(building.0.clone())}
                                            <button on:click=upgrade_cb(planet3.clone(), building.clone().0.clone())>
                                                "("
                                                upgrade
                                                {metal_cost}/{crystal_cost}/{deuterium_cost}
                                                "|"
                                                {build_time}s
                                                ")"
                                            </button>
                                        </li>
                                    }
                                }
                                </For>
                            <For
                                each=build_queue(planet.id)
                                key=|item| item.finish_date
                                let:item
                            >
                                <li>
                                    {item.r#type.to_string()} {item.finish_date as i32 - now() as i32}s
                                </li>
                            </For>
                        }
                    }
                }
            }
        </For>
    }
}
