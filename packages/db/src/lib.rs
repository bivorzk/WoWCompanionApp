pub type DbPool = sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<DbPool, sqlx::Error> {
    DbPool::connect(database_url).await
}
