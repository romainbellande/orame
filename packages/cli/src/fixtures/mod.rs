mod models;

use models::user;
use prisma_client::new_client_with_url;
use prisma_client_rust::{NewClientError, QueryError};

#[derive(Debug)]
pub enum FixtureError {
    QueryError(QueryError),
    ClientError(NewClientError),
}

pub async fn exec(database_url: String) -> Result<(), FixtureError> {
    let conn = new_client_with_url(&database_url)
        .await
        .map_err(FixtureError::ClientError)?;

    user::fixture(&conn)
        .await
        .map_err(FixtureError::QueryError)?;

    Ok(())
}
