use ogame_core::ship_type::ShipType;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Ship {
    pub id: Uuid,
    pub ship_type: ShipType,
    pub name: String,
    pub class: String,
    pub description: String,
    pub amount: usize,
}

impl Ship {
    pub fn set_amount(&mut self, amount: usize) -> Self {
        self.amount = amount;
        self.clone()
    }

    pub fn get_type(&self) -> ShipType {
        self.ship_type.clone()
    }
}

impl From<ShipType> for Ship {
    fn from(ship_type: ShipType) -> Self {
        match ship_type {
      ShipType::Battlecruiser => Self {
        ship_type,
        name: "Battlecruiser".to_string(),
        class: "interceptor".to_string(),
        description: "This ship is deadly when it comes to destroying fleets. With its improved laser cannons, it holds a privileged position among the heavy ships, capable of taking down several of them in no time.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Bomber => Self {
        ship_type,
        name: "Bomber".to_string(),
        class: "bomber".to_string(),
        description: "The bomber is a special purpose spacecraft developed to break through heavy planetary defense.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Battleship => Self {
        ship_type,
        name: "Battleship".to_string(),
        class: "battleship".to_string(),
        description: "Battleships provide the backbone of any military fleet. Heavy armour, strong weapons systems and high cruising speed, as well as a large cargo bay, make this ship a tough opponent to fight against.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::ColonyShip => Self {
        ship_type,
        name: "Colony Ship".to_string(),
        class: "colonyShip".to_string(),
        description: "This ship provides the means necessary to go where no man has gone before and to colonize new worlds.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Cruiser => Self {
        ship_type,
        name: "Cruiser".to_string(),
        class: "cruiser".to_string(),
        description: "Combat cruisers have armor almost three times as strong as heavy fighters and support more than twice their firing power. Their traveling speed is also amongst the fastest seen.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Deathstar => Self {
        ship_type,
        name: "Deathstar".to_string(),
        class: "deathstar".to_string(),
        description: "The Deathstar is the most powerful ship in the universe. It is the ultimate weapon of destruction. It is so powerful that it can destroy entire planets. It is also the slowest ship in the universe.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Destroyer => Self {
        ship_type,
        name: "Destroyer".to_string(),
        class: "destroyer".to_string(),
        description: "The destroyer is a heavily armed ship that can be used for either attack or defense. It is equipped with a powerful shield and a strong hull. It is also capable of carrying a large cargo.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::EspionageProbe => Self {
        ship_type,
        name: "Espionage Probe".to_string(),
        class: "espionageProbe".to_string(),
        description: "The espionage probe is a small, fast ship that is used to spy on enemy planets. It is equipped with a powerful sensor that can scan enemy planets and fleets. It is also equipped with a cloaking device that makes it invisible to enemy radar.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::HeavyFighter => Self {
        ship_type,
        name: "Heavy Fighter".to_string(),
        class: "fighterHeavy".to_string(),
        description: "The heavy fighter is a straight evolution of the light fighter offering increased shielding and firing power.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::LargeCargo => Self {
        ship_type,
        name: "Large Cargo".to_string(),
        class: "transporterLarge".to_string(),
        description: "The large cargo ship is a large, slow ship that is used to transport large amounts of resources. It is equipped with a powerful engine that allows it to travel at high speeds.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::LightFighter => Self {
        ship_type,
        name: "Light Fighter".to_string(),
        class: "fighterLight".to_string(),
        description: "The light fighter is a maneuverable ship you can find on nearly every planet. The costs are not particularly high and its shield power and cargo capacity are very low. Used for moonshots in most universes.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::Recycler => Self {
        ship_type,
        name: "Recycler".to_string(),
        class: "recycler".to_string(),
        description: "The recycler is a ship that is used to collect debris from destroyed ships and moons. It is equipped with a powerful tractor beam that can pull in debris from a distance.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::SmallCargo => Self {
        ship_type,
        name: "Small Cargo".to_string(),
        class: "transporterSmall".to_string(),
        description: "Small cargo ships are very agile ships used to transport resources from one planet to another. Useful for farming. Switches to Impulse Drive at level 5, doubling its speed.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      },
      ShipType::SolarSatellite => Self {
        ship_type,
        name: "Solar Satellite".to_string(),
        class: "solarSatellite".to_string(),
        description: "The solar satellite is a small, unmanned satellite that is used to collect solar energy. It is equipped with a powerful solar panel that can collect solar energy from a distance.".to_string(),
        amount: 0,
        id: Uuid::new_v4(),
      }
    }
    }
}

pub fn get_all_ships() -> Vec<ShipType> {
    vec![
        ShipType::Battlecruiser,
        ShipType::Bomber,
        ShipType::Battleship,
        ShipType::ColonyShip,
        ShipType::Cruiser,
        ShipType::Deathstar,
        ShipType::Destroyer,
        ShipType::EspionageProbe,
        ShipType::HeavyFighter,
        ShipType::LargeCargo,
        ShipType::LightFighter,
        ShipType::Recycler,
        ShipType::SmallCargo,
        ShipType::SolarSatellite,
    ]
}
