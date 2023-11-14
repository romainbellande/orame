use ogame_core::protocol::Protocol;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;

use crate::error::*;

#[derive(Clone)]
pub struct ConnectedUsers {
    pub users: Arc<RwLock<HashMap<String, Sender<Protocol>>>>,
}

impl ConnectedUsers {
    pub fn empty() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&self, user_id: String) -> Receiver<Protocol> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        self.users.write().await.insert(user_id, tx);
        rx
    }

    pub async fn remove(&self, user_id: String) {
        self.users.write().await.remove(&user_id);
    }

    pub async fn send(&self, user_id: String, message: Protocol) -> Result<()> {
        if let Some(sender) = self.users.write().await.get_mut(&user_id) {
            sender.try_send(message)?;
        }

        Ok(())
    }
}
