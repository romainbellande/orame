use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod consts;
#[cfg(not(target_arch = "wasm32"))]
mod export;
#[cfg(not(target_arch = "wasm32"))]
mod generate;
#[cfg(not(target_arch = "wasm32"))]
mod plot;

type SystemId = i32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    x: i32,
    y: i32,
    links: Vec<SystemId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planet {
    system_id: SystemId,
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameData {
    systems: Vec<System>,
    planets: BTreeMap<SystemId, Vec<(i32, i32)>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let folder = timestamp_folder()?;

    let systems = generate::systems();
    let planets = generate::planets(systems.len());

    let game_data = GameData { systems, planets };

    export::to_files(&game_data, &folder)?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn timestamp_folder() -> Result<String, Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let folder = format!("data/{timestamp}");
    std::fs::create_dir_all(&folder)?;

    Ok(folder)
}
