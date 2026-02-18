use crate::api::handlers::common::extract_claims;
use crate::core::container::AppState;
use crate::error::AppError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tower_cookies::Cookies;

#[utoipa::path(
    get,
    path = "/api/v1/dashboard",
    responses((status = 200, description = "Información del usuario actual"))
)]
pub async fn dashboard(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get("auth_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    match extract_claims(&cookie) {
        Ok(claims) => load_user_dashboard(state, claims).await,
        Err(_) => Err(AppError::AuthError(
            "Sesión inválida o expirada".to_string(),
        )),
    }
}

async fn load_user_dashboard(
    state: AppState,
    claims: crate::core::models::user::Claims,
) -> Result<impl IntoResponse, AppError> {
    let service = state.user_service();
    let user = service.get_user(claims.user_id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "user": user,
            "message": format!(
                "🔐 Panel de Control | Agente: {} | Rango: {:?}",
                user.username, user.role
            )
        })),
    ))
}
