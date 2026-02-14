use axum::{debug_handler, extract::{Path, State, Query}, http::StatusCode, Json, response::IntoResponse};
use sqlx::SqlitePool;
use sqlx::error::ErrorKind;
use crate::core::models::user::{CreateUserRequest, LoginRequest, User, Claims, AuditLog, UserSearch};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tower_cookies::{Cookie, Cookies};
use validator::Validate;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use serde_json::json;

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // 0. Validar inputs antes de procesar
    if let Err(e) = payload.validate() {
        return (StatusCode::BAD_REQUEST, format!("Datos inválidos: {}", e)).into_response();
    }

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
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, role, created_at"
    )
    .bind(&payload.username)
    .bind(password_hash)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            // 2. Manejo de errores específicos (Duplicados)
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    return (StatusCode::CONFLICT, "El nombre de usuario ya existe").into_response();
                }
            }

            tracing::error!("Error creando usuario: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear usuario").into_response()
        }
    }
}

pub async fn get_audit_logs(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<AuditLog>>, (StatusCode, String)> {
    let logs = sqlx::query_as::<_, AuditLog>("SELECT id, admin_username, action, target, timestamp FROM audit_logs ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error obteniendo logs de auditoría: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al consultar la bitácora".to_string())
        })?;

    Ok(Json(logs))
}

pub async fn get_users(
    State(pool): State<SqlitePool>,
    Query(params): Query<UserSearch>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = crate::data::user_repository::get_all(&pool, params.q).await.map_err(|e| {
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
        // GENERAR JWT
        let expiration = Utc::now().checked_add_signed(Duration::hours(24)).expect("Tiempo inválido").timestamp();
        let claims = Claims { 
            sub: user.username, 
            role: user.role, 
            exp: expiration as usize 
        };
        // NOTA: En producción, "secret" debe venir de variables de entorno (.env)
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
        
        cookies.add(Cookie::new("auth_token", token));
        (StatusCode::OK, "Login exitoso").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Credenciales inválidas").into_response()
    }
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("auth_token", ""));
    (StatusCode::OK, "Sesión cerrada correctamente").into_response()
}

pub async fn dashboard(cookies: Cookies) -> impl IntoResponse {
    let cookie = cookies.get("auth_token").map(|c| c.value().to_string()).unwrap_or_default();

    // Decodificar el token para saber quién es
    let token_data = decode::<Claims>(
        &cookie,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()
    );

    match token_data {
        Ok(c) => (StatusCode::OK, Json(json!({
            "username": c.claims.sub,
            "role": c.claims.role,
            "message": format!("🔐 Panel de Control | Agente: {} | Rango: {:?}", c.claims.sub, c.claims.role)
        }))).into_response(),
        Err(_) => (StatusCode::UNAUTHORIZED, "Sesión inválida o expirada").into_response()
    }
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // 1. Identificar al Admin (Auditoría)
    // Aunque el middleware ya validó, necesitamos el username para el log.
    let admin_username = if let Some(cookie) = cookies.get("auth_token") {
        if let Ok(token_data) = decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default()
        ) {
            token_data.claims.sub
        } else {
            "Desconocido".to_string()
        }
    } else {
        "Desconocido".to_string()
    };

    // 2. Obtener datos del objetivo antes de eliminar (para el log)
    let target_username = sqlx::query_scalar::<_, String>("SELECT username FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "Usuario Fantasma".to_string());

    // 3. Ejecutar Eliminación
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            // 4. Registrar en Auditoría
            let _ = sqlx::query("INSERT INTO audit_logs (admin_username, action, target) VALUES ($1, 'DELETE_USER', $2)")
                .bind(admin_username)
                .bind(target_username)
                .execute(&pool)
                .await;

            (StatusCode::OK, "Usuario eliminado y auditado").into_response()
        },
        Err(e) => {
            tracing::error!("Error eliminando usuario: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al eliminar").into_response()
        }
    }
}