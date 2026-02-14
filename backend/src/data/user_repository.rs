use sqlx::{SqlitePool, QueryBuilder, Sqlite};
use crate::core::models::user::User;

/// Obtiene todos los usuarios registrados en la frecuencia
pub async fn get_all(pool: &SqlitePool, filter: Option<String>) -> Result<Vec<User>, sqlx::Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT id, username, password_hash, role, created_at FROM users");

    if let Some(q) = filter {
        let search_term = format!("%{}%", q);
        query_builder.push(" WHERE username LIKE ");
        query_builder.push_bind(search_term);
    }

    query_builder.push(" ORDER BY created_at DESC");

    let query = query_builder.build_query_as::<User>();
    query.fetch_all(pool).await
}

/// Busca un usuario por su nombre de usuario
pub async fn get_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, password_hash, role, created_at FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}