use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    #[sqlx(default)] // Maneja casos donde la columna no existía antes (migración suave)
    pub role: Role,
    // Usamos String por simplicidad inicial (SQLite devuelve texto)
    pub created_at: String, 
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "El usuario debe tener al menos 3 caracteres"))]
    pub username: String,
    #[validate(length(min = 8, message = "La contraseña debe tener al menos 8 caracteres"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (Usuario)
    pub role: Role,  // Rango del usuario
    pub exp: usize,  // Expiration
}

#[derive(Debug, Serialize, FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub admin_username: String,
    pub action: String,
    pub target: String,
    pub timestamp: String,
}
