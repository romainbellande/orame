use crate::utils::error::*;
use futures::{channel::mpsc::Sender, SinkExt};
use leptos::spawn_local;
use ogame_core::{building_type::BuildingType, game::Game, protocol::Protocol};

#[derive(Clone)]
pub struct GameWrapper {
    pub game: Game,
    socket_sender: Sender<Protocol>,
}

impl GameWrapper {
    pub fn new(game: Game, socket_sender: Sender<Protocol>) -> Self {
        Self {
            game,
            socket_sender,
        }
    }

    pub fn upgrade_building(
        &mut self,
        planet_id: String,
        building_type: BuildingType,
    ) -> Result<()> {
        let message = Protocol::UpgradeBuilding {
            planet_id,
            building_type,
        };

        self.game.process_message(message.clone())?;
        let mut socket_sender = self.socket_sender.clone();
        spawn_local(async move {
            socket_sender.send(message).await.unwrap();
        });

        Ok(())
    }
}

impl std::ops::Deref for GameWrapper {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

impl std::ops::DerefMut for GameWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.game
    }
}
