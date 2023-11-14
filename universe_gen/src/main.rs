use serde::{Deserialize, Serialize};

mod consts;
mod export;
mod generate;
mod plot;

type SystemId = i32;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct System {
    id: SystemId,
    x: i32,
    y: i32,
    links: Vec<SystemId>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);

        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let folder = timestamp_folder()?;

    let systems = generate::systems();

    let planets = generate::planets(systems.len());

    export::to_files(&systems, &planets, &folder)?;

    Ok(())
}

fn timestamp_folder() -> Result<String, Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let folder = format!("data/{timestamp}");
    std::fs::create_dir_all(&folder)?;

    Ok(folder)
}
