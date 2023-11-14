use std::sync::Arc;

use ogame_core::protocol::Protocol;
use prisma_client::{fetch_game, save_game, PrismaClient};

use crate::{connected_users::ConnectedUsers, error::*};

pub async fn send_initial_game(
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    let mut game = fetch_game(user_id.clone(), &conn).await;

    game.tick()?;

    connected_users
        .send(user_id.clone(), Protocol::Game(game.clone()))
        .await?;

    save_game(game, &conn).await;

    Ok(())
}
