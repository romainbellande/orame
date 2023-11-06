use leptos::*;
use ogame_core::planet::Planet;
use uuid::Uuid;

enum ResourceItem {
    Metal,
    Crystal,
    Deuterium,
}

#[derive(Clone)]
struct ResourceConfig {
    name: &'static str,
    class: &'static str,
    description: &'static str,
    amount: usize,
}

impl From<ResourceItem> for ResourceConfig {
    fn from(value: ResourceItem) -> Self {
        match value {
          ResourceItem::Metal => Self {
            name: "Metal",
            class: "metal",
            description: "Metal is the primary resource used in the foundation of your Empire. At greater depths, the mines can produce more output of metal.You can use the available metal for use in the construction of buildings, ships, defense systems, and research. As the mines drill deeper, more energy is required for maximum production. As metal is the most abundant of all resources available, its value is considered to be the lowest of all resources for trading.",
            amount: 0,
          },
          ResourceItem::Crystal => Self {
            name: "Crystal",
            class: "crystal",
            description: "Crystal is one of the 3 main resources in the game. It is advisable to upgrade the crystal mine daily, because a constant supply of crystal is required for research and energy development, being required in the most quantities for researches. It is used to create electronic circuits and form alloys.",
            amount: 0,
          },
          ResourceItem::Deuterium => Self {
            name: "Deuterium",
            class: "deuterium",
            description: "Deuterium is a stable isotope of hydrogen with a natural abundance in the oceans of colonies of approximately one atom in 6500 of hydrogen (~154 PPM). Deuterium thus accounts for approximately 0.015% (on a weight basis, 0.030%) of all water. Deuterium is processed by special synthesizers which can separate the water from the Deuterium using specially designed centrifuges. The upgrade of the synthesizer allows for increasing the amount of Deuterium deposits processed. Deuterium is used when carrying out sensor phalanx scans, viewing galaxies, as fuel for ships, and performing specialized research upgrades.",
            amount: 0,
          },
      }
    }
}

impl ResourceConfig {
    pub fn set_amount(&mut self, amount: usize) -> Self {
        self.amount = amount;
        self.clone()
    }
}

#[component]
fn ResourceTile(resource: ReadSignal<ResourceConfig>) -> impl IntoView {
    view! {
      <li class="space-y-2 flex flex-col justify-center">
        <div class=format!("resourceIcon {}", resource().class)></div>
        <span class="text-xs text-center text-slate-300">{ resource().amount }</span>
      </li>
    }
}

#[component]
pub fn ResourceBar(planet: Signal<Planet>) -> impl IntoView {
    let initial_resources = vec![
        ResourceConfig::from(ResourceItem::Metal).set_amount(planet().resources().metal),
        ResourceConfig::from(ResourceItem::Crystal).set_amount(planet().resources().crystal),
        ResourceConfig::from(ResourceItem::Deuterium).set_amount(planet().resources().deuterium),
    ]
    .into_iter()
    .map(|resource| (Uuid::new_v4(), create_signal(resource)))
    .collect::<Vec<_>>();

    let (resources, _) = create_signal(initial_resources);

    view! {
      <ul class="resourcesbarcomponent flex space-x-4">
        <For
          each=resources
          key=|resource| resource.0
          children=move |(id, (resource, _))| {
            view! {
              <ResourceTile resource=resource />
            }
          }
        />
      </ul>
    }
}
