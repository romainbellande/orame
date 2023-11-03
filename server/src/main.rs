mod auth;
mod config;
#[allow(warnings, unused)]
mod socket;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    socket::run::<ogame_core::protocol::Protocol>().await;
}
