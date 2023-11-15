use std::collections::BTreeMap;

use leptos::*;

mod sidenav_item;
use ogame_core::{
    building_type::BuildingType, planet::Planet, resources::ResourceType, ship_type::ShipType,
};
use sidenav_item::SidenavItem;

use crate::{components::window::Window, utils::GameWrapper};

trait IntoTreeItem {
    fn into_tree_item(&self) -> TreeItem;
}

#[derive(Clone)]
struct TreeItem {
    pub view: View,
    pub id: String,
    pub children: Vec<TreeItem>,
}

pub struct BuildingsByPlanetTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for BuildingsByPlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            "Planets"
        }
        .into_view();

        TreeItem {
            view,
            id: "Planets".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(id, planet)| PlanetWithBuildingsTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct ShipsByPlanetsTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for ShipsByPlanetsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            "Planets"
        }
        .into_view();

        TreeItem {
            view,
            id: "Planets".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(id, planet)| PlanetWithShipsTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct PlanetWithShipsTreeItem(pub Planet);

impl IntoTreeItem for PlanetWithShipsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            {self.0.id.clone()}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: self
                .0
                .ships
                .clone()
                .ships
                .into_iter()
                .map(|b| ShipTreeItem(b).into_tree_item())
                .collect(),
        }
    }
}

pub struct ShipTreeItem(pub (ShipType, usize));

impl IntoTreeItem for ShipTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            {self.0.0.to_string()} {self.0.1}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0 .0.to_string(),
            children: vec![],
        }
    }
}

pub struct PlanetWithBuildingsTreeItem(pub Planet);

impl IntoTreeItem for PlanetWithBuildingsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            {self.0.id.clone()}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: self
                .0
                .buildings
                .clone()
                .into_iter()
                .map(|b| BuildingTreeItem(b).into_tree_item())
                .collect(),
        }
    }
}

pub struct BuildingTreeItem(pub (BuildingType, usize));

impl IntoTreeItem for BuildingTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            {self.0.0.to_string()} {self.0.1}
        }
        .into_view();

        TreeItem {
            view,
            id: self.0 .0.to_string(),
            children: vec![],
        }
    }
}

pub struct PlanetsTreeItem(pub BTreeMap<String, Planet>);

impl IntoTreeItem for PlanetsTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            "Planets"
        }
        .into_view();

        TreeItem {
            view,
            id: "Planets".to_string(),
            children: self
                .0
                .clone()
                .into_iter()
                .map(|(id, planet)| PlanetTreeItem(planet).into_tree_item())
                .collect(),
        }
    }
}

pub struct PlanetTreeItem(pub Planet);

impl IntoTreeItem for PlanetTreeItem {
    fn into_tree_item(&self) -> TreeItem {
        let view = view! {
            <span>{self.0.id.clone()}</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Metal).floor()} metal</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Crystal).floor()} crystal</span>
            <span class="ml-2">{self.0.resources.get(ResourceType::Deuterium).floor()} deuterium</span>
        }
        .into_view();

        TreeItem {
            view,
            id: self.0.id.clone(),
            children: vec![],
        }
    }
}

#[component]
pub fn TreeRow(
    tree_item: TreeItem,
    #[prop(default = 255)] depth: u8,
    #[prop(optional)] is_child: bool,
) -> impl IntoView {
    let tree_item = create_rw_signal(tree_item);
    let (collapsed, set_collapsed) = create_signal(is_child);

    if depth == 0 {
        return view! {<div></div>};
    }

    let arrow_classes = move || {
        if depth == 0 || !is_child || tree_item().children.is_empty() {
            ""
        } else if collapsed() {
            "arrow right"
        } else {
            "arrow down"
        }
    };

    view! {
        <div class="flex flex-col">
            <div on:click=move |_| if is_child {set_collapsed(!collapsed())}>
                <i class=arrow_classes> </i>
                {tree_item().view}
            </div>
            <For
                each=move || tree_item().children
                key=move |child| child.id.clone()
                let:child
            >
                {
                    let child = create_rw_signal(child.clone());

                    view! {
                        <Show when=move || !collapsed()>
                            <div class="ml-4 pt-2">
                                <TreeRow tree_item=child() depth={depth - 1} is_child=true/>
                            </div>
                        </Show>
                    }
                }
            </For>
        </div>
    }
}

