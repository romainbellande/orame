use std::sync::Arc;

use crate::{error::*, PrismaClient};

#[allow(async_fn_in_trait)]
pub trait DbModel {
    async fn create(&mut self, conn: &Arc<PrismaClient>) -> Result<&mut Self>
    where
        Self: Sized;
    async fn save(&self, conn: &Arc<PrismaClient>) -> Result<&Self>
    where
        Self: Sized;
    async fn fetch(id: String, conn: &Arc<PrismaClient>) -> Result<Self>
    where
        Self: Sized;
    async fn delete(&self, conn: &Arc<PrismaClient>) -> Result<()>
    where
        Self: Sized;
}
