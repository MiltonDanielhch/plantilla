use crate::core::models::user::{
    AuditLog, Claims, CreateUserRequest, LoginRequest, User, UserSearch,
};
use crate::core::models::user::UpdateUserRequest;
use crate::core::repository::UserRepository;
use crate::data::user_repository::SqliteRepository;
use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    debug_handler,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::json;
use sqlx::SqlitePool;
use tower_cookies::{Cookie, Cookies};
use validator::Validate;
use csv::WriterBuilder;

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
    let user = repo.create_user(&payload.username, &password_hash, payload.email.as_deref()).await?;
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
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = SqliteRepository::new(pool);
    let (users, total) = repo.get_all(params.q, params.page, params.limit).await?;
    
    let total_pages = (total as f64 / params.limit as f64).ceil() as i64;

    Ok(Json(json!({
        "data": users,
        "meta": {
            "total": total,
            "page": params.page,
            "limit": params.limit,
            "totalPages": total_pages
        }
    })))
}

#[utoipa::path(
    get,
    path = "/api/v1/stats",
    responses(
        (status = 200, description = "Estadísticas del dashboard")
    )
)]
pub async fn get_stats(
    State(pool): State<SqlitePool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = SqliteRepository::new(pool);
    let (total, admins, new_today) = repo.get_stats().await?;

    Ok(Json(json!({
        "total_users": total,
        "active_users": total,
        "admin_users": admins,
        "new_users_today": new_today
    })))
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
            user_id: user_id,
        };
        // NOTA: En producción, "secret" debe venir de variables de entorno (.env)
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|_| AppError::AuthError("Error generando token".to_string()))?;

        let mut cookie = Cookie::new("auth_token", token.clone());
        cookie.set_http_only(true);
        cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
        cookie.set_path("/");
        cookies.add(cookie);
        Ok((StatusCode::OK, Json(json!({
            "user": {
                "id": user_id,
                "username": username,
                "role": role
            },
            "token": token,
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
    let mut cookie = Cookie::new("auth_token", "");
    cookie.set_path("/");
    cookies.remove(cookie);
    (StatusCode::OK, Json(json!({"message": "Sesión cerrada correctamente"}))).into_response()
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
                "user": {
                    "id": c.claims.user_id,
                    "username": c.claims.sub,
                    "role": c.claims.role
                },
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

    Ok((StatusCode::OK, Json(json!({"message": "Usuario eliminado y auditado"}))))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    params(("id" = i64, Path, description = "ID del usuario a actualizar")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Usuario actualizado", body = User),
        (status = 403, description = "No tienes permiso para editar este usuario"),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn update_user(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    // 1. Validar inputs
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(format!("Datos inválidos: {}", e)));
    }

    // 2. Verificar permisos (Solo el propio usuario o un Admin pueden editar)
    let cookie = cookies.get("auth_token").ok_or(AppError::AuthError("No autenticado".to_string()))?;
    let token_data = decode::<Claims>(
        cookie.value(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ).map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

    let requester_id = token_data.claims.user_id;
    let requester_role = token_data.claims.role;

    // Si no es Admin Y no es el dueño de la cuenta -> Prohibido
    if requester_role != crate::core::models::user::Role::Admin && requester_id != id {
        return Err(AppError::Forbidden("No puedes editar otros usuarios".to_string()));
    }

    let repo = SqliteRepository::new(pool);
    let updated_user = repo.update_user(id, payload.email.as_deref()).await?;
    
    Ok(Json(updated_user))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(("id" = i64, Path, description = "ID del usuario")),
    responses(
        (status = 200, description = "Detalle del usuario", body = User),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn get_user_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let repo = SqliteRepository::new(pool);
    let user = repo.get_by_id(id).await?.ok_or(AppError::NotFound("Usuario no encontrado".to_string()))?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/export",
    responses(
        (status = 200, description = "Exportación CSV de usuarios", content_type = "text/csv"),
        (status = 500, description = "Error al generar CSV")
    )
)]
pub async fn export_users(
    State(pool): State<SqlitePool>,
) -> Result<Response, AppError> {
    let repo = SqliteRepository::new(pool);
    let (users, _total) = repo.get_all(None, 1, 10000).await?; // Obtener todos los usuarios
    
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);
    
    // Escribir encabezados
    wtr.write_record(&["ID", "Username", "Email", "Role", "Created At"])
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    // Escribir datos
    for user in users {
        wtr.write_record(&[
            user.id.to_string(),
            user.username,
            user.email.unwrap_or_default(),
            format!("{:?}", user.role),
            user.created_at,
        ]).map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    }
    
    let data = wtr.into_inner()
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"users_export.csv\"")
        .body(Body::from(data))
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    Ok(response)
}

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs/export",
    responses(
        (status = 200, description = "Exportación CSV de logs de auditoría", content_type = "text/csv"),
        (status = 500, description = "Error al generar CSV")
    )
)]
pub async fn export_audit_logs(
    State(pool): State<SqlitePool>,
) -> Result<Response, AppError> {
    let repo = SqliteRepository::new(pool);
    let logs = repo.get_audit_logs().await?;
    
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);
    
    // Escribir encabezados
    wtr.write_record(&["ID", "Admin Username", "Action", "Target", "Timestamp"])
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    // Escribir datos
    for log in logs {
        wtr.write_record(&[
            log.id.to_string(),
            log.admin_username,
            log.action,
            log.target,
            log.timestamp,
        ]).map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    }
    
    let data = wtr.into_inner()
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"audit_logs_export.csv\"")
        .body(Body::from(data))
        .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))))?;
    
    Ok(response)
}

