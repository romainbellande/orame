use std::{collections::BTreeMap, io::Read};

use rand::Rng;

use crate::consts::*;
use ogame_core::{Planet, PlanetId, Station, StationId, System, SystemId};

pub fn parse_system_names() -> Vec<String> {
    let mut res = vec![];

    let mut file = std::fs::File::open("./data/system_names.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        res.push(line.to_string());
    }

    res
}

pub fn systems() -> BTreeMap<SystemId, System> {
    print!("{:<30}", "Generating systems");

    let system_names = parse_system_names();

    let mut systems: BTreeMap<_, _> = generate(SYSTEM_NB, SYSTEM_GAP)
        .iter()
        .enumerate()
        .map(|(i, (x, y))| {
            let id = cuid2::cuid().to_string();
            let system = System {
                id: id.clone(),
                name: system_names[i].clone(),
                x: *x,
                y: *y,
                links: vec![],
            };
            (id, system)
        })
        .collect();

    println!("{}", systems.len());

    links(&mut systems);

    systems
}

fn links(systems: &mut BTreeMap<SystemId, System>) {
    print!("{:<30}\r", "Generating system links");

    let mut nb_links = 0;

    let mut links = BTreeMap::new();
    for system in systems.values() {
        for system2 in systems.values() {
            if system.id != system2.id {
                let distance = {
                    (((system.x as i64 - system2.x as i64).pow(2)
                        + (system.y as i64 - system2.y as i64).pow(2)) as f64)
                        .sqrt()
                        .abs()
                };

                if distance < (SYSTEM_GAP + SYSTEM_GAP / 3) as f64 {
                    nb_links += 1;
                    print!("{:<30}{nb_links}\r", "Generating system links");
                    let system2_id = system2.id.clone();
                    links
                        .entry(system.id.clone())
                        .or_insert(vec![])
                        .push(system2_id);
                }
            }
        }
    }

    for link in links {
        systems.get_mut(&link.0).unwrap().links = link.1;
    }

    println!("{:<30}{nb_links}", "Generating system links");
}

pub fn planets(systems: &BTreeMap<SystemId, System>) -> BTreeMap<PlanetId, Planet> {
    print!("{:<30}", "Generating planets\r");

    let mut planets = BTreeMap::new();

    for (i, system) in systems.values().enumerate() {
        print!(
            "{:<30}System {}/{}, Planets {}\r",
            "Generating planets",
            i,
            systems.len(),
            PLANET_NB * PLANET_NB * i as i32
        );

        for (j, (x, y)) in generate(PLANET_NB, PLANET_GAP).into_iter().enumerate() {
            let id = cuid2::cuid().to_string();
            let planet = Planet {
                id: id.clone(),
                name: system.name.clone() + " " + &(j + 1).to_string(),
                system_id: system.id.clone(),
                x,
                y,
            };
            planets.insert(id, planet);
        }
    }

    println!(
        "{:<30}System {}/{}, Planets {}\r",
        "Generating planets",
        systems.len(),
        systems.len(),
        PLANET_NB * PLANET_NB * systems.len() as i32
    );

    planets
}

pub fn stations(systems: &BTreeMap<SystemId, System>) -> BTreeMap<StationId, Station> {
    print!("{:<30}", "Generating stations\r");

    let mut stations = BTreeMap::new();

    for (i, system) in systems.values().enumerate() {
        print!(
            "{:<30}System {}/{}, Stations {}\r",
            "Generating stations",
            i,
            systems.len(),
            STATION_NB * STATION_NB * i as i32
        );

        for (x, y) in generate(STATION_NB, STATION_GAP) {
            let id = cuid2::cuid().to_string();
            let station = Station {
                id: id.clone(),
                name: system.name.clone() + &" Station",
                system_id: system.id.clone(),
                x,
                y,
            };
            stations.insert(id, station);
        }
    }

    println!(
        "{:<30}System {}/{}, Stations {}\r",
        "Generating stations",
        systems.len(),
        systems.len(),
        STATION_NB * STATION_NB * systems.len() as i32
    );

    stations
}
fn generate(nb: i32, gap: i32) -> Vec<(i32, i32)> {
    let mut res = vec![];

    for x in 0..nb {
        for y in 0..nb {
            let random = randomize(gap);
            let x = x * gap + random + gap;
            let y = y * gap + random + gap;

            res.push((x, y))
        }
    }

    res
}

fn randomize(gap: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-(gap / 2)..(gap / 4))
}
