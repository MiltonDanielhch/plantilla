use crate::api::handlers::common::csv_response;
use crate::core::container::AppState;
use crate::error::AppError;
use axum::extract::State;
use axum::response::Response;
use axum::Json;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs",
    responses(
        (status = 200, description = "Bitácora de auditoría del sistema", body = Vec<crate::core::models::user::AuditLog>)
    )
)]
pub async fn get_audit_logs(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::core::models::user::AuditLog>>, AppError> {
    let service = state.audit_service();
    let logs = service.get_audit_logs().await?;
    Ok(Json(logs))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs/export",
    responses(
        (status = 200, description = "Exportación CSV de logs de auditoría", content_type = "text/csv"),
        (status = 500, description = "Error al generar CSV")
    )
)]
pub async fn export_audit_logs(State(state): State<AppState>) -> Result<Response, AppError> {
    let service = state.audit_service();
    let csv_data = service.export_audit_logs().await?;
    csv_response("audit_logs_export.csv", csv_data)
}

#[utoipa::path(
    get,
    path = "/api/v1/stats",
    responses((status = 200, description = "Estadísticas del dashboard"))
)]
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let service = state.audit_service();
    let stats = service.get_stats().await?;

    Ok(Json(json!({
        "total_users": stats.total_users,
        "active_users": stats.active_users,
        "admin_users": stats.admin_users,
        "new_users_today": stats.new_users_today
    })))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/export",
    responses(
        (status = 200, description = "Exportación CSV de usuarios", content_type = "text/csv"),
        (status = 500, description = "Error al generar CSV")
    )
)]
pub async fn export_users(State(state): State<AppState>) -> Result<Response, AppError> {
    let service = state.audit_service();
    let csv_data = service.export_users().await?;
    csv_response("users_export.csv", csv_data)
}
