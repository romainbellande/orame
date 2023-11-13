use std::sync::Arc;

use ogame_core::{fleet::Fleet, game::Game, protocol::Protocol};
use prisma_client::*;

use crate::{connected_users::ConnectedUsers, error::*};

pub async fn handle_flight(
    game: &mut Game,
    message: Protocol,
    connected_users: ConnectedUsers,
    conn: &Arc<PrismaClient>,
) -> Result<()> {
    // special case for flight
    if let Protocol::SendShips {
        from_planet_id,
        to_planet_id,
        ships,
        mission,
        resources,
        speed_ratio,
    } = message
    {
        // fetch to_planet from db
        let to_planet = conn
            .planet()
            .find_unique(prisma_client::planet::id::equals(to_planet_id.clone()))
            .exec()
            .await?
            .ok_or(Error::NotFound)?;

        // create flight
        let flight = game
            .create_flight(
                "".to_string(),
                from_planet_id,
                to_planet_id.clone(),
                &((*to_planet.coordinates.unwrap()).into()),
                Fleet::new("".to_string(), ships),
                resources,
                mission,
                speed_ratio,
            )
            .unwrap();

        // save it in db
        let ships = create_ships(&flight.ships.ships, conn).await;
        let resources = create_resources(&flight.resources, conn).await;
        let db_flight = create_flight(flight, ships.id, resources.id, conn).await;

        // send it to the two players
        let msg = Protocol::InboundFleet(db_flight.into());

        // get player id from to_planet
        let target_planet = fetch_planet(to_planet_id, conn).await;

        connected_users
            .send(game.player_id.clone(), msg.clone())
            .await;

        game.process_message(msg.clone()).unwrap();

        // if target player is different from current player then send it to him
        if target_planet.user_id != game.player_id {
            connected_users
                .send(target_planet.user_id.clone(), msg.clone())
                .await;

            super::apply_to_game_with(target_planet.user_id.clone(), conn, move |game| {
                game.process_message(msg.clone()).unwrap();
            })
            .await;
        }
    }
}
