use crate::core::models::user::{AuditLog, User};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, username: &str, password_hash: &str, email: Option<&str>) -> Result<User, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    async fn get_all(
        &self,
        q: Option<String>,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<User>, i64), AppError>;
    async fn get_stats(&self) -> Result<(i64, i64, i64), AppError>;
    async fn delete_user(&self, id: i64, admin_username: &str) -> Result<(), AppError>;
    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError>;
    async fn update_user(&self, id: i64, email: Option<&str>) -> Result<User, AppError>;
    async fn update_avatar(&self, id: i64, avatar_url: &str) -> Result<User, AppError>;
}
