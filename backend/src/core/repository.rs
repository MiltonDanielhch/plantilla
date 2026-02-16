use crate::core::models::user::{AuditLog, PasswordResetToken, RefreshToken, User};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, username: &str, password_hash: &str, email: Option<&str>) -> Result<User, AppError>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
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
    async fn update_password(&self, id: i64, password_hash: &str) -> Result<(), AppError>;
    
    // Refresh Tokens
    async fn create_refresh_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<RefreshToken, AppError>;
    async fn get_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;
    async fn mark_refresh_token_used(&self, token_id: i64) -> Result<(), AppError>;
    async fn revoke_user_refresh_tokens(&self, user_id: i64) -> Result<(), AppError>;
    
    // Password Reset Tokens
    async fn create_password_reset_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<PasswordResetToken, AppError>;
    async fn get_password_reset_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError>;
    async fn mark_password_reset_token_used(&self, token_id: i64) -> Result<(), AppError>;
}
