use crate::core::models::user::{
    AuditLog, ChangePasswordRequest, Claims, CreateRoleRequest, CreateUserRequest, DbRole, ForgotPasswordRequest, 
    LoginRequest, Permission, RefreshRequest, ResetPasswordRequest, RolePermission, TokenResponse, UpdatePermissionRequest, UpdateRoleRequest, User, UserSearch, VerifyEmailRequest,
};
use crate::core::models::user::UpdateUserRequest;
use crate::services::email::create_email_service;
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
    
    // Enviar email de verificación si el usuario tiene email
    if let Some(ref email) = payload.email {
        // Generar token de verificación
        let verification_token = uuid::Uuid::new_v4().to_string();
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Tiempo inválido")
            .timestamp();
        
        // Guardar token en base de datos
        let expires_at = chrono::NaiveDateTime::from_timestamp(expiration, 0)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        
        repo.create_email_verification_token(user.id, &verification_token, &expires_at).await?;
        
        // Enviar email (si el servicio está configurado)
        if let Some(email_service) = create_email_service() {
            if let Err(e) = email_service.send_email_verification(
                email,
                &verification_token,
                &payload.username
            ).await {
                tracing::error!("Error enviando email de verificación: {}", e);
            }
        } else {
            // En desarrollo, loguear el token
            tracing::info!("Email verification token para {}: {}", email, verification_token);
            tracing::info!("URL de verificación: http://localhost:4321/verify-email?token={}", verification_token);
        }
    }
    
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
        
        // GENERAR ACCESS TOKEN (15 minutos)
        let access_expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("Tiempo inválido")
            .timestamp();
        let access_claims = Claims {
            sub: username.clone(),
            role: role.clone(),
            exp: access_expiration as usize,
            user_id: user_id,
        };
        
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|_| AppError::AuthError("Error generando access token".to_string()))?;

        // GENERAR REFRESH TOKEN (7 días)
        let refresh_token_str = uuid::Uuid::new_v4().to_string();
        let refresh_expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("Tiempo inválido")
            .timestamp();
        
        // Guardar refresh token en base de datos
        repo.create_refresh_token(
            user_id,
            &refresh_token_str,
            &chrono::NaiveDateTime::from_timestamp(refresh_expiration, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        ).await?;

        // Establecer cookie con access token
        let mut cookie = Cookie::new("auth_token", access_token.clone());
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
            "access_token": access_token,
            "refresh_token": refresh_token_str,
            "expires_in": 15 * 60,
            "token_type": "Bearer",
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

    // Seguridad: Solo los administradores pueden cambiar el rol
    if payload.role.is_some() && requester_role != crate::core::models::user::Role::Admin {
        return Err(AppError::Forbidden("Solo los administradores pueden asignar roles".to_string()));
    }

    let repo = SqliteRepository::new(pool);
    let updated_user = repo.update_user(id, payload.email.as_deref(), payload.role).await?;
    
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

pub async fn refresh_token(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let repo = SqliteRepository::new(pool);
    
    // 1. Buscar el refresh token en la base de datos
    let stored_token = repo.get_refresh_token(&payload.refresh_token).await?;
    
    let stored_token = stored_token.ok_or(AppError::AuthError("Refresh token inválido".to_string()))?;
    
    // 2. Verificar que no haya sido usado
    if stored_token.used {
        return Err(AppError::AuthError("Refresh token ya fue utilizado".to_string()));
    }
    
    // 3. Verificar que no haya expirado
    let expires_at = chrono::NaiveDateTime::parse_from_str(&stored_token.expires_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| AppError::AuthError("Error parseando fecha de expiración".to_string()))?;
    
    let now = chrono::Local::now().naive_local();
    if now > expires_at {
        return Err(AppError::AuthError("Refresh token expirado".to_string()));
    }
    
    // 4. Obtener el usuario
    let user = repo.get_by_id(stored_token.user_id).await?
        .ok_or(AppError::AuthError("Usuario no encontrado".to_string()))?;
    
    // 5. Marcar el refresh token como usado (rotación)
    repo.mark_refresh_token_used(stored_token.id).await?;
    
    // 6. Generar nuevos tokens
    // Access token: 15 minutos
    let access_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("Tiempo inválido")
        .timestamp();
    
    let access_claims = Claims {
        sub: user.username.clone(),
        role: user.role.clone(),
        exp: access_expiration as usize,
        user_id: user.id,
    };
    
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|_| AppError::AuthError("Error generando access token".to_string()))?;
    
    // Refresh token: 7 días
    let refresh_token_str = uuid::Uuid::new_v4().to_string();
    let refresh_expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("Tiempo inválido")
        .timestamp();
    
    // Guardar el nuevo refresh token
    let new_refresh = repo.create_refresh_token(
        user.id, 
        &refresh_token_str,
        &chrono::NaiveDateTime::from_timestamp(refresh_expiration, 0)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    ).await?;
    
    // 7. Actualizar cookies
    let mut cookie = Cookie::new("auth_token", access_token.clone());
    cookie.set_http_only(true);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    cookie.set_path("/");
    cookies.add(cookie);
    
    // 8. Devolver respuesta
    Ok(Json(TokenResponse {
        access_token,
        refresh_token: new_refresh.token,
        expires_in: 15 * 60, // 15 minutos en segundos
        token_type: "Bearer".to_string(),
    }))
}

