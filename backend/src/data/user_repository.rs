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
    async fn create_user(&self, username: &str, password_hash: &str, email: Option<&str>) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3) RETURNING id, username, email, password_hash, role, created_at"
        )
        .bind(username)
        .bind(password_hash)
        .bind(email)
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
            "SELECT id, username, email, password_hash, role, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_all(
        &self,
        q: Option<String>,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<User>, i64), AppError> {
        let offset = (page - 1) * limit;
        
        // 1. Obtener usuarios (Paginados)
        let users = match q {
            Some(ref text) if !text.is_empty() => {
                let search = format!("%{}%", text);
                sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, role, created_at FROM users WHERE username LIKE $1 OR email LIKE $1 LIMIT $2 OFFSET $3")
                    .bind(search).bind(limit).bind(offset)
                    .fetch_all(&self.pool).await
            },
            _ => {
                sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, role, created_at FROM users LIMIT $1 OFFSET $2")
                    .bind(limit).bind(offset)
                    .fetch_all(&self.pool).await
            }
        }.map_err(AppError::Database)?;

        // 2. Obtener conteo total (Para paginación)
        let total: i64 = match q {
            Some(ref text) if !text.is_empty() => {
                let search = format!("%{}%", text);
                sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username LIKE $1 OR email LIKE $1")
                    .bind(search)
                    .fetch_one(&self.pool).await
            },
            _ => {
                sqlx::query_scalar("SELECT COUNT(*) FROM users")
                    .fetch_one(&self.pool).await
            }
        }.map_err(AppError::Database)?;

        Ok((users, total))
    }

    async fn get_stats(&self) -> Result<(i64, i64, i64), AppError> {
        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        let admin_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'Admin'")
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        let new_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE date(created_at) = date('now')")
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok((total_users, admin_users, new_users))
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

    async fn update_user(&self, id: i64, email: Option<&str>) -> Result<User, AppError> {
        // Actualizamos solo el email por ahora. 
        // COALESCE asegura que si pasamos NULL, no se borre (aunque aquí controlamos la lógica antes).
        sqlx::query_as::<_, User>(
            "UPDATE users SET email = $1 WHERE id = $2 RETURNING id, username, email, password_hash, role, created_at"
        )
        .bind(email)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
