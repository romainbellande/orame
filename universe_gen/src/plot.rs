use std::io::{self, Write};

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

use super::{consts::*, System};

pub fn draw(systems: &Vec<System>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    print!("Plotting galaxy              ");
    stdout.flush()?;

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
    }

    root.present()?;

    println!("{filename}");

    Ok(())
}
