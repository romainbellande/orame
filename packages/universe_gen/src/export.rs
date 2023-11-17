use std::io::{self, Write};

pub fn to_files(
    game_data: &crate::GameData,
    folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // super::plot::draw(systems, &format!("{folder}/galaxy.png"))?;

    cbor(game_data, &format!("{folder}/game_data.cbor"))?;

    Ok(())
}

pub fn json<T>(t: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize,
{
    let mut stdout = io::stdout();
    print!("{:<30}", "Serializing to json");
    stdout.flush()?;

    let json = serde_json::to_vec(t)?;

    std::fs::write(filename, &json)?;

    println!("{filename} - {} kb", json.len() / 1024);

    Ok(())
}

pub fn cbor<T>(t: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize,
{
    let mut stdout = io::stdout();
    print!("{:<30}", "Serializing to cbor");
    stdout.flush()?;

    let cbor = serde_cbor::to_vec(t)?;

    std::fs::write(filename, &cbor)?;

    println!("{filename} - {} kb", cbor.len() / 1024);

    Ok(())
}
