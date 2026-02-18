use crate::core::models::user::{
    EmailVerificationToken, PasswordResetToken, RefreshToken
};
use crate::core::repository::TokenRepository;
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteTokenRepository {
    pool: SqlitePool,
}

impl SqliteTokenRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepository for SqliteTokenRepository {
    // Refresh Tokens
    async fn create_refresh_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<RefreshToken, AppError> {
        sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, expires_at, created_at, used"
        )
        .bind(user_id)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError> {
        sqlx::query_as::<_, RefreshToken>(
            "SELECT id, user_id, token, expires_at, created_at, used FROM refresh_tokens WHERE token = $1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn mark_refresh_token_used(&self, token_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE refresh_tokens SET used = TRUE WHERE id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn revoke_user_refresh_tokens(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    // Password Reset Tokens
    async fn create_password_reset_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<PasswordResetToken, AppError> {
        sqlx::query_as::<_, PasswordResetToken>(
            "INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, expires_at, created_at, used"
        )
        .bind(user_id)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_password_reset_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError> {
        sqlx::query_as::<_, PasswordResetToken>(
            "SELECT id, user_id, token, expires_at, created_at, used FROM password_reset_tokens WHERE token = $1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn mark_password_reset_token_used(&self, token_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE password_reset_tokens SET used = TRUE WHERE id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    // Email Verification Tokens
    async fn create_email_verification_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<EmailVerificationToken, AppError> {
        sqlx::query_as::<_, EmailVerificationToken>(
            "INSERT INTO email_verification_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, expires_at, created_at, used"
        )
        .bind(user_id)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_email_verification_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, AppError> {
        sqlx::query_as::<_, EmailVerificationToken>(
            "SELECT id, user_id, token, expires_at, created_at, used FROM email_verification_tokens WHERE token = $1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn mark_email_verification_token_used(&self, token_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE email_verification_tokens SET used = TRUE WHERE id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}
