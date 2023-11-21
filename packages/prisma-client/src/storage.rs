use std::sync::Arc;

use ogame_core::storage::Storage;

use crate::{db::*, db_model::DbModel, error::*};

impl From<storage::Data> for Storage {
    fn from(db_storage: storage::Data) -> Self {
        Self {
            id: db_storage.id.clone(),
            user_id: db_storage.user_id.clone(),
            structure_id: db_storage.structure_id.clone(),
            items: serde_json::from_str(&db_storage.items).unwrap(),
        }
    }
}

impl DbModel for Storage {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self> {
        let db_storage = conn
            .storage()
            .create(
                user::id::equals(self.user_id.clone()),
                self.structure_id.clone(),
                serde_json::to_string(&self.items).unwrap(),
                vec![],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotCreate(e.to_string()))?;

        self.id = db_storage.id.clone();

        Ok(self)
    }

    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self> {
        conn.storage()
            .update(
                storage::id::equals(self.id.clone()),
                vec![storage::items::set(
                    serde_json::to_string(&self.items).unwrap(),
                )],
            )
            .exec()
            .await
            .map_err(|e| Error::CannotSave(e.to_string()))?;

        Ok(self)
    }

    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self> {
        let db_storage = conn
            .storage()
            .find_first(vec![storage::id::equals(id.clone())])
            .exec()
            .await
            .map_err(|e| Error::CannotFetch(e.to_string()))?
            .ok_or(Error::CannotFetch(format!("Storage {} not found", id)))?;

        Ok(Self::from(db_storage))
    }

    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()> {
        conn.storage()
            .delete(storage::id::equals(self.id.clone()))
            .exec()
            .await
            .map_err(|e| Error::CannotDelete(e.to_string()))?;

        Ok(())
    }
}
