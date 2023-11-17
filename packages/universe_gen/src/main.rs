use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

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
    let recipes = read_recipes("./recipes.toml");

    let game_data = GameData {
        systems,
        planets,
        stations,
        recipes,
    };

    export::to_files(&game_data, &folder)?;

    Ok(())
}

use serde::{Deserialize, Serialize};
pub type Ticks = usize;
pub type RecipeId = String;
pub type ItemId = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TomlRecipe {
    ticks: Ticks,
    inputs: ItemSet,
    outputs: ItemSet,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Recipes(pub HashMap<RecipeId, TomlRecipe>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ItemSet(HashMap<ItemId, usize>);

fn read_recipes(filename: &str) -> BTreeMap<universe_gen::RecipeId, universe_gen::Recipe> {
    let mut stdout = std::io::stdout();
    print!("{:<30}", "Reading recipes");
    stdout.flush().unwrap();

    let recipes = std::fs::read_to_string(filename).unwrap();
    let recipes: Recipes = toml::from_str(&recipes).unwrap();

    println!("{nb_recipes}", nb_recipes = recipes.0.len());

    let recipes = recipes
        .0
        .into_iter()
        .map(|(name, recipe)| {
            let recipe = universe_gen::Recipe {
                name: name.clone(),
                description: "".to_string(),
                ticks: recipe.ticks,
                inputs: recipe.inputs.0.into_iter().collect(),
                outputs: recipe.outputs.0.into_iter().collect(),
            };
            (name, recipe)
        })
        .collect::<BTreeMap<_, _>>();

    recipes
}

fn timestamp_folder() -> Result<String, Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let folder = format!("data/{timestamp}");
    std::fs::create_dir_all(&folder)?;

    Ok(folder)
}
