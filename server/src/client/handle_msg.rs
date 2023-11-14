use std::sync::Arc;

use prisma_client::PrismaClient;

use ogame_core::protocol::Protocol;

use crate::{client::apply_msg_to_game, connected_users::ConnectedUsers, error::*};

pub async fn handle_msg(
    user_id: String,
    protocol: Protocol,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    apply_msg_to_game(user_id.clone(), protocol, connected_users.clone(), conn).await
}
