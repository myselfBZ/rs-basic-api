use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String
}
