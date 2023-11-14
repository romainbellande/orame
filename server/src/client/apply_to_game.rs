use std::sync::Arc;

use futures::Future;
use ogame_core::{game::Game, protocol::Protocol};
use prisma_client::{fetch_game, save_game, PrismaClient};

use super::handle_flight;
use crate::connected_users::ConnectedUsers;
use crate::error::*;

pub async fn apply_to_game_with<F: FnMut(&mut Game) -> Result<T>, T>(
    user_id: String,
    conn: &Arc<PrismaClient>,
    mut cb: F,
) -> Result<T> {
    let mut game = fetch_game(user_id, conn).await;

    let ret = cb(&mut game);

    save_game(game, conn).await;

    ret
}

pub async fn apply_to_game_with_async<Fut: Future<Output = Result<Game>>, F: FnMut(Game) -> Fut>(
    user_id: String,
    conn: &Arc<PrismaClient>,
    mut cb: F,
) -> Result<()> {
    let game = fetch_game(user_id, conn).await;

    let game = cb(game).await?;

    save_game(game, conn).await;

    Ok(())
}

pub async fn apply_msg_to_game(
    user_id: String,
    message: Protocol,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    let message2 = message.clone();
    let connected_users2 = connected_users.clone();

    apply_to_game_with_async(user_id, conn, move |mut game| {
        let message3 = message2.clone();
        let connected_users3 = connected_users2.clone();
        async move {
            handle_flight(&mut game, message3.clone(), connected_users3, conn).await?;

            game.process_message(message3)?;

            Ok(game)
        }
    })
    .await?;

    Ok(())
}