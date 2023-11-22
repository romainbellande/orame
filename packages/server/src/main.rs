mod auth;
mod client;
mod config;
mod connected_users;
mod error;
mod socket;

use dotenv::dotenv;

use error::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if let Err(e) = run().await {
        eprintln!("Error: {:?}", e);

        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let data = std::fs::read("../../data/game_data.cbor").unwrap();
    let game_data = serde_cbor::from_slice(&data[..]).unwrap();
    *ogame_core::GAME_DATA.write().unwrap() = game_data;

    socket::run().await
}
