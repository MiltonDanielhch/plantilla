pub mod audit_repository;
pub mod rbac_repository;
pub mod token_repository;
pub mod user_repository;

use crate::core::models::user::{
    AuditLog, DbRole, EmailVerificationToken, PasswordResetToken, Permission, 
    RefreshToken, Role, RolePermission, User
};
use crate::core::repository::{
    AuditRepository, RbacRepository, Repository, TokenRepository, UserRepository
};
use crate::data::{
    audit_repository::SqliteAuditRepository,
    rbac_repository::SqliteRbacRepository,
    token_repository::SqliteTokenRepository,
    user_repository::SqliteUserRepository,
};
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::SqlitePool;

/// Repositorio compuesto que implementa todos los traits
/// 
/// Mantiene compatibilidad hacia atrás con código existente
/// mientras permite la migración gradual a repositorios especializados
pub struct SqliteRepository {
    users: SqliteUserRepository,
    tokens: SqliteTokenRepository,
    audit: SqliteAuditRepository,
    rbac: SqliteRbacRepository,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            users: SqliteUserRepository::new(pool.clone()),
            tokens: SqliteTokenRepository::new(pool.clone()),
            audit: SqliteAuditRepository::new(pool.clone()),
            rbac: SqliteRbacRepository::new(pool),
        }
    }

    // Acceso a repositorios individuales
    pub fn users(&self) -> &SqliteUserRepository {
        &self.users
    }

    pub fn tokens(&self) -> &SqliteTokenRepository {
        &self.tokens
    }

    pub fn audit(&self) -> &SqliteAuditRepository {
        &self.audit
    }

    pub fn rbac(&self) -> &SqliteRbacRepository {
        &self.rbac
    }
}

// Implementación de UserRepository (delegación)
#[async_trait]
impl UserRepository for SqliteRepository {
    async fn create_user(&self, username: &str, password_hash: &str, email: Option<&str>) -> Result<User, AppError> {
        self.users.create_user(username, password_hash, email).await
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.users.get_by_username(username).await
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        self.users.get_by_id(id).await
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.users.get_by_email(email).await
    }

    async fn get_all(&self, q: Option<String>, page: i64, limit: i64) -> Result<(Vec<User>, i64), AppError> {
        self.users.get_all(q, page, limit).await
    }

    async fn get_stats(&self) -> Result<(i64, i64, i64), AppError> {
        self.users.get_stats().await
    }

    async fn delete_user(&self, id: i64, admin_username: &str) -> Result<(), AppError> {
        self.users.delete_user(id, admin_username).await
    }

    async fn update_user(&self, id: i64, email: Option<&str>, role: Option<Role>) -> Result<User, AppError> {
        self.users.update_user(id, email, role).await
    }

    async fn update_avatar(&self, id: i64, avatar_url: &str) -> Result<User, AppError> {
        self.users.update_avatar(id, avatar_url).await
    }

    async fn update_password(&self, id: i64, password_hash: &str) -> Result<(), AppError> {
        self.users.update_password(id, password_hash).await
    }

    async fn verify_email(&self, user_id: i64) -> Result<(), AppError> {
        self.users.verify_email(user_id).await
    }
}

// Implementación de TokenRepository (delegación)
#[async_trait]
impl TokenRepository for SqliteRepository {
    async fn create_refresh_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<RefreshToken, AppError> {
        self.tokens.create_refresh_token(user_id, token, expires_at).await
    }

    async fn get_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError> {
        self.tokens.get_refresh_token(token).await
    }

    async fn mark_refresh_token_used(&self, token_id: i64) -> Result<(), AppError> {
        self.tokens.mark_refresh_token_used(token_id).await
    }

    async fn revoke_user_refresh_tokens(&self, user_id: i64) -> Result<(), AppError> {
        self.tokens.revoke_user_refresh_tokens(user_id).await
    }

    async fn create_password_reset_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<PasswordResetToken, AppError> {
        self.tokens.create_password_reset_token(user_id, token, expires_at).await
    }

    async fn get_password_reset_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError> {
        self.tokens.get_password_reset_token(token).await
    }

    async fn mark_password_reset_token_used(&self, token_id: i64) -> Result<(), AppError> {
        self.tokens.mark_password_reset_token_used(token_id).await
    }

    async fn create_email_verification_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<EmailVerificationToken, AppError> {
        self.tokens.create_email_verification_token(user_id, token, expires_at).await
    }

    async fn get_email_verification_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, AppError> {
        self.tokens.get_email_verification_token(token).await
    }

    async fn mark_email_verification_token_used(&self, token_id: i64) -> Result<(), AppError> {
        self.tokens.mark_email_verification_token_used(token_id).await
    }
}

// Implementación de AuditRepository (delegación)
#[async_trait]
impl AuditRepository for SqliteRepository {
    async fn get_audit_logs(&self) -> Result<Vec<AuditLog>, AppError> {
        self.audit.get_audit_logs().await
    }

    async fn create_audit_log(&self, admin_username: &str, action: &str, target: &str) -> Result<(), AppError> {
        self.audit.create_audit_log(admin_username, action, target).await
    }
}

// Implementación de RbacRepository (delegación)
#[async_trait]
impl RbacRepository for SqliteRepository {
    async fn get_roles(&self) -> Result<Vec<DbRole>, AppError> {
        self.rbac.get_roles().await
    }

    async fn get_permissions(&self) -> Result<Vec<Permission>, AppError> {
        self.rbac.get_permissions().await
    }

    async fn get_role_permissions(&self) -> Result<Vec<RolePermission>, AppError> {
        self.rbac.get_role_permissions().await
    }

    async fn create_role(&self, name: &str, description: Option<&str>, permissions: &[i64]) -> Result<DbRole, AppError> {
        self.rbac.create_role(name, description, permissions).await
    }

    async fn update_role(&self, id: i64, name: Option<&str>, description: Option<&str>, permissions: Option<&[i64]>) -> Result<DbRole, AppError> {
        self.rbac.update_role(id, name, description, permissions).await
    }

    async fn delete_role(&self, id: i64) -> Result<(), AppError> {
        self.rbac.delete_role(id).await
    }

    async fn update_permission(&self, id: i64, description: &str) -> Result<Permission, AppError> {
        self.rbac.update_permission(id, description).await
    }
}

// SqliteRepository ya implementa Repository automáticamente
// porque implementa todos los traits individuales
// impl Repository for SqliteRepository {}
