mod auth;
mod client;
mod config;
mod connected_users;
mod error;
mod socket;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    socket::run::<ogame_core::protocol::Protocol>().await;
}
