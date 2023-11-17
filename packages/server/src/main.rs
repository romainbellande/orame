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
    socket::run().await
}

lazy_static::lazy_static! {
    pub static ref GAME_DATA: ogame_core::GameData = {
        let data = std::fs::read("../../data/game_data.cbor").unwrap();
        serde_cbor::from_slice(&data[..]).unwrap()
    };
}
