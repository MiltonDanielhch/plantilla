use crate::core::{
    models::user::{AuditLog, User},
    repository::UserRepository,
};
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::{error::ErrorKind, SqlitePool};

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteRepository {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, role, created_at"
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.kind() == ErrorKind::UniqueViolation {
                        return Err(AppError::Conflict(
                            "El nombre de usuario ya existe".to_string(),
                        ));
                    }
                }
                Err(AppError::Database(e))
            }
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, role, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_all(
        &self,
        q: Option<String>,
        page: i64,
        limit: i64,
    ) -> Result<Vec<User>, AppError> {
        let offset = (page - 1) * limit;
        let result = match q {
            Some(ref text) if !text.is_empty() => {
                let search = format!("%{}%", text);
                sqlx::query_as::<_, User>("SELECT id, username, password_hash, role, created_at FROM users WHERE username LIKE $1 LIMIT $2 OFFSET $3")
                    .bind(search).bind(limit).bind(offset)
                    .fetch_all(&self.pool).await
            },
            _ => {
                sqlx::query_as::<_, User>("SELECT id, username, password_hash, role, created_at FROM users LIMIT $1 OFFSET $2")
                    .bind(limit).bind(offset)
                    .fetch_all(&self.pool).await
            }
        };
        result.map_err(AppError::Database)
    }

    async fn delete_user(&self, id: i64, admin_username: &str) -> Result<(), AppError> {
        // Transacción implícita o lógica de negocio encapsulada
        let target = sqlx::query_scalar::<_, String>("SELECT username FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .unwrap_or("Fantasma".to_string());
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        sqlx::query("INSERT INTO audit_logs (admin_username, action, target) VALUES ($1, 'DELETE_USER', $2)").bind(admin_username).bind(target).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, admin_username, action, target, timestamp FROM audit_logs ORDER BY id DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
