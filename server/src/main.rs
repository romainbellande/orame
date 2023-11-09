mod apply_to_game;
mod auth;
mod config;
mod connected_users;
mod socket;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    socket::run::<ogame_core::protocol::Protocol>().await;
}