#[component]
pub fn SideNav() -> impl IntoView {
    let state = expect_context::<RwSignal<GameWrapper>>();
    let (show_buildings, set_show_buildings) = create_signal(false);
    let (show_ships, set_show_ships) = create_signal(false);
    let (show_planets, set_show_planets) = create_signal(false);
    let buildings_by_planets = move || BuildingsByPlanetTreeItem(state().planets.clone());
    let planets_tree_view = move || PlanetsTreeItem(state().planets.clone());
    let ships_by_planets = move || ShipsByPlanetsTreeItem(state().planets.clone());

    view! {
        <Show when=show_planets>
          <Window title="Planets" on_show=set_show_planets>
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=planets_tree_view().into_tree_item() depth=2/>
            </ul>
          </Window>
        </Show>
        <Show when=show_buildings>
          <Window title="Buildings" on_show=set_show_buildings>
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=buildings_by_planets().into_tree_item() />
            </ul>
          </Window>
        </Show>
        <Show when=show_ships>
          <Window title="Ships" on_show=set_show_ships>
            <ul class="text-white flex space-x-4">
                <TreeRow tree_item=ships_by_planets().into_tree_item() />
            </ul>
          </Window>
        </Show>
        <aside class="hidden w-12 overflow-y-auto bg-white dark:bg-gray-800 md:block flex-shrink-0">
          <nav class="w-12 h-screen space-y-4 px-2 shadow">
            <div class="flex items-center justify-center card w-full bg-base-100 shadow py-4 text-white">
              <ul class="space-y-4">
                // Planets
                <li on:click=move |_| set_show_planets(!show_planets()) class="w-6 h-6">
                  <svg version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 502.74 502.74" xml:space="preserve" fill="#000000" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #000000;"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <path style="fill: none; --darkreader-inline-fill: none;" d="M395.431,251.37c0,0.962-0.018,1.919-0.036,2.876C395.418,253.291,395.431,252.332,395.431,251.37z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(182, 128, 87); --darkreader-inline-fill: #bc8b66;" d="M434.563,175.558c-0.695-1.68-5.984-4.57-20.336-4.57c-8.906,0-21.297,1.109-38.25,4.313 c-38.578,7.281-87.578,22.789-137.969,43.664c-50.391,20.875-96.008,44.555-128.43,66.688 c-37.219,25.398-42.375,38.883-41.25,41.609c1.733,4.199,19.547,7.848,60.63-0.13c-1.195-1.927-2.345-3.885-3.449-5.871 c-0.039-0.07-0.079-0.141-0.118-0.211c-0.462-0.834-0.915-1.674-1.361-2.518c-0.077-0.146-0.154-0.292-0.231-0.438 c-0.417-0.795-0.826-1.595-1.228-2.399c-0.098-0.197-0.196-0.394-0.294-0.591c-0.382-0.772-0.758-1.547-1.126-2.327 c-0.109-0.231-0.217-0.464-0.325-0.696c-0.353-0.758-0.701-1.517-1.041-2.282c-0.114-0.257-0.225-0.515-0.338-0.772 c-0.329-0.753-0.656-1.507-0.973-2.267c-0.003-0.006-0.005-0.012-0.008-0.018c5.518-10.29,33.918-28.665,74.006-48.367 c15.792-7.762,33.396-15.729,52.128-23.488c66.279-27.454,124.41-43.083,139.996-38.382c2.892,7.013,5.251,14.301,7.016,21.819 C426.293,194.911,436.297,179.738,434.563,175.558z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M279.66,110.144c0.924,0.184,1.844,0.379,2.761,0.58 C281.504,110.523,280.584,110.328,279.66,110.144z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M286.315,111.634c1.041,0.26,2.078,0.53,3.111,0.813 C288.394,112.165,287.357,111.894,286.315,111.634z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M283.012,110.853c0.98,0.22,1.957,0.45,2.929,0.69C284.968,111.304,283.992,111.072,283.012,110.853 z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M395.394,254.246c0.019-0.957,0.036-1.915,0.036-2.876c0-0.786-0.022-1.565-0.035-2.348 c-0.012,0.008-0.023,0.015-0.035,0.022c-0.051-3.104-0.212-6.178-0.464-9.227c0.015-0.01,0.029-0.019,0.044-0.029 c-0.604-7.305-1.758-14.452-3.418-21.405c0.031-0.021,0.061-0.042,0.092-0.063c-1.766-7.518-4.124-14.807-7.016-21.819 c-15.586-4.7-73.717,10.929-139.996,38.382c-18.732,7.759-36.336,15.726-52.128,23.488c-2.248,10.108-3.443,20.613-3.443,31.398 c0,41.168,17.304,78.272,45.001,104.516c-25.063-3.021-48.141-12.482-67.54-26.67c-0.012,0.003-0.024,0.006-0.036,0.009 c23.819,17.44,53.191,27.745,84.975,27.745c72.008,0,131.67-52.855,142.31-121.885c-0.067,0.041-0.133,0.082-0.201,0.122 C394.526,267.263,395.231,260.825,395.394,254.246z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M336.357,135.08c-13.912-10.177-29.717-17.917-46.79-22.595 c17.057,4.682,32.842,12.433,46.744,22.606C336.326,135.087,336.341,135.083,336.357,135.08z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M272.829,108.955c0.812,0.121,1.619,0.254,2.427,0.388 C274.448,109.209,273.641,109.076,272.829,108.955z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M269.35,108.479c0.761,0.094,1.519,0.201,2.276,0.308 C270.869,108.68,270.112,108.573,269.35,108.479z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M265.837,108.085c0.706,0.07,1.407,0.153,2.11,0.233 C267.244,108.238,266.542,108.155,265.837,108.085z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M262.283,107.775c0.646,0.048,1.289,0.109,1.934,0.166 C263.572,107.885,262.929,107.824,262.283,107.775z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M255.08,107.416c0.45,0.011,0.898,0.034,1.347,0.05C255.978,107.45,255.53,107.427,255.08,107.416z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M258.699,107.552c0.573,0.029,1.142,0.07,1.713,0.105 C259.841,107.622,259.271,107.581,258.699,107.552z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M276.269,109.511c0.867,0.151,1.73,0.313,2.592,0.479 C277.999,109.825,277.136,109.662,276.269,109.511z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M128.965,327.129c-1.196-1.927-2.35-3.883-3.456-5.87c1.105,1.987,2.255,3.944,3.449,5.871 C128.961,327.13,128.963,327.13,128.965,327.129z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M119.786,309.797c0.34,0.765,0.688,1.524,1.041,2.282 C120.474,311.322,120.126,310.562,119.786,309.797z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M122.572,315.693c0.402,0.804,0.811,1.603,1.228,2.399 C123.383,317.297,122.974,316.497,122.572,315.693z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M124.03,318.53c0.446,0.844,0.899,1.684,1.361,2.518C124.929,320.214,124.477,319.374,124.03,318.53 z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M121.152,312.775c0.368,0.78,0.744,1.555,1.126,2.327 C121.896,314.33,121.521,313.554,121.152,312.775z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(255, 208, 66); --darkreader-inline-fill: #ffd148;" d="M118.476,306.758c0.317,0.76,0.643,1.514,0.973,2.267 C119.119,308.272,118.793,307.517,118.476,306.758z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(219, 178, 70); --darkreader-inline-fill: #ddb753;" d="M189.031,289.77c0-10.785,1.195-21.29,3.443-31.398c-40.088,19.702-68.488,38.078-74.006,48.367 c0.003,0.006,0.005,0.012,0.008,0.018c0.317,0.76,0.643,1.514,0.973,2.267c0.113,0.257,0.224,0.516,0.338,0.772 c0.34,0.765,0.688,1.525,1.041,2.282c0.108,0.232,0.215,0.464,0.325,0.696c0.368,0.78,0.744,1.555,1.126,2.327 c0.098,0.197,0.196,0.394,0.294,0.591c0.402,0.804,0.811,1.604,1.228,2.399c0.076,0.146,0.154,0.292,0.231,0.438 c0.446,0.844,0.899,1.684,1.361,2.518c0.039,0.071,0.079,0.141,0.118,0.211c1.106,1.987,2.26,3.943,3.456,5.87 c0.052-0.01,0.101-0.019,0.153-0.029c3.776,6.088,7.995,11.867,12.604,17.303c0.059-0.012,0.117-0.024,0.177-0.037 c1.97,2.32,3.965,4.608,6.075,6.797c-0.04,0.009-0.078,0.017-0.118,0.025c5.762,5.985,12.032,11.477,18.751,16.399 c-0.039,0.01-0.076,0.018-0.115,0.028c19.399,14.188,42.477,23.649,67.54,26.67C206.335,368.042,189.031,330.938,189.031,289.77z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(219, 178, 70); --darkreader-inline-fill: #ddb753;" d="M336.311,135.091c-13.902-10.173-29.687-17.924-46.744-22.606c-0.047-0.013-0.094-0.025-0.141-0.038 c-1.032-0.282-2.069-0.553-3.111-0.813c-0.125-0.031-0.25-0.061-0.374-0.091c-0.973-0.24-1.949-0.47-2.929-0.69 c-0.197-0.044-0.394-0.085-0.591-0.128c-0.917-0.202-1.837-0.396-2.761-0.58c-0.266-0.053-0.533-0.102-0.799-0.153 c-0.862-0.167-1.725-0.329-2.592-0.479c-0.337-0.059-0.676-0.112-1.013-0.168c-0.807-0.135-1.615-0.267-2.427-0.388 c-0.4-0.06-0.802-0.112-1.202-0.168c-0.757-0.106-1.515-0.213-2.276-0.308c-0.467-0.058-0.936-0.107-1.404-0.161 c-0.703-0.08-1.404-0.163-2.11-0.233c-0.539-0.054-1.08-0.096-1.62-0.144c-0.644-0.057-1.287-0.118-1.934-0.166 c-0.622-0.046-1.247-0.08-1.871-0.118c-0.571-0.035-1.14-0.076-1.713-0.105c-0.755-0.038-1.514-0.06-2.272-0.086 c-0.449-0.015-0.897-0.038-1.347-0.05c-1.213-0.03-2.429-0.046-3.649-0.046c-72.03,0-131.706,52.886-142.32,121.946 c33.467-20.203,71.687-39.286,110.289-55.274C259.584,157.396,299.477,144.157,336.311,135.091z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(212, 158, 108); --darkreader-inline-fill: #d5a171;" d="M500.446,148.229v-0.004c-9.359-22.59-43.781-31.23-99.531-25.031 c-19.884,2.215-41.629,6.243-64.558,11.885c-0.015,0.004-0.031,0.008-0.046,0.011c-36.833,9.066-76.726,22.305-116.911,38.951 c-38.602,15.988-76.822,35.071-110.289,55.274C37.917,272.293-11.737,320.347,2.415,354.518 c7.398,17.844,31.414,26.914,68.148,26.914c26.028,0,58.493-4.599,95.893-13.807c0.012-0.003,0.024-0.006,0.036-0.009 c0.039-0.01,0.076-0.018,0.115-0.028c-6.719-4.922-12.989-10.414-18.751-16.399c-27.116,5.97-51.087,9.111-70.179,9.111 c-25.144,0-41.901-5.37-46.548-16.627c-13.104-31.747,75.128-98.287,197.071-148.621c78.701-32.485,152.217-50.281,197.977-50.283 c0.003,0,0.009,0,0.009,0v0v0c25.139,0,41.893,5.371,46.539,16.627c8.173,19.799-23.077,53.132-77.33,87.626 c0.013,0.782,0.035,1.562,0.035,2.348c0,0.962-0.013,1.921-0.036,2.876c-0.163,6.579-0.868,13.017-1.854,19.361 c0.068-0.041,0.133-0.082,0.201-0.122C473.159,225.421,513.054,178.686,500.446,148.229z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(212, 158, 108); --darkreader-inline-fill: #d5a171;" d="M231.253,202.447C99.035,257.022,29.891,319.706,38.524,340.621 c3.063,7.422,17.334,11.68,39.153,11.68c17.928,0,39.741-2.764,64.045-7.897c-4.61-5.436-8.828-11.216-12.604-17.303 c-0.052,0.01-0.101,0.019-0.153,0.029c-0.002,0-0.004,0.001-0.006,0.001c-41.083,7.978-58.897,4.329-60.63,0.13 c-1.125-2.727,4.031-16.211,41.25-41.609c32.422-22.133,78.039-45.813,128.43-66.688c50.391-20.875,99.39-36.383,137.969-43.664 c16.953-3.203,29.344-4.313,38.25-4.313c14.352,0,19.641,2.891,20.336,4.57c1.733,4.18-8.271,19.353-42.949,42.764 c-0.031,0.021-0.061,0.042-0.092,0.063c1.66,6.953,2.814,14.1,3.418,21.405c51.099-33.256,75.75-62.353,70.39-75.34 c-3.063-7.422-17.331-11.68-39.144-11.68v-8l-0.008,8C379.478,152.771,306.609,171.343,231.253,202.447z" data-darkreader-inline-fill=""></path> <path style="fill: rgb(182, 128, 87); --darkreader-inline-fill: #bc8b66;" d="M141.899,344.367c-0.059,0.013-0.117,0.024-0.177,0.037c-24.305,5.134-46.117,7.897-64.045,7.897 c-21.818,0-36.089-4.257-39.153-11.68c-8.633-20.915,60.511-83.599,192.729-138.174c75.356-31.104,148.225-49.676,194.925-49.678 l0.008-8c0,0-0.006,0-0.009,0c-45.76,0.002-119.276,17.798-197.977,50.283C106.257,245.387,18.025,311.927,31.129,343.674 c4.647,11.258,21.404,16.627,46.548,16.627c19.092,0,43.063-3.141,70.179-9.111c0.04-0.009,0.078-0.017,0.118-0.025 C145.864,348.975,143.869,346.687,141.899,344.367z" data-darkreader-inline-fill=""></path> <line style="fill: none; stroke: rgb(182, 128, 87); stroke-width: 0; stroke-linecap: round; stroke-linejoin: round; --darkreader-inline-fill: none; --darkreader-inline-stroke: #bc8b66;" x1="426.186" y1="144.77" x2="426.186" y2="144.769" data-darkreader-inline-fill="" data-darkreader-inline-stroke=""></line> <path style="fill: rgb(182, 128, 87); --darkreader-inline-fill: #bc8b66;" d="M465.33,164.449c5.361,12.987-19.291,42.084-70.39,75.34c-0.015,0.01-0.029,0.019-0.044,0.029 c0.252,3.049,0.413,6.124,0.464,9.227c0.012-0.007,0.023-0.015,0.035-0.022c54.253-34.494,85.502-67.827,77.33-87.626 c-4.646-11.256-21.4-16.627-46.539-16.627v8C447.999,152.769,462.267,157.026,465.33,164.449z" data-darkreader-inline-fill=""></path> </g> </g></svg>
                </li>
                // Buildings
                <li on:click=move |_| set_show_buildings(!show_buildings()) class="w-6 h-6">
                    <svg version="1.0" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 64 64" enable-background="new 0 0 64 64" xml:space="preserve" fill="#000000" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #000000;"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <g> <path fill="#B4CCB9" d="M2,15v46c0,1.104,0.896,2,2,2h12V13H4C2.896,13,2,13.896,2,15z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #334b3d;"></path> <path fill="#B4CCB9" d="M44,3H20c-1.104,0-2,0.896-2,2v58l0.001,0.002H27V54c0-0.553,0.447-1,1-1h8c0.553,0,1,0.447,1,1v9.002 h8.999L46,63V5C46,3.896,45.104,3,44,3z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #334b3d;"></path> <path fill="#B4CCB9" d="M60,23H48v40h12c1.104,0,2-0.896,2-2V25C62,23.896,61.104,23,60,23z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #334b3d;"></path> </g> <path fill="#394240" d="M60,21H48V5c0-2.211-1.789-4-4-4H20c-2.211,0-4,1.789-4,4v6H4c-2.211,0-4,1.789-4,4v46c0,2.211,1.789,4,4,4 h56c2.211,0,4-1.789,4-4V25C64,22.789,62.211,21,60,21z M16,63H4c-1.104,0-2-0.896-2-2V15c0-1.104,0.896-2,2-2h12V63z M35,63.002 h-6V55h6V63.002z M46,63l-0.001,0.002H37V54c0-0.553-0.447-1-1-1h-8c-0.553,0-1,0.447-1,1v9.002h-8.999L18,63V5 c0-1.104,0.896-2,2-2h24c1.104,0,2,0.896,2,2V63z M62,61c0,1.104-0.896,2-2,2H48V23h12c1.104,0,2,0.896,2,2V61z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #2e3234;"></path> <path fill="#394240" d="M7,25h4c0.553,0,1-0.447,1-1v-4c0-0.553-0.447-1-1-1H7c-0.553,0-1,0.447-1,1v4C6,24.553,6.447,25,7,25z M8,21h2v2H8V21z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M7,35h4c0.553,0,1-0.447,1-1v-4c0-0.553-0.447-1-1-1H7c-0.553,0-1,0.447-1,1v4C6,34.553,6.447,35,7,35z M8,31h2v2H8V31z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M7,45h4c0.553,0,1-0.447,1-1v-4c0-0.553-0.447-1-1-1H7c-0.553,0-1,0.447-1,1v4C6,44.553,6.447,45,7,45z M8,41h2v2H8V41z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M29,19h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C30,19.447,29.553,19,29,19 z M28,23h-2v-2h2V23z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M29,29h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C30,29.447,29.553,29,29,29 z M28,33h-2v-2h2V33z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M29,39h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C30,39.447,29.553,39,29,39 z M28,43h-2v-2h2V43z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M39,19h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C40,19.447,39.553,19,39,19 z M38,23h-2v-2h2V23z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M29,9h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C30,9.447,29.553,9,29,9z M28,13h-2v-2h2V13z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M39,9h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C40,9.447,39.553,9,39,9z M38,13h-2v-2h2V13z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M39,29h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C40,29.447,39.553,29,39,29 z M38,33h-2v-2h2V33z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M39,39h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C40,39.447,39.553,39,39,39 z M38,43h-2v-2h2V43z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M57,29h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C58,29.447,57.553,29,57,29 z M56,33h-2v-2h2V33z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M57,39h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C58,39.447,57.553,39,57,39 z M56,43h-2v-2h2V43z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M57,49h-4c-0.553,0-1,0.447-1,1v4c0,0.553,0.447,1,1,1h4c0.553,0,1-0.447,1-1v-4C58,49.447,57.553,49,57,49 z M56,53h-2v-2h2V53z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <path fill="#394240" d="M7,55h4c0.553,0,1-0.447,1-1v-4c0-0.553-0.447-1-1-1H7c-0.553,0-1,0.447-1,1v4C6,54.553,6.447,55,7,55z M8,51h2v2H8V51z" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #c1bcb4;"></path> <g opacity="0.15"> <path d="M2,15v46c0,1.104,0.896,2,2,2h12V13H4C2.896,13,2,13.896,2,15z"></path> <path d="M60,23H48v40h12c1.104,0,2-0.896,2-2V25C62,23.896,61.104,23,60,23z"></path> </g> <rect x="29" y="55" fill="#F76D57" width="6" height="8.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f76e59;"></rect> <g> <rect x="8" y="21" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="8" y="31" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="8" y="41" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="8" y="51" fill="#506C7F" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #a79f93;"></rect> <rect x="26" y="11" fill="#506C7F" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #a79f93;"></rect> <rect x="26" y="21" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="26" y="31" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="26" y="41" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="36" y="11" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="36" y="21" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="36" y="31" fill="#506C7F" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #a79f93;"></rect> <rect x="36" y="41" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="54" y="31" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> <rect x="54" y="41" fill="#506C7F" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #a79f93;"></rect> <rect x="54" y="51" fill="#F9EBB2" width="2.001" height="2.002" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #f7e59a;"></rect> </g> </g> </g></svg>
                </li>
                // Ships
                <li on:click=move |_| set_show_ships(!show_ships()) class="w-6 h-6">
                    <svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512.052 512.052" xml:space="preserve" fill="#000000" data-darkreader-inline-fill="" style="--darkreader-inline-fill: #000000;"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g transform="translate(0 -1)"> <path style="fill: rgb(232, 237, 238); --darkreader-inline-fill: #dbd8d4;" d="M290.365,29.865C274.484-1.584,256.026,1.054,256.026,1.054s-18.458-2.638-34.339,28.811 C210.31,52.393,27.622,337.872,6.378,371.066c-1.328,2.074-2.013,4.452-2.013,6.916v67.15c0,8.183,7.515,14.301,15.525,12.644 l88.611-18.337V306.25c0-14.093,10.561-26.728,24.637-27.466c15.013-0.79,27.431,11.151,27.431,25.999V428.67l45.125-9.337 l15.62-305.464l17.356-17.356h34.712l17.356,17.356l15.62,305.464l45.125,9.337V306.25c0-14.093,10.561-26.728,24.637-27.466 c15.013-0.79,27.431,11.151,27.431,25.999v134.656l88.611,18.337c8.01,1.658,15.525-4.46,15.525-12.644v-67.15 c0-2.465-0.677-4.842-2.013-6.916C484.43,337.872,301.741,52.393,290.365,29.865" data-darkreader-inline-fill=""></path> <path style="fill: rgb(176, 182, 187); --darkreader-inline-fill: #bab4ab;" d="M108.501,304.781v147.525v8.678c0,4.79,3.888,8.678,8.678,8.678h34.712 c4.79,0,8.678-3.888,8.678-8.678v-8.678V304.781c0-14.379-11.655-26.034-26.034-26.034l0,0 C120.156,278.747,108.501,290.401,108.501,304.781" data-darkreader-inline-fill=""></path> <polygon style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" points="108.501,339.493 160.569,339.493 160.569,322.137 108.501,322.137 " data-darkreader-inline-fill=""></polygon> <path style="fill: rgb(243, 213, 91); --darkreader-inline-fill: #f3d55d;" d="M134.535,513.052c-4.79,0-8.678-3.888-8.678-8.678v-34.712c0-4.79,3.888-8.678,8.678-8.678 s8.678,3.888,8.678,8.678v34.712C143.213,509.164,139.326,513.052,134.535,513.052" data-darkreader-inline-fill=""></path> <g> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M143.213,365.526c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678 s3.888-8.678,8.678-8.678S143.213,360.736,143.213,365.526" data-darkreader-inline-fill=""></path> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M143.213,400.238c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678 s3.888-8.678,8.678-8.678S143.213,395.448,143.213,400.238" data-darkreader-inline-fill=""></path> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M143.213,434.95c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678s3.888-8.678,8.678-8.678 S143.213,430.16,143.213,434.95" data-darkreader-inline-fill=""></path> </g> <g> <path d="M264.705,244.035c0,4.79-3.888,8.678-8.678,8.678c-4.79,0-8.678-3.888-8.678-8.678s3.888-8.678,8.678-8.678 C260.817,235.357,264.705,239.245,264.705,244.035"></path> <path d="M264.705,278.747c0,4.79-3.888,8.678-8.678,8.678c-4.79,0-8.678-3.888-8.678-8.678s3.888-8.678,8.678-8.678 C260.817,270.069,264.705,273.957,264.705,278.747"></path> <path d="M264.705,313.459c0,4.79-3.888,8.678-8.678,8.678c-4.79,0-8.678-3.888-8.678-8.678s3.888-8.678,8.678-8.678 C260.817,304.781,264.705,308.668,264.705,313.459"></path> </g> <path style="fill: rgb(176, 182, 187); --darkreader-inline-fill: #bab4ab;" d="M351.484,304.781v147.525v8.678c0,4.79,3.888,8.678,8.678,8.678h34.712 c4.79,0,8.678-3.888,8.678-8.678v-8.678V304.781c0-14.379-11.655-26.034-26.034-26.034l0,0 C363.139,278.747,351.484,290.401,351.484,304.781" data-darkreader-inline-fill=""></path> <polygon style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" points="351.484,339.493 403.552,339.493 403.552,322.137 351.484,322.137 " data-darkreader-inline-fill=""></polygon> <path style="fill: rgb(243, 213, 91); --darkreader-inline-fill: #f3d55d;" d="M377.518,513.052c-4.79,0-8.678-3.888-8.678-8.678v-34.712c0-4.79,3.888-8.678,8.678-8.678 s8.678,3.888,8.678,8.678v34.712C386.196,509.164,382.309,513.052,377.518,513.052" data-darkreader-inline-fill=""></path> <g> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M386.196,365.526c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678 s3.888-8.678,8.678-8.678S386.196,360.736,386.196,365.526" data-darkreader-inline-fill=""></path> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M386.196,400.238c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678 s3.888-8.678,8.678-8.678S386.196,395.448,386.196,400.238" data-darkreader-inline-fill=""></path> <path style="fill: rgb(131, 138, 142); --darkreader-inline-fill: #9d9588;" d="M386.196,434.95c0,4.79-3.888,8.678-8.678,8.678s-8.678-3.888-8.678-8.678s3.888-8.678,8.678-8.678 S386.196,430.16,386.196,434.95" data-darkreader-inline-fill=""></path> </g> <path style="fill: rgb(176, 182, 187); --darkreader-inline-fill: #bab4ab;" d="M293.149,452.306h-74.24c-8.131,0-14.588-6.812-14.162-14.926l16.566-323.515l17.356-17.356h34.712 l17.356,17.356l16.575,323.515C307.737,445.494,301.272,452.306,293.149,452.306" data-darkreader-inline-fill=""></path> <path style="fill: rgb(243, 213, 91); --darkreader-inline-fill: #f3d55d;" d="M256.027,513.052c-4.79,0-8.678-3.888-8.678-8.678V374.204c0-4.79,3.888-8.678,8.678-8.678 c4.79,0,8.678,3.888,8.678,8.678v130.169C264.705,509.164,260.817,513.052,256.027,513.052" data-darkreader-inline-fill=""></path> <polygon style="fill: rgb(134, 151, 203); --darkreader-inline-fill: #87a6cb;" points="293.85,174.611 290.734,113.865 273.378,96.509 238.667,96.509 221.311,113.865 218.204,174.611 " data-darkreader-inline-fill=""></polygon> <polygon style="fill: rgb(67, 76, 109); --darkreader-inline-fill: #b0aa9f;" points="247.349,174.611 264.705,174.611 264.705,96.509 247.349,96.509 " data-darkreader-inline-fill=""></polygon> </g> </g></svg>
                </li>
              </ul>
            </div>
          </nav>
        </aside>
    }
}