pub async fn upload_avatar(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    mut multipart: Multipart,
) -> Result<Json<User>, AppError> {
    // 1. Obtener user_id del token
    let cookie = cookies.get("auth_token").ok_or(AppError::AuthError("No autenticado".to_string()))?;
    let token_data = decode::<Claims>(
        cookie.value(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ).map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

    let user_id = token_data.claims.user_id;

    // 2. Procesar el archivo
    let mut file_data: Option<(String, Vec<u8>, String)> = None;
    
    while let Some(mut field) = multipart.next_field().await
        .map_err(|e| AppError::Validation(format!("Error leyendo formulario: {}", e)))? 
    {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "avatar" {
            let filename = field.file_name().unwrap_or("avatar.jpg").to_string();
            let content_type = field.content_type().unwrap_or("image/jpeg").to_string();
            
            // Validar que sea una imagen
            if !content_type.starts_with("image/") {
                return Err(AppError::Validation("El archivo debe ser una imagen".to_string()));
            }
            
            let data = field.bytes().await
                .map_err(|e| AppError::Validation(format!("Error leyendo archivo: {}", e)))?;
            
            // Validar tamaño (máximo 2MB)
            if data.len() > 2 * 1024 * 1024 {
                return Err(AppError::Validation("El archivo es demasiado grande (máximo 2MB)".to_string()));
            }
            
            file_data = Some((filename, data.to_vec(), content_type));
        }
    }
    
    let (filename, data, _content_type) = file_data.ok_or(
        AppError::Validation("No se proporcionó archivo de avatar".to_string())
    )?;
    
    // 3. Crear directorio uploads si no existe
    let upload_dir = std::path::Path::new("uploads");
    if !upload_dir.exists() {
        std::fs::create_dir_all(upload_dir)
            .map_err(|e| AppError::Database(sqlx::Error::Io(e)))?;
    }
    
    // 4. Generar nombre único para el archivo
    let timestamp = Utc::now().timestamp();
    let extension = std::path::Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");
    let new_filename = format!("{}_{}.{}", user_id, timestamp, extension);
    let file_path = upload_dir.join(&new_filename);
    
    // 5. Guardar archivo
    std::fs::write(&file_path, &data)
        .map_err(|e| AppError::Database(sqlx::Error::Io(e)))?;
    
    // 6. Actualizar URL en la base de datos
    let avatar_url = format!("/uploads/{}", new_filename);
    let repo = SqliteRepository::new(pool);
    let user = repo.update_avatar(user_id, &avatar_url).await?;
    
    Ok(Json(user))
}
