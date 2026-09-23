use sqlx::PgPool;

pub mod users;

pub async fn open_db_connection(pg_url: &str) -> Result<PgPool, sqlx::Error> {
    let conn = PgPool::connect(pg_url).await?; 
    return Ok(conn)
}
