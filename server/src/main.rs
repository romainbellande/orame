mod auth;
#[allow(warnings, unused)]
mod db;
mod socket;
mod config;

#[tokio::main]
async fn main() {
    socket::run::<ogame_core::protocol::Protocol>().await;
}
