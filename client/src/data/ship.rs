use ogame_core::ship_type::ShipType;

#[derive(Clone)]
pub struct Ship {
    pub ship_type: ShipType,
    pub name: &'static str,
    pub class: &'static str,
    pub description: &'static str,
    pub amount: usize,
}

impl Ship {
    pub fn set_amount(&mut self, amount: usize) -> Self {
        self.amount = amount;
        self.clone()
    }
}

impl From<ShipType> for Ship {
    fn from(ship_type: ShipType) -> Self {
        match ship_type {
      ShipType::Battlecruiser => Self {
        ship_type,
        name: "Battlecruiser",
        class: "interceptor",
        description: "This ship is deadly when it comes to destroying fleets. With its improved laser cannons, it holds a privileged position among the heavy ships, capable of taking down several of them in no time.",
        amount: 0,
      },
      ShipType::Bomber => Self {
        ship_type,
        name: "Bomber",
        class: "bomber",
        description: "The bomber is a special purpose spacecraft developed to break through heavy planetary defense.",
        amount: 0,
      },
      ShipType::Battleship => Self {
        ship_type,
        name: "Battleship",
        class: "battleship",
        description: "Battleships provide the backbone of any military fleet. Heavy armour, strong weapons systems and high cruising speed, as well as a large cargo bay, make this ship a tough opponent to fight against.",
        amount: 0,
      },
      ShipType::ColonyShip => Self {
        ship_type,
        name: "Colony Ship",
        class: "colonyShip",
        description: "This ship provides the means necessary to go where no man has gone before and to colonize new worlds.",
        amount: 0,
      },
      ShipType::Cruiser => Self {
        ship_type,
        name: "Cruiser",
        class: "cruiser",
        description: "Combat cruisers have armor almost three times as strong as heavy fighters and support more than twice their firing power. Their traveling speed is also amongst the fastest seen.",
        amount: 0,
      },
      ShipType::Deathstar => Self {
        ship_type,
        name: "Deathstar",
        class: "deathstar",
        description: "The Deathstar is the most powerful ship in the universe. It is the ultimate weapon of destruction. It is so powerful that it can destroy entire planets. It is also the slowest ship in the universe.",
        amount: 0,
      },
      ShipType::Destroyer => Self {
        ship_type,
        name: "Destroyer",
        class: "destroyer",
        description: "The destroyer is a heavily armed ship that can be used for either attack or defense. It is equipped with a powerful shield and a strong hull. It is also capable of carrying a large cargo.",
        amount: 0,
      },
      ShipType::EspionageProbe => Self {
        ship_type,
        name: "Espionage Probe",
        class: "espionageProbe",
        description: "The espionage probe is a small, fast ship that is used to spy on enemy planets. It is equipped with a powerful sensor that can scan enemy planets and fleets. It is also equipped with a cloaking device that makes it invisible to enemy radar.",
        amount: 0,
      },
      ShipType::HeavyFighter => Self {
        ship_type,
        name: "Heavy Fighter",
        class: "fighterHeavy",
        description: "The heavy fighter is a straight evolution of the light fighter offering increased shielding and firing power.",
        amount: 0,
      },
      ShipType::LargeCargo => Self {
        ship_type,
        name: "Large Cargo",
        class: "transporterLarge",
        description: "The large cargo ship is a large, slow ship that is used to transport large amounts of resources. It is equipped with a powerful engine that allows it to travel at high speeds.",
        amount: 0,
      },
      ShipType::LightFighter => Self {
        ship_type,
        name: "Light Fighter",
        class: "fighterLight",
        description: "The light fighter is a maneuverable ship you can find on nearly every planet. The costs are not particularly high and its shield power and cargo capacity are very low. Used for moonshots in most universes.",
        amount: 0,
      },
      ShipType::Recycler => Self {
        ship_type,
        name: "Recycler",
        class: "recycler",
        description: "The recycler is a ship that is used to collect debris from destroyed ships and moons. It is equipped with a powerful tractor beam that can pull in debris from a distance.",
        amount: 0,
      },
      ShipType::SmallCargo => Self {
        ship_type,
        name: "Small Cargo",
        class: "transporterSmall",
        description: "Small cargo ships are very agile ships used to transport resources from one planet to another. Useful for farming. Switches to Impulse Drive at level 5, doubling its speed.",
        amount: 0,
      },
      ShipType::SolarSatellite => Self {
        ship_type,
        name: "Solar Satellite",
        class: "solarSatellite",
        description: "The solar satellite is a small, unmanned satellite that is used to collect solar energy. It is equipped with a powerful solar panel that can collect solar energy from a distance.",
        amount: 0,
      }
    }
    }
}
