use sqlx::{Error, PgPool};
use crate::entities::User;


pub struct CreateUserParam {
   pub name: String 
}

pub async fn get_user(pool: &PgPool, id: uuid::Uuid) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name FROM users WHERE id = $1"#,
        id
        )
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn del(pool: &PgPool, id: uuid::Uuid) -> Option<Error> {
    let r = sqlx::query_scalar!(
        r#"DELETE FROM users WHERE id = $1"#,
        id
    )
     .execute(pool)
    .await; 
    match r {
        Ok(r) => {
            if r.rows_affected() == 0 {
                Some(sqlx::Error::RowNotFound)
            } else {
                None
            }
        },
        Err(e) => Some(e)
    }
}

pub async fn create_user(pool: &PgPool, user: CreateUserParam) -> Result<uuid::Uuid, Error> {
    let id: uuid::Uuid = sqlx::query_scalar!(
        r#"INSERT INTO users(name) VALUES($1) RETURNING id"#,
        user.name
        )
        .fetch_one(pool)
        .await?;
    Ok(id)
}
