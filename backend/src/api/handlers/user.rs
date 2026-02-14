use axum::{debug_handler, extract::State, http::StatusCode, Json, response::IntoResponse};
use sqlx::SqlitePool;
use crate::core::models::user::{CreateUserRequest, LoginRequest, User};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tower_cookies::{Cookie, Cookies};

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // 1. Generar Salt y Hash seguro
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            tracing::error!("Error hasheando password: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error de seguridad").into_response();
        }
    };

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, created_at"
    )
    .bind(&payload.username)
    .bind(password_hash)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            // En un sistema real, manejaríamos mejor los errores (ej. duplicados)
            tracing::error!("Error creando usuario: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear usuario").into_response()
        }
    }
}

pub async fn get_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = crate::data::user_repository::get_all(&pool).await.map_err(|e| {
        tracing::error!("Error de sintonía al leer usuarios: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Fallo en la matriz de datos".to_string())
    })?;

    Ok(Json(users))
}

#[debug_handler]
pub async fn login(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // 1. Buscar usuario en DB
    let user = match crate::data::user_repository::get_by_username(&pool, &payload.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Credenciales inválidas").into_response(),
        Err(e) => {
            tracing::error!("Error buscando usuario: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error interno").into_response();
        }
    };

    // 2. Verificar password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error de datos").into_response(),
    };

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
        // Crear cookie de sesión (temporalmente un valor fijo)
        cookies.add(Cookie::new("auth_token", "token-secreto-temporal"));
        (StatusCode::OK, "Login exitoso").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Credenciales inválidas").into_response()
    }
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("auth_token", ""));
    (StatusCode::OK, "Sesión cerrada correctamente").into_response()
}

pub async fn dashboard() -> impl IntoResponse {
    (StatusCode::OK, "🔐 Bienvenido al Panel de Control (Acceso Autorizado)")
}