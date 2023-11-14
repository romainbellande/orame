use std::collections::BTreeMap;
use std::io::{self, Write};

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

const GALAXY_SIZE: i32 = 100000; // on one axis
const SYSTEM_NB: i32 = 100; // on one axis
const SYSTEM_SIZE: i32 = 1000; // on one axis
const PLANET_NB: i32 = 10; // on one axis
const SYSTEM_GAP: i32 = GALAXY_SIZE / SYSTEM_NB;
const PLANET_GAP: i32 = SYSTEM_SIZE / PLANET_NB;

type SystemId = i32;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct System {
    id: SystemId,
    x: i32,
    y: i32,
    links: Vec<SystemId>,
}

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    let mut systems = generate_systems();

    generate_links(&mut systems);

    let planets = generate_planets(systems.len());

    print!("Plotting galaxy              ");
    stdout.flush()?;
    plot(&systems).unwrap();
    println!("galaxy.png");

    print!("Serializing systems          ");
    stdout.flush()?;
    let systems_json = serde_json::to_string(&systems)?;
    println!("{} kb", systems_json.len() / 1024);

    print!("Serializing planets          ");
    stdout.flush()?;
    let planets_json = serde_json::to_string(&planets)?;
    println!("{} kb", planets_json.len() / 1024);

    print!("Writing systems files        ");
    stdout.flush()?;
    std::fs::write("systems.json", systems_json)?;
    println!("systems.json");

    print!("Writing planets files        ");
    stdout.flush()?;
    std::fs::write("planets.json", planets_json)?;
    println!("planets.json");

    Ok(())
}

fn generate_systems() -> Vec<System> {
    print!("Generating systems           ");

    let systems: Vec<_> = generate(SYSTEM_NB, SYSTEM_GAP)
        .iter()
        .enumerate()
        .map(|(i, (x, y))| System {
            id: i as i32,
            x: *x,
            y: *y,
            links: vec![],
        })
        .collect();

    println!("{}", systems.len());

    systems
}

fn generate_links(systems: &mut Vec<System>) {
    print!("Generating system links      \r");

    let mut nb_links = 0;

    for i in 0..systems.len() {
        for j in 0..systems.len() {
            if i != j {
                let system1 = systems[i].clone();
                let system2 = systems[j].clone();

                let distance = (((system1.x as i64 - system2.x as i64).pow(2)
                    + (system1.y as i64 - system2.y as i64).pow(2))
                    as f64)
                    .sqrt()
                    .abs();

                if distance < (SYSTEM_GAP + SYSTEM_GAP / 3) as f64 {
                    nb_links += 1;
                    print!("Generating system links      {}\r", nb_links);
                    systems[i].links.push(system2.id);
                }
            }
        }
    }

    println!("Generating system links      {}", nb_links);
}

fn generate_planets(system_len: usize) -> BTreeMap<SystemId, Vec<(i32, i32)>> {
    print!("Generating planets           \r");

    let mut planets = BTreeMap::new();

    for i in 0..system_len {
        print!(
            "Generating planets           System {}/{}, Planets {} \r",
            i,
            system_len,
            PLANET_NB * PLANET_NB * i as i32
        );
        planets.insert(i as i32, generate(PLANET_NB, PLANET_GAP));
    }

    println!(
        "Generating planets           {}                                       ",
        PLANET_NB * PLANET_NB * system_len as i32
    );

    planets
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

fn plot(systems: &Vec<System>) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("galaxy.png");

    let backend = BitMapBackend::new(&filename, (1980, 1080));
    let root = backend.into_drawing_area();

    root.fill(&RGBColor(0, 0, 0))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..GALAXY_SIZE as f32 + SYSTEM_GAP as f32,
        0f32..GALAXY_SIZE as f32 + SYSTEM_GAP as f32,
        (0..1980, 0..1080),
    ));

    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 2, ShapeStyle::from(&WHITE).filled());
    };

    for system in systems {
        root.draw(&dot_and_label(system.x as f32, system.y as f32))?;
        for link in &system.links {
            let system2 = systems.iter().find(|s| s.id == *link).unwrap();
            root.draw(&PathElement::new(
                vec![
                    (system.x as f32, system.y as f32),
                    (system2.x as f32, system2.y as f32),
                ],
                ShapeStyle::from(&WHITE).filled(),
            ))?;
        }
        // backend.draw_line((system.x, system.y), (system.x, system.y), &WHITE)?;
    }
    root.present()?;
    Ok(())
}