pub async fn forgot_password(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validar email
    if payload.email.is_empty() || !payload.email.contains('@') {
        return Err(AppError::Validation("Email inválido".to_string()));
    }
    
    let repo = SqliteRepository::new(pool);
    
    // Buscar usuario por email
    let user = match repo.get_by_email(&payload.email).await? {
        Some(u) => u,
        None => {
            // Por seguridad, no revelar si el email existe o no
            return Ok((StatusCode::OK, Json(json!({
                "message": "Si el email existe, recibirás instrucciones para restablecer tu contraseña"
            }))));
        }
    };
    
    // Verificar que el usuario tenga email
    if user.email.is_none() {
        return Ok((StatusCode::OK, Json(json!({
            "message": "Si el email existe, recibirás instrucciones para restablecer tu contraseña"
        }))));
    }
    
    // Generar token único
    let reset_token = uuid::Uuid::new_v4().to_string();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("Tiempo inválido")
        .timestamp();
    
    // Guardar token en base de datos
    let expires_at = chrono::NaiveDateTime::from_timestamp(expiration, 0)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    repo.create_password_reset_token(user.id, &reset_token, &expires_at).await?;
    
    // Enviar email (si el servicio está configurado)
    if let Some(email_service) = create_email_service() {
        if let Err(e) = email_service.send_password_reset(
            &payload.email,
            &reset_token,
            &user.username
        ).await {
            tracing::error!("Error enviando email de recuperación: {}", e);
            // No fallamos la petición, pero logeamos el error
        }
    } else {
        // En desarrollo, logear el token para poder probar
        tracing::info!("Password reset token para {}: {}", payload.email, reset_token);
        tracing::info!("URL de reset: http://localhost:4321/reset-password?token={}", reset_token);
    }
    
    Ok((StatusCode::OK, Json(json!({
        "message": "Si el email existe, recibirás instrucciones para restablecer tu contraseña"
    }))))
}

