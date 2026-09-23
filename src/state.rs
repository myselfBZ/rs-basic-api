use std::collections::HashMap;

use sqlx::PgPool;

use crate::{db, response::User};

pub struct MyState {
    db: PgPool,
    pub users: HashMap<String, User>
}

impl MyState {
    pub async  fn new(pg_url: &str) -> Self {
        let conn = db::open_db_connection(pg_url).await.expect("failed to connect to database");
        return Self{db: conn, users: HashMap::new()}
    }
}
