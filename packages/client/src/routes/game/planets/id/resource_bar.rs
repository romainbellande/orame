use crate::components::tooltip::{TooltipContent, TooltipProvider};
use leptos::*;
use ogame_core::{building_type::BuildingType, planet::Planet, resources::ResourceType};
use uuid::Uuid;

#[derive(Clone)]
struct Resource {
    name: &'static str,
    class: &'static str,
    description: &'static str,
    amount: f64,
    producers: Vec<BuildingType>,
    resource_type: ResourceType,
}

impl From<ResourceType> for Resource {
    fn from(value: ResourceType) -> Self {
        match value {
          ResourceType::Metal => Self {
            name: "Metal",
            class: "metal",
            description: "Metal is the primary resource used in the foundation of your Empire. At greater depths, the mines can produce more output of metal.You can use the available metal for use in the construction of buildings, ships, defense systems, and research. As the mines drill deeper, more energy is required for maximum production. As metal is the most abundant of all resources available, its value is considered to be the lowest of all resources for trading.",
            amount: 0.0,
            producers: vec![BuildingType::Metal],
            resource_type: ResourceType::Metal
          },
          ResourceType::Crystal => Self {
            name: "Crystal",
            class: "crystal",
            description: "Crystal is one of the 3 main resources in the game. It is advisable to upgrade the crystal mine daily, because a constant supply of crystal is required for research and energy development, being required in the most quantities for researches. It is used to create electronic circuits and form alloys.",
            amount: 0.0,
            producers: vec![BuildingType::Crystal],
            resource_type: ResourceType::Crystal
          },
          ResourceType::Deuterium => Self {
            name: "Deuterium",
            class: "deuterium",
            description: "Deuterium is a stable isotope of hydrogen with a natural abundance in the oceans of colonies of approximately one atom in 6500 of hydrogen (~154 PPM). Deuterium thus accounts for approximately 0.015% (on a weight basis, 0.030%) of all water. Deuterium is processed by special synthesizers which can separate the water from the Deuterium using specially designed centrifuges. The upgrade of the synthesizer allows for increasing the amount of Deuterium deposits processed. Deuterium is used when carrying out sensor phalanx scans, viewing galaxies, as fuel for ships, and performing specialized research upgrades.",
            amount: 0.0,
            producers: vec![BuildingType::Deuterium],
            resource_type: ResourceType::Deuterium
          },
      }
    }
}

impl Resource {
    pub fn set_amount(&mut self, amount: f64) -> Self {
        self.amount = amount;
        self.clone()
    }

    pub fn produced(&self, planet: Planet, ticks: usize) -> f64 {
        planet.get_produced_resource(self.resource_type.clone(), ticks)
    }
}

#[component]
fn ResourceTile(resource: ReadSignal<Resource>, planet: Signal<Planet>) -> impl IntoView {
    view! {
      <TooltipProvider>
        <li class="space-y-2 flex flex-col justify-center ">
          <div class=format!("resourceIcon {}", resource().class)></div>
          <span class="text-xs text-center text-slate-300">{ resource().amount.floor() }</span>
        </li>
        <TooltipContent>
          <ul>
            <li class="space-x-8 space-y-4">
              <span>"amount"</span>
              <span>{resource().amount.floor()}</span>
            </li>
            <li class="space-x-8 space-y-4">
              <span>"produced"</span>
              <span>{resource().produced(planet(), 3600).floor()}"/h"</span>
            </li>
          </ul>
        </TooltipContent>
      </TooltipProvider>
    }
}

#[component]
pub fn ResourceBar(planet: Signal<Planet>) -> impl IntoView {
    let resources = move || {
        vec![
            Resource::from(ResourceType::Metal)
                .set_amount(planet.get().resources.get(ResourceType::Metal)),
            Resource::from(ResourceType::Crystal)
                .set_amount(planet.get().resources.get(ResourceType::Crystal)),
            Resource::from(ResourceType::Deuterium)
                .set_amount(planet.get().resources.get(ResourceType::Deuterium)),
        ]
        .into_iter()
        .map(|resource| (Uuid::new_v4(), create_signal(resource)))
        .collect::<Vec<_>>()
    };

    view! {
      <ul class="resourcesbarcomponent flex space-x-4">
        <For
          each=resources
          key=|resource| resource.0
          children=move |(_id, (resource, _))| {
            view! {
              <ResourceTile resource=resource planet=planet />
            }
          }
        />
      </ul>
    }
}
