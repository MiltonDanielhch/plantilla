use crate::api::handlers::common::extract_claims;
use crate::core::container::AppState;
use crate::core::models::user::{
    Claims, ForgotPasswordRequest, LoginRequest, RefreshRequest, ResetPasswordRequest,
};
use crate::error::AppError;
use crate::services::email::create_email_service;
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use std::collections::HashMap;
use tower_cookies::Cookies;
use validator::Validate;

// ============ HANDLERS ============

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
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = state.auth_service();
    let (user, access_token, refresh_token) = service
        .authenticate(&payload.username, &payload.password)
        .await?;

    set_auth_cookie(&cookies, &access_token);

    Ok((
        StatusCode::OK,
        Json(json!({
            "user": {
                "id": user.id,
                "username": user.username,
                "role": user.role
            },
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": 15 * 60,
            "token_type": "Bearer",
            "message": "Login exitoso"
        })),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    responses((status = 200, description = "Sesión cerrada correctamente"))
)]
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    remove_auth_cookie(&cookies);
    (
        StatusCode::OK,
        Json(json!({"message": "Sesión cerrada correctamente"})),
    )
}

pub async fn refresh_token(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = state.auth_service();
    let response = service.refresh_access_token(&payload.refresh_token).await?;

    set_auth_cookie(&cookies, &response.access_token);
    Ok((StatusCode::OK, Json(response)))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = state.auth_service();

    if let Some(token) = service.initiate_password_reset(&payload.email).await? {
        send_reset_email(&payload.email, &token).await;
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Si el email existe, recibirás instrucciones para restablecer tu contraseña"
        })),
    ))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

    let service = state.auth_service();
    service.reset_password(&payload.token, &payload.new_password).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"message": "Contraseña actualizada correctamente"})),
    ))
}

pub async fn send_verification_email(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let claims = extract_claims_from_cookies(&cookies)?;
    let service = state.auth_service();

    match service.initiate_email_verification(claims.user_id).await? {
        Some(token) => {
            // Obtener usuario para el email
            let user_service = state.user_service();
            let user = user_service.get_user(claims.user_id).await?;
            
            if let Some(email) = user.email {
                send_verification_email_impl(&email, &token, &user.username).await;
            }
            
            Ok((
                StatusCode::OK,
                Json(json!({"message": "Email de verificación enviado"})),
            ))
        }
        None => Ok((
            StatusCode::OK,
            Json(json!({"message": "El email ya está verificado"})),
        )),
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let token = params
        .get("token")
        .ok_or_else(|| AppError::Validation("Token no proporcionado".to_string()))?;

    let service = state.auth_service();
    service.verify_email(token).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"message": "Email verificado correctamente"})),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/password",
    request_body = crate::core::models::user::ChangePasswordRequest,
    responses(
        (status = 200, description = "Contraseña actualizada"),
        (status = 400, description = "Contraseña actual incorrecta"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<crate::core::models::user::ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

    let claims = extract_claims_from_cookies(&cookies)?;
    let service = state.auth_service();

    service
        .change_password(claims.user_id, &payload.current_password, &payload.new_password)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({"message": "Contraseña actualizada correctamente"})),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/logout-all",
    responses((status = 200, description = "Todas las sesiones cerradas correctamente"))
)]
pub async fn logout_all(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let claims = extract_claims_from_cookies(&cookies)?;
    let service = state.auth_service();

    service.logout_all_sessions(claims.user_id).await?;
    remove_auth_cookie(&cookies);

    Ok((
        StatusCode::OK,
        Json(json!({"message": "Todas las sesiones han sido cerradas"})),
    ))
}

// ============ HELPERS ============

fn extract_claims_from_cookies(cookies: &Cookies) -> Result<Claims, AppError> {
    let cookie = cookies
        .get("auth_token")
        .ok_or_else(|| AppError::AuthError("No autenticado".to_string()))?;
    extract_claims(cookie.value())
}

fn set_auth_cookie(cookies: &Cookies, token: &str) {
    let mut cookie = tower_cookies::Cookie::new("auth_token", token.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    cookie.set_path("/");
    cookies.add(cookie);
}

fn remove_auth_cookie(cookies: &Cookies) {
    let mut cookie = tower_cookies::Cookie::new("auth_token", "");
    cookie.set_path("/");
    cookies.remove(cookie);
}

async fn send_reset_email(email: &str, token: &str) {
    if let Some(service) = create_email_service() {
        if let Err(e) = service.send_password_reset(email, token, "Usuario").await {
            tracing::error!("Error enviando email de recuperación: {}", e);
        }
    } else {
        tracing::info!("Password reset token para {}: {}", email, token);
        tracing::info!("URL de reset: http://localhost:4321/reset-password?token={}", token);
    }
}

async fn send_verification_email_impl(email: &str, token: &str, username: &str) {
    if let Some(service) = create_email_service() {
        if let Err(e) = service.send_email_verification(email, token, username).await {
            tracing::error!("Error enviando email de verificación: {}", e);
        }
    } else {
        tracing::info!("Email verification token para {}: {}", email, token);
        tracing::info!("URL de verificación: http://localhost:4321/verify-email?token={}", token);
    }
}
