use crate::core::models::user::{
    AuditLog, DbRole, EmailVerificationToken, PasswordResetToken, Permission, 
    RefreshToken, Role, RolePermission, User
};
use crate::error::AppError;
use async_trait::async_trait;

/// Repositorio de usuarios - Operaciones CRUD básicas
#[async_trait]
pub trait UserRepository: Send + Sync {
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
    async fn update_user(&self, id: i64, email: Option<&str>, role: Option<Role>) -> Result<User, AppError>;
    async fn update_avatar(&self, id: i64, avatar_url: &str) -> Result<User, AppError>;
    async fn update_password(&self, id: i64, password_hash: &str) -> Result<(), AppError>;
    async fn verify_email(&self, user_id: i64) -> Result<(), AppError>;
}

/// Repositorio de tokens - Autenticación y seguridad
#[async_trait]
pub trait TokenRepository: Send + Sync {
    // Refresh Tokens
    async fn create_refresh_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<RefreshToken, AppError>;
    async fn get_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;
    async fn mark_refresh_token_used(&self, token_id: i64) -> Result<(), AppError>;
    async fn revoke_user_refresh_tokens(&self, user_id: i64) -> Result<(), AppError>;
    
    // Password Reset Tokens
    async fn create_password_reset_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<PasswordResetToken, AppError>;
    async fn get_password_reset_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError>;
    async fn mark_password_reset_token_used(&self, token_id: i64) -> Result<(), AppError>;
    
    // Email Verification Tokens
    async fn create_email_verification_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<EmailVerificationToken, AppError>;
    async fn get_email_verification_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, AppError>;
    async fn mark_email_verification_token_used(&self, token_id: i64) -> Result<(), AppError>;
}

/// Repositorio de auditoría - Trazabilidad
#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError>;
    async fn create_audit_log(&self, admin_username: &str, action: &str, target: &str) -> Result<(), AppError>;
}

/// Repositorio de RBAC - Control de acceso
#[async_trait]
pub trait RbacRepository: Send + Sync {
    async fn get_roles(&self) -> Result<Vec<DbRole>, AppError>;
    async fn get_permissions(&self) -> Result<Vec<Permission>, AppError>;
    async fn get_role_permissions(&self) -> Result<Vec<RolePermission>, AppError>;
    async fn create_role(&self, name: &str, description: Option<&str>, permissions: &[i64]) -> Result<DbRole, AppError>;
    async fn update_role(&self, id: i64, name: Option<&str>, description: Option<&str>, permissions: Option<&[i64]>) -> Result<DbRole, AppError>;
    async fn delete_role(&self, id: i64) -> Result<(), AppError>;
    async fn update_permission(&self, id: i64, description: &str) -> Result<Permission, AppError>;
}

/// Trait compuesto para compatibilidad hacia atrás
/// 
/// Implementa todos los repositorios en uno solo
#[async_trait]
pub trait Repository: UserRepository + TokenRepository + AuditRepository + RbacRepository {}

// Implementación automática para cualquier tipo que implemente todos los traits
impl<T> Repository for T where T: UserRepository + TokenRepository + AuditRepository + RbacRepository {}
