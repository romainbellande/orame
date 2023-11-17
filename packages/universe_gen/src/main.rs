use universe_gen::GameData;

mod export;
mod generate;
mod plot;

fn main() {
    if let Err(e) = run(std::env::args().nth(1)) {
        eprintln!("Error: {}", e);

        std::process::exit(1);
    }
}

fn run(folder: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let folder = match folder {
        Some(folder) => folder,
        None => timestamp_folder()?,
    };

    let systems = generate::systems();
    let planets = generate::planets(&systems);
    let stations = generate::stations(&systems);

    let game_data = GameData {
        systems,
        planets,
        stations,
    };

    export::to_files(&game_data, &folder)?;

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
