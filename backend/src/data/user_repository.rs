use sqlx::SqlitePool;
use crate::core::models::user::User;

/// Obtiene todos los usuarios registrados en la frecuencia
pub async fn get_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, password_hash, role, created_at FROM users")
        .fetch_all(pool)
        .await
}

/// Busca un usuario por su nombre de usuario
pub async fn get_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, password_hash, role, created_at FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}