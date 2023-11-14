use std::collections::BTreeMap;

use rand::Rng;

use super::{consts::*, System, SystemId};

pub fn systems() -> Vec<System> {
    print!("Generating systems           ");

    let mut systems: Vec<_> = generate(SYSTEM_NB, SYSTEM_GAP)
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

    links(&mut systems);

    systems
}

fn links(systems: &mut Vec<System>) {
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

pub fn planets(system_len: usize) -> BTreeMap<SystemId, Vec<(i32, i32)>> {
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
