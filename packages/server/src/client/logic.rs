use std::sync::Arc;

use ogame_core::protocol::Protocol;
use prisma_client::{DbModel, PrismaClient};

use crate::error::*;

pub async fn db_msg_handling(
    user_id: String,
    message: Protocol,
    conn: &Arc<PrismaClient>,
) -> Result<Protocol> {
    match message {
        Protocol::SendShips {
            from_id,
            to_id,
            ships,
            mission,
            speed_ratio,
        } => {
            let mut flight = ogame_core::flight::Flight::create(
                "".to_string(),
                user_id.clone(),
                from_id,
                to_id,
                ships,
                mission,
                speed_ratio,
            )?;

            flight.create(conn).await?;

            Ok(Protocol::Flight(flight.clone()))
        }
        _ => unimplemented!("Not implemented yet!"),
    }
}
