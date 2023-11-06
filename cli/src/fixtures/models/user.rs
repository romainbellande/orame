use prisma_client::PrismaClient;
use prisma_client_rust::QueryError;

pub async fn fixture(conn: &PrismaClient) -> Result<(), QueryError> {
    conn.user()
        .create_many(vec![(
            "test".to_string(),
            "test@test.test".to_string(),
            "test".to_string(),
            vec![],
        )])
        .exec()
        .await?;

    Ok(())
}
