use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, ToSchema, Default)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Role {
    Admin,
    #[default]
    User,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[sqlx(default)]
    pub email: Option<String>,
    #[serde(skip)]
    pub password_hash: String,
    #[sqlx(default)] // Maneja casos donde la columna no existía antes (migración suave)
    pub role: Role,
    #[sqlx(default)]
    pub avatar_url: Option<String>,
    // Usamos String por simplicidad inicial (SQLite devuelve texto)
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "El usuario debe tener al menos 3 caracteres"))]
    pub username: String,
    #[validate(email(message = "Formato de email inválido"))]
    pub email: Option<String>,
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Formato de email inválido"))]
    pub email: Option<String>,
    // Aquí podríamos agregar password o avatar en el futuro
}

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    10
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct UserSearch {
    pub q: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (Usuario)
    pub role: Role,   // Rango del usuario
    pub exp: usize,   // Expiration
    pub user_id: i64, // ID del usuario
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct AuditLog {
    pub id: i64,
    pub admin_username: String,
    pub action: String,
    pub target: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires_at: String,
    pub created_at: String,
    pub used: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // segundos
    pub token_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_role_default_is_user() {
        assert_eq!(Role::default(), Role::User);
    }

    #[test]
    fn test_create_user_validation() {
        // Caso 1: Datos válidos
        let req = CreateUserRequest {
            username: "usuario_valido".to_string(),
            email: Some("test@example.com".to_string()),
            password: "passwordSeguro123".to_string(),
        };
        assert!(req.validate().is_ok());

        // Caso 2: Usuario muy corto
        let req_bad_user = CreateUserRequest {
            username: "yo".to_string(),
            email: None,
            password: "passwordSeguro123".to_string(),
        };
        assert!(req_bad_user.validate().is_err());

        // Caso 3: Password muy corto
        let req_bad_pass = CreateUserRequest {
            username: "usuario_valido".to_string(),
            email: None,
            password: "123".to_string(),
        };
        assert!(req_bad_pass.validate().is_err());
    }
}
