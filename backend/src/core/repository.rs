use crate::core::models::user::{AuditLog, User};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn get_all(
        &self,
        q: Option<String>,
        page: i64,
        limit: i64,
    ) -> Result<Vec<User>, AppError>;
    async fn delete_user(&self, id: i64, admin_username: &str) -> Result<(), AppError>;
    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError>;
}
