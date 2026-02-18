use crate::core::models::user::{
    Claims, EmailVerificationToken, PasswordResetToken, RefreshToken, TokenResponse, User,
};
use crate::core::repository::Repository;
use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct AuthService<R: Repository> {
    pub repository: R,
    jwt_secret: String,
}

impl<R: Repository> AuthService<R> {
    pub fn new(repository: R, jwt_secret: String) -> Self {
        Self {
            repository,
            jwt_secret,
        }
    }

    // ============ AUTENTICACIÓN ============

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(User, String, String), AppError> {
        let user = self
            .repository
            .get_by_username(username)
            .await?
            .ok_or_else(|| AppError::AuthError("Credenciales inválidas".to_string()))?;

        self.verify_password(&user.password_hash, password)?;

        let access_token = self.generate_access_token(&user)?;
        let refresh_token = self.create_refresh_token(user.id).await?;

        Ok((user, access_token, refresh_token))
    }

    fn verify_password(&self, hash: &str, password: &str) -> Result<(), AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::AuthError("Error verificando credenciales".to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::AuthError("Credenciales inválidas".to_string()))
    }

    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::AuthError(format!("Error de seguridad: {}", e)))
            .map(|h| h.to_string())
    }

    // ============ TOKENS ============

    pub fn generate_access_token(&self, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .ok_or_else(|| AppError::AuthError("Tiempo inválido".to_string()))?
            .timestamp();

        let claims = Claims {
            sub: user.username.clone(),
            role: user.role.clone(),
            exp: expiration as usize,
            user_id: user.id,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::AuthError("Error generando token".to_string()))
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::AuthError("Token inválido".to_string()))
    }

    async fn create_refresh_token(&self, user_id: i64) -> Result<String, AppError> {
        let token_str = uuid::Uuid::new_v4().to_string();
        let expires_at = self.format_expiration(Duration::days(7));

        self.repository
            .create_refresh_token(user_id, &token_str, &expires_at)
            .await?;

        Ok(token_str)
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, AppError> {
        let stored = self.validate_refresh_token(refresh_token).await?;
        let user = self.get_user_by_id(stored.user_id).await?;

        self.repository.mark_refresh_token_used(stored.id).await?;

        let access_token = self.generate_access_token(&user)?;
        let new_refresh = self.create_refresh_token(user.id).await?;

        Ok(TokenResponse {
            access_token,
            refresh_token: new_refresh,
            expires_in: 15 * 60,
            token_type: "Bearer".to_string(),
        })
    }

    async fn validate_refresh_token(&self, token: &str) -> Result<RefreshToken, AppError> {
        let stored = self
            .repository
            .get_refresh_token(token)
            .await?
            .ok_or_else(|| AppError::AuthError("Refresh token inválido".to_string()))?;

        if stored.used {
            return Err(AppError::AuthError(
                "Refresh token ya fue utilizado".to_string(),
            ));
        }

        self.check_token_expiration(&stored.expires_at)?;
        Ok(stored)
    }

    // ============ PASSWORD RESET ============

    pub async fn initiate_password_reset(&self, email: &str) -> Result<Option<String>, AppError> {
        if !self.is_valid_email(email) {
            return Err(AppError::Validation("Email inválido".to_string()));
        }

        let user = match self.repository.get_by_email(email).await? {
            Some(u) => u,
            None => return Ok(None),
        };

        if user.email.is_none() {
            return Ok(None);
        }

        let token = self.create_password_reset_token(user.id).await?;
        Ok(Some(token))
    }

    async fn create_password_reset_token(&self, user_id: i64) -> Result<String, AppError> {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = self.format_expiration(Duration::hours(1));

        self.repository
            .create_password_reset_token(user_id, &token, &expires_at)
            .await?;

        Ok(token)
    }

    pub async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        self.validate_password_strength(new_password)?;

        let stored = self.validate_password_reset_token(token).await?;
        let hash = self.hash_password(new_password)?;

        self.repository.update_password(stored.user_id, &hash).await?;
        self.repository
            .mark_password_reset_token_used(stored.id)
            .await?;
        self.repository
            .revoke_user_refresh_tokens(stored.user_id)
            .await?;

        Ok(())
    }

    async fn validate_password_reset_token(
        &self,
        token: &str,
    ) -> Result<PasswordResetToken, AppError> {
        let stored = self
            .repository
            .get_password_reset_token(token)
            .await?
            .ok_or_else(|| AppError::AuthError("Token inválido".to_string()))?;

        if stored.used {
            return Err(AppError::AuthError("Token ya fue utilizado".to_string()));
        }

        self.check_token_expiration(&stored.expires_at)?;
        Ok(stored)
    }

    // ============ EMAIL VERIFICATION ============

    pub async fn initiate_email_verification(
        &self,
        user_id: i64,
    ) -> Result<Option<String>, AppError> {
        let user = self.get_user_by_id(user_id).await?;

        if user.email_verified {
            return Ok(None);
        }

        if user.email.is_none() {
            return Err(AppError::Validation("El usuario no tiene email".to_string()));
        }

        let token = self.create_email_verification_token(user_id).await?;
        Ok(Some(token))
    }

    async fn create_email_verification_token(&self, user_id: i64) -> Result<String, AppError> {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = self.format_expiration(Duration::hours(24));

        self.repository
            .create_email_verification_token(user_id, &token, &expires_at)
            .await?;

        Ok(token)
    }

    pub async fn verify_email(&self, token: &str) -> Result<(), AppError> {
        let stored = self.validate_email_verification_token(token).await?;

        self.repository.verify_email(stored.user_id).await?;
        self.repository
            .mark_email_verification_token_used(stored.id)
            .await?;

        Ok(())
    }

    async fn validate_email_verification_token(
        &self,
        token: &str,
    ) -> Result<EmailVerificationToken, AppError> {
        let stored = self
            .repository
            .get_email_verification_token(token)
            .await?
            .ok_or_else(|| AppError::AuthError("Token inválido".to_string()))?;

        if stored.used {
            return Err(AppError::AuthError("Token ya fue utilizado".to_string()));
        }

        self.check_token_expiration(&stored.expires_at)?;
        Ok(stored)
    }

    // ============ CHANGE PASSWORD ============

    pub async fn change_password(
        &self,
        user_id: i64,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        self.validate_password_strength(new_password)?;

        let user = self.get_user_by_id(user_id).await?;
        self.verify_password(&user.password_hash, current_password)
            .map_err(|_| AppError::Validation("La contraseña actual es incorrecta".to_string()))?;

        let hash = self.hash_password(new_password)?;
        self.repository.update_password(user_id, &hash).await?;
        self.repository.revoke_user_refresh_tokens(user_id).await?;

        Ok(())
    }

    // ============ UTILIDADES ============

    pub async fn logout_all_sessions(&self, user_id: i64) -> Result<(), AppError> {
        self.repository.revoke_user_refresh_tokens(user_id).await
    }

    async fn get_user_by_id(&self, user_id: i64) -> Result<User, AppError> {
        self.repository
            .get_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))
    }

    fn format_expiration(&self, duration: Duration) -> String {
        Utc::now()
            .checked_add_signed(duration)
            .unwrap()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }

    fn check_token_expiration(&self, expires_at: &str) -> Result<(), AppError> {
        let expiration = chrono::NaiveDateTime::parse_from_str(expires_at, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| AppError::AuthError("Error parseando fecha".to_string()))?;

        if chrono::Local::now().naive_local() > expiration {
            return Err(AppError::AuthError("Token expirado".to_string()));
        }
        Ok(())
    }

    fn is_valid_email(&self, email: &str) -> bool {
        !email.is_empty() && email.contains('@')
    }

    fn validate_password_strength(&self, password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::Validation(
                "La contraseña debe tener al menos 8 caracteres".to_string(),
            ));
        }
        Ok(())
    }
}

// Tests unitarios se agregarán en fase posterior con mocks apropiados
