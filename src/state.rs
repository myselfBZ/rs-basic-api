use sqlx::PgPool;

use crate::db;

pub struct MyState {
    pub db: PgPool,
}

impl MyState {
    pub async  fn new(pg_url: &str) -> Self {
        let conn = db::open_db_connection(pg_url).await.expect("failed to connect to database");
        return Self{db: conn}
    }
}