pub async fn reset_password(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validar contraseña
    if payload.new_password.len() < 8 {
        return Err(AppError::Validation("La contraseña debe tener al menos 8 caracteres".to_string()));
    }
    
    let repo = SqliteRepository::new(pool);
    
    // Buscar el token
    let stored_token = match repo.get_password_reset_token(&payload.token).await? {
        Some(t) => t,
        None => return Err(AppError::AuthError("Token inválido".to_string())),
    };
    
    // Verificar que no haya sido usado
    if stored_token.used {
        return Err(AppError::AuthError("Token ya fue utilizado".to_string()));
    }
    
    // Verificar que no haya expirado
    let expires_at = chrono::NaiveDateTime::parse_from_str(&stored_token.expires_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| AppError::AuthError("Error parseando fecha de expiración".to_string()))?;
    
    let now = chrono::Local::now().naive_local();
    if now > expires_at {
        return Err(AppError::AuthError("Token expirado".to_string()));
    }
    
    // Generar hash de la nueva contraseña
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|e| AppError::AuthError(format!("Error de seguridad: {}", e)))?
        .to_string();
    
    // Actualizar contraseña
    repo.update_password(stored_token.user_id, &password_hash).await?;
    
    // Marcar token como usado
    repo.mark_password_reset_token_used(stored_token.id).await?;
    
    // Revocar todos los refresh tokens del usuario (forzar re-login)
    repo.revoke_user_refresh_tokens(stored_token.user_id).await?;
    
    Ok((StatusCode::OK, Json(json!({
        "message": "Contraseña actualizada correctamente"
    }))))
}

pub async fn send_verification_email(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    // Obtener user_id del token
    let cookie = cookies.get("auth_token").ok_or(AppError::AuthError("No autenticado".to_string()))?;
    let token_data = decode::<Claims>(
        cookie.value(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ).map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

    let user_id = token_data.claims.user_id;
    
    let repo = SqliteRepository::new(pool);
    
    // Obtener usuario
    let user = repo.get_by_id(user_id).await?
        .ok_or(AppError::NotFound("Usuario no encontrado".to_string()))?;
    
    // Verificar que tenga email
    let email = user.email.ok_or(AppError::Validation("El usuario no tiene email".to_string()))?;
    
    // Verificar si ya está verificado
    if user.email_verified {
        return Ok((StatusCode::OK, Json(json!({
            "message": "El email ya está verificado"
        }))));
    }
    
    // Generar token de verificación
    let verification_token = uuid::Uuid::new_v4().to_string();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Tiempo inválido")
        .timestamp();
    
    // Guardar token en base de datos
    let expires_at = chrono::NaiveDateTime::from_timestamp(expiration, 0)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    repo.create_email_verification_token(user_id, &verification_token, &expires_at).await?;
    
    // Enviar email (si el servicio está configurado)
    if let Some(email_service) = create_email_service() {
        if let Err(e) = email_service.send_email_verification(
            &email,
            &verification_token,
            &user.username
        ).await {
            tracing::error!("Error enviando email de verificación: {}", e);
        }
    } else {
        // En desarrollo, loguear el token
        tracing::info!("Email verification token para {}: {}", email, verification_token);
        tracing::info!("URL de verificación: http://localhost:4321/verify-email?token={}", verification_token);
    }
    
    Ok((StatusCode::OK, Json(json!({
        "message": "Email de verificación enviado"
    }))))
}

pub async fn verify_email(
    State(pool): State<SqlitePool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let token = params.get("token")
        .ok_or(AppError::Validation("Token no proporcionado".to_string()))?;
    
    let repo = SqliteRepository::new(pool);
    
    // Buscar el token
    let stored_token = match repo.get_email_verification_token(token).await? {
        Some(t) => t,
        None => return Err(AppError::AuthError("Token inválido".to_string())),
    };
    
    // Verificar que no haya sido usado
    if stored_token.used {
        return Err(AppError::AuthError("Token ya fue utilizado".to_string()));
    }
    
    // Verificar que no haya expirado
    let expires_at = chrono::NaiveDateTime::parse_from_str(&stored_token.expires_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| AppError::AuthError("Error parseando fecha de expiración".to_string()))?;
    
    let now = chrono::Local::now().naive_local();
    if now > expires_at {
        return Err(AppError::AuthError("Token expirado".to_string()));
    }
    
    // Verificar el email del usuario
    repo.verify_email(stored_token.user_id).await?;
    
    // Marcar token como usado
    repo.mark_email_verification_token_used(stored_token.id).await?;
    
    Ok((StatusCode::OK, Json(json!({
        "message": "Email verificado correctamente"
    }))))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Contraseña actualizada"),
        (status = 400, description = "Contraseña actual incorrecta"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn change_password(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validar inputs
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(format!("Datos inválidos: {}", e)));
    }

    // 2. Obtener usuario autenticado
    let cookie = cookies.get("auth_token").ok_or(AppError::AuthError("No autenticado".to_string()))?;
    let token_data = decode::<Claims>(
        cookie.value(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ).map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

    let user_id = token_data.claims.user_id;
    let repo = SqliteRepository::new(pool);

    // 3. Obtener usuario para verificar password actual
    let user = repo.get_by_id(user_id).await?
        .ok_or(AppError::NotFound("Usuario no encontrado".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::AuthError("Error verificando credenciales".to_string()))?;

    if Argon2::default()
        .verify_password(payload.current_password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AppError::Validation("La contraseña actual es incorrecta".to_string()));
    }

    // 4. Hashear nueva password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let new_hash = Argon2::default()
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|e| AppError::AuthError(format!("Error de seguridad: {}", e)))?
        .to_string();

    // 5. Actualizar en DB
    repo.update_password(user_id, &new_hash).await?;

    // 6. Revocar refresh tokens (opcional, pero recomendado por seguridad)
    repo.revoke_user_refresh_tokens(user_id).await?;

    Ok((StatusCode::OK, Json(json!({"message": "Contraseña actualizada correctamente"}))))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles",
    responses(
        (status = 200, description = "Lista de roles del sistema", body = Vec<DbRole>)
    )
)]
pub async fn get_roles(State(pool): State<SqlitePool>) -> Result<Json<Vec<DbRole>>, AppError> {
    let repo = SqliteRepository::new(pool);
    let roles = repo.get_roles().await?;
    Ok(Json(roles))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    responses(
        (status = 200, description = "Lista de permisos del sistema", body = Vec<Permission>)
    )
)]
pub async fn get_permissions(State(pool): State<SqlitePool>) -> Result<Json<Vec<Permission>>, AppError> {
    let repo = SqliteRepository::new(pool);
    let permissions = repo.get_permissions().await?;
    Ok(Json(permissions))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/permissions",
    responses(
        (status = 200, description = "Asociaciones Rol-Permiso", body = Vec<RolePermission>)
    )
)]
pub async fn get_role_permissions(State(pool): State<SqlitePool>) -> Result<Json<Vec<RolePermission>>, AppError> {
    let repo = SqliteRepository::new(pool);
    let rps = repo.get_role_permissions().await?;
    Ok(Json(rps))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Rol creado", body = DbRole),
        (status = 409, description = "El rol ya existe")
    )
)]
pub async fn create_role(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<DbRole>), AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(format!("Datos inválidos: {}", e)));
    }
    let repo = SqliteRepository::new(pool);
    let role = repo.create_role(&payload.name, payload.description.as_deref(), &payload.permissions).await?;
    Ok((StatusCode::CREATED, Json(role)))
}

#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Rol actualizado", body = DbRole)
    )
)]
pub async fn update_role(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<DbRole>, AppError> {
    let repo = SqliteRepository::new(pool);
    let role = repo.update_role(id, payload.name.as_deref(), payload.description.as_deref(), payload.permissions.as_deref()).await?;
    Ok(Json(role))
}

pub async fn delete_role(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    let repo = SqliteRepository::new(pool);
    repo.delete_role(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/api/v1/permissions/{id}",
    request_body = UpdatePermissionRequest,
    responses(
        (status = 200, description = "Permiso actualizado", body = Permission)
    )
)]
pub async fn update_permission(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<Permission>, AppError> {
    let repo = SqliteRepository::new(pool);
    let permission = repo.update_permission(id, &payload.description).await?;
    Ok(Json(permission))
}
