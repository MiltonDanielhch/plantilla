use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    // Usamos String por simplicidad inicial (SQLite devuelve texto)
    pub created_at: String, 
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}
