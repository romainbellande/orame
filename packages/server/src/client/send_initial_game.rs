use std::sync::Arc;

use ogame_core::{game::Game, protocol::Protocol};
use prisma_client::{DbModel, PrismaClient, User};

use crate::{connected_users::ConnectedUsers, error::*};

pub async fn send_initial_game(
    user_id: String,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    let mut game: Game = User::fetch(user_id.clone(), conn).await?.into();

    game.tick()?;

    connected_users
        .send(user_id.clone(), Protocol::Game(game.clone()))
        .await?;

    let user: User = game.into();

    user.save(conn).await?;

    Ok(())
}
