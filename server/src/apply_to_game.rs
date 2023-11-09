use std::sync::Arc;

use futures::Future;
use ogame_core::{game::Game, protocol::Protocol};
use prisma_client::{fetch_game, save_game, PrismaClient};

use crate::connected_users::ConnectedUsers;
use crate::socket::handle_flight;

pub async fn apply_to_game_with<F: FnMut(&mut Game) -> ()>(
    user_id: String,
    conn: &Arc<PrismaClient>,
    mut cb: F,
) {
    let mut game = fetch_game(user_id, conn).await;

    cb(&mut game);

    save_game(game, conn).await;
}

pub async fn apply_to_game_with_async<Fut: Future<Output = Game>, F: FnMut(Game) -> Fut>(
    user_id: String,
    conn: &Arc<PrismaClient>,
    mut cb: F,
) {
    let game = fetch_game(user_id, conn).await;

    let game = cb(game).await;

    save_game(game, conn).await;
}

pub async fn apply_msg_to_game(
    user_id: String,
    message: Protocol,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) {
    let message2 = message.clone();
    let connected_users2 = connected_users.clone();
    apply_to_game_with_async(user_id, conn, move |mut game| {
        let message3 = message2.clone();
        let connected_users3 = connected_users2.clone();
        async move {
            handle_flight(&mut game, message3.clone(), connected_users3, conn).await;
            game.process_message(message3).unwrap();
            game
        }
    })
    .await;
}
