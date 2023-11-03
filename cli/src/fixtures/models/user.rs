use prisma_client::PrismaClient;
use prisma_client_rust::QueryError;

pub async fn fixture(conn: &PrismaClient) -> Result<(), QueryError> {
    conn.user()
        .create_many(vec![(
            "JDOE".to_string(),
            "john@example.com".to_string(),
            "foobar".to_string(),
            vec![],
        )])
        .exec()
        .await?;

    Ok(())
}
