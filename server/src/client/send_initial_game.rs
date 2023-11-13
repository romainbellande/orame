use std::sync::Arc;

use ogame_core::protocol::Protocol;
use prisma_client::PrismaClient;

use crate::{connected_users::ConnectedUsers, error::*};

pub async fn send_initial_game(
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) {
    let mut game = fetch_game(user_id.clone(), &conn).await;

    game.tick().unwrap();

    connected_users
        .send(user_id.clone(), Protocol::Game(game.clone()))
        .await;

    save_game(game, &conn).await;
}
