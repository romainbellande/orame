use ogame_core::building_type::BuildingType;

#[derive(Clone)]
pub struct BuildingConfig {
    pub name: &'static str,
    pub class: &'static str,
    pub description: &'static str,
    pub building_type: BuildingType,
}

impl From<BuildingType> for BuildingConfig {
    fn from(value: BuildingType) -> Self {
        match value {
        BuildingType::Metal => Self {
          name: "Metal Mine",
          class: "metalMine",
          description: "Metal is the primary resource used in the foundation of your Empire. At greater depths, the mines can produce more output of metal.You can use the available metal for use in the construction of buildings, ships, defense systems, and research. As the mines drill deeper, more energy is required for maximum production. As metal is the most abundant of all resources available, its value is considered to be the lowest of all resources for trading.",
          building_type: BuildingType::Metal,
        },
        BuildingType::Crystal => Self {
          name: "Crystal Mine",
          class: "crystalMine",
          description: "Crystal mines supply the main resource used to produce electronic circuits and form certain alloy compounds. Mining crystal consumes some one and half times more energy than a mining metal, making crystal more valuable. Almost all ships and all buildings require crystal. Most crystals required to build spaceships, however, are very rare, and like metal can only be found at a certain depth. Therefore, building mines in deeper strata will increase the amount of crystal produced.",
          building_type: BuildingType::Crystal,
        },
        BuildingType::Deuterium => Self {
          name: "Deuterium Synthesizer",
          class: "deuteriumSynthesizer",
          description: "Deuterium is a stable isotope of hydrogen with a natural abundance in the oceans of colonies of approximately one atom in 6500 of hydrogen (~154 PPM). Deuterium thus accounts for approximately 0.015% (on a weight basis, 0.030%) of all water. Deuterium is processed by special synthesizers which can separate the water from the Deuterium using specially designed centrifuges. The upgrade of the synthesizer allows for increasing the amount of Deuterium deposits processed. Deuterium is used when carrying out sensor phalanx scans, viewing galaxies, as fuel for ships, and performing specialized research upgrades.",
          building_type: BuildingType::Deuterium
        },
        BuildingType::Shipyard => Self {
          name: "Shipyard",
          class: "shipyard",
          description: "The shipyard is the center of ship construction. The higher the level of the shipyard, the faster the ships are built. The shipyard also allows you to build new types of ships. The shipyard is also required to repair damaged ships. The shipyard is also required to build and repair defense systems.",
          building_type: BuildingType::Shipyard
        }
      }
    }
}
