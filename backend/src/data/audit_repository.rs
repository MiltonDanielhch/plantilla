use crate::core::models::user::AuditLog;
use crate::core::repository::AuditRepository;
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteAuditRepository {
    pool: SqlitePool,
}

impl SqliteAuditRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for SqliteAuditRepository {
    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT id, admin_username, action, target, timestamp FROM audit_logs ORDER BY id DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn create_audit_log(&self, admin_username: &str, action: &str, target: &str) -> Result<(), AppError> {
        sqlx::query("INSERT INTO audit_logs (admin_username, action, target) VALUES ($1, $2, $3)")
            .bind(admin_username)
            .bind(action)
            .bind(target)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}
