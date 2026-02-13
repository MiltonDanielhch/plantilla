use sqlx::SqlitePool;
use crate::core::models::user::User;

/// Obtiene todos los usuarios registrados en la frecuencia
pub async fn get_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}