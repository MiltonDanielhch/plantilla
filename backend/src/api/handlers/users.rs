use crate::api::handlers::common::extract_claims;
use crate::core::container::AppState;
use crate::core::models::user::{CreateUserRequest, UpdateUserRequest, UserSearch};
use crate::core::services::user_service::AvatarData;
use crate::error::AppError;
use crate::services::email::create_email_service;
use axum::extract::{Json, Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use tower_cookies::Cookies;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Usuario creado exitosamente", body = crate::core::models::user::User),
        (status = 409, description = "El usuario ya existe"),
        (status = 400, description = "Datos inválidos")
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<crate::core::models::user::User>), AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

    let service = state.user_service();
    let user = service.create_user(payload).await?;
    
    // Enviar email de verificación si tiene email
    if let Some(ref email) = user.email {
        send_verification_email(email, &user.username).await;
    }

    Ok((StatusCode::CREATED, Json(user)))
}

async fn send_verification_email(email: &str, username: &str) {
    if let Some(service) = create_email_service() {
        tracing::info!("Enviando email de verificación a {}", email);
        if let Err(e) = service.send_email_verification(email, "token", username).await {
            tracing::error!("Error enviando email: {}", e);
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(UserSearch),
    responses(
        (status = 200, description = "Lista de usuarios registrados", body = Vec<crate::core::models::user::User>)
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<UserSearch>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = params.page;
    let limit = params.limit;
    
    let service = state.user_service();
    let (users, total, total_pages) = service.get_users(params).await?;

    Ok(Json(json!({
        "data": users,
        "meta": {
            "total": total,
            "page": page,
            "limit": limit,
            "totalPages": total_pages
        }
    })))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(("id" = i64, Path, description = "ID del usuario")),
    responses(
        (status = 200, description = "Detalle del usuario", body = crate::core::models::user::User),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<crate::core::models::user::User>, AppError> {
    let service = state.user_service();
    let user = service.get_user(id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    params(("id" = i64, Path, description = "ID del usuario a actualizar")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Usuario actualizado", body = crate::core::models::user::User),
        (status = 403, description = "No tienes permiso para editar este usuario"),
        (status = 404, description = "Usuario no encontrado")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<crate::core::models::user::User>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

    let cookie = cookies
        .get("auth_token")
        .ok_or_else(|| AppError::AuthError("No autenticado".to_string()))?;
    let claims = extract_claims(cookie.value())?;

    let service = state.user_service();
    let user = service
        .update_user(id, payload, claims.user_id, claims.role)
        .await?;

    Ok(Json(user))
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
    State(state): State<AppState>,
    cookies: Cookies,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get("auth_token")
        .ok_or_else(|| AppError::AuthError("No autenticado".to_string()))?;
    let claims = extract_claims(cookie.value())?;

    let service = state.user_service();
    service
        .delete_user(id, claims.user_id, claims.role, &claims.sub)
        .await?;

    if claims.user_id == id {
        remove_auth_cookie(&cookies);
    }

    Ok((
        StatusCode::OK,
        Json(json!({"message": "Usuario eliminado y auditado"})),
    ))
}

fn remove_auth_cookie(cookies: &Cookies) {
    let mut cookie = tower_cookies::Cookie::new("auth_token", "");
    cookie.set_path("/");
    cookies.remove(cookie);
}

pub async fn upload_avatar(
    State(state): State<AppState>,
    cookies: Cookies,
    mut multipart: Multipart,
) -> Result<Json<crate::core::models::user::User>, AppError> {
    let cookie = cookies
        .get("auth_token")
        .ok_or_else(|| AppError::AuthError("No autenticado".to_string()))?;
    let claims = extract_claims(cookie.value())?;

    let avatar_data = extract_avatar_file(&mut multipart).await?;

    let service = state.user_service();
    let user = service.update_avatar(claims.user_id, avatar_data).await?;

    Ok(Json(user))
}

async fn extract_avatar_file(multipart: &mut Multipart) -> Result<AvatarData, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Error leyendo formulario: {}", e)))?
    {
        if field.name().unwrap_or("") == "avatar" {
            let filename = field.file_name().unwrap_or("avatar.jpg").to_string();
            let content_type = field.content_type().unwrap_or("image/jpeg").to_string();

            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::Validation(format!("Error leyendo archivo: {}", e)))?
                .to_vec();

            let avatar_data = AvatarData {
                filename,
                data,
                content_type,
            };

            avatar_data.validate()?;
            return Ok(avatar_data);
        }
    }

    Err(AppError::Validation("No se proporcionó archivo de avatar".to_string()))
}
