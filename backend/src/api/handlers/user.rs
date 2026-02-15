use crate::core::models::user::{
    AuditLog, Claims, CreateUserRequest, LoginRequest, User, UserSearch,
};
use crate::core::repository::UserRepository;
use crate::data::user_repository::SqliteRepository;
use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::json;
use sqlx::SqlitePool;
use tower_cookies::{Cookie, Cookies};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Usuario creado exitosamente", body = User),
        (status = 409, description = "El usuario ya existe"),
        (status = 400, description = "Datos inválidos")
    )
)]
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    // 0. Validar inputs antes de procesar
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(format!("Datos inválidos: {}", e)));
    }

    // 1. Generar Salt y Hash seguro
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::AuthError(format!("Error de seguridad: {}", e)))?
        .to_string();

    let repo = SqliteRepository::new(pool);
    let user = repo.create_user(&payload.username, &password_hash).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs",
    responses(
        (status = 200, description = "Bitácora de auditoría del sistema", body = Vec<AuditLog>)
    )
)]
pub async fn get_audit_logs(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    let repo = SqliteRepository::new(pool);
    let logs = repo.get_audit_logs().await?;
    Ok(Json(logs))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(UserSearch),
    responses(
        (status = 200, description = "Lista de usuarios registrados", body = Vec<User>)
    )
)]
pub async fn get_users(
    State(pool): State<SqlitePool>,
    Query(params): Query<UserSearch>,
) -> Result<Json<Vec<User>>, AppError> {
    let repo = SqliteRepository::new(pool);
    let users = repo.get_all(params.q, params.page, params.limit).await?;
    Ok(Json(users))
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login exitoso (Cookie establecida)"),
        (status = 401, description = "Credenciales inválidas")
    )
)]
pub async fn login(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Buscar usuario en DB
    let repo = SqliteRepository::new(pool);
    let user = repo
        .get_by_username(&payload.username)
        .await?
        .ok_or(AppError::AuthError("Credenciales inválidas".to_string()))?;

    // 2. Verificar password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::AuthError("Error verificando credenciales".to_string()))?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        // Guardar datos antes de mover
        let user_id = user.id;
        let username = user.username.clone();
        let role = user.role.clone();
        
        // GENERAR JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Tiempo inválido")
            .timestamp();
        let claims = Claims {
            sub: username.clone(),
            role: role.clone(),
            exp: expiration as usize,
        };
        // NOTA: En producción, "secret" debe venir de variables de entorno (.env)
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|_| AppError::AuthError("Error generando token".to_string()))?;

        cookies.add(Cookie::new("auth_token", token));
        Ok((StatusCode::OK, Json(json!({
            "user": {
                "id": user_id,
                "username": username,
                "role": role
            },
            "message": "Login exitoso"
        }))))
    } else {
        Err(AppError::AuthError("Credenciales inválidas".to_string()))
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    responses(
        (status = 200, description = "Sesión cerrada correctamente")
    )
)]
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("auth_token", ""));
    (StatusCode::OK, "Sesión cerrada correctamente").into_response()
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard",
    responses(
        (status = 200, description = "Información del usuario actual")
    )
)]
pub async fn dashboard(cookies: Cookies) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get("auth_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    // Decodificar el token para saber quién es
    let token_data = decode::<Claims>(
        &cookie,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match token_data {
        Ok(c) => Ok((
            StatusCode::OK,
            Json(json!({
                "username": c.claims.sub,
                "role": c.claims.role,
                "message": format!("🔐 Panel de Control | Agente: {} | Rango: {:?}", c.claims.sub, c.claims.role)
            })),
        )),
        Err(_) => Err(AppError::AuthError(
            "Sesión inválida o expirada".to_string(),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    params(("id" = i64, Path, description = "ID del usuario a eliminar")),
    responses(
        (status = 200, description = "Usuario eliminado y auditado"),
        (status = 401, description = "No autorizado")
    )
)]
pub async fn delete_user(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Identificar al Admin (Auditoría)
    // Aunque el middleware ya validó, necesitamos el username para el log.
    let admin_username = if let Some(cookie) = cookies.get("auth_token") {
        if let Ok(token_data) = decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        ) {
            token_data.claims.sub
        } else {
            "Desconocido".to_string()
        }
    } else {
        "Desconocido".to_string()
    };

    let repo = SqliteRepository::new(pool);
    repo.delete_user(id, &admin_username).await?;

    Ok((StatusCode::OK, "Usuario eliminado y auditado"))
}
