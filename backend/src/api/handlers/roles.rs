use crate::core::container::AppState;
use crate::core::models::user::{
    CreateRoleRequest, UpdatePermissionRequest, UpdateRoleRequest,
};
use crate::error::AppError;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::Json as AxumJson;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/v1/roles",
    responses(
        (status = 200, description = "Lista de roles del sistema", body = Vec<crate::core::models::user::DbRole>)
    )
)]
pub async fn get_roles(
    State(state): State<AppState>,
) -> Result<AxumJson<Vec<crate::core::models::user::DbRole>>, AppError> {
    let service = state.role_service();
    let roles = service.get_roles().await?;
    Ok(AxumJson(roles))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    responses(
        (status = 200, description = "Lista de permisos del sistema", body = Vec<crate::core::models::user::Permission>)
    )
)]
pub async fn get_permissions(
    State(state): State<AppState>,
) -> Result<AxumJson<Vec<crate::core::models::user::Permission>>, AppError> {
    let service = state.role_service();
    let permissions = service.get_permissions().await?;
    Ok(AxumJson(permissions))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/permissions",
    responses(
        (status = 200, description = "Asociaciones Rol-Permiso", body = Vec<crate::core::models::user::RolePermission>)
    )
)]
pub async fn get_role_permissions(
    State(state): State<AppState>,
) -> Result<AxumJson<Vec<crate::core::models::user::RolePermission>>, AppError> {
    let service = state.role_service();
    let rps = service.get_role_permissions().await?;
    Ok(AxumJson(rps))
}

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Rol creado", body = crate::core::models::user::DbRole),
        (status = 409, description = "El rol ya existe")
    )
)]
pub async fn create_role(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, AxumJson<crate::core::models::user::DbRole>), AppError> {
    let service = state.role_service();
    let role = service.create_role(payload).await?;
    Ok((StatusCode::CREATED, AxumJson(role)))
}

#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    request_body = UpdateRoleRequest,
    responses((status = 200, description = "Rol actualizado", body = crate::core::models::user::DbRole))
)]
pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<AxumJson<crate::core::models::user::DbRole>, AppError> {
    let service = state.role_service();
    let role = service.update_role(id, payload).await?;
    Ok(AxumJson(role))
}

pub async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let service = state.role_service();
    service.delete_role(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/api/v1/permissions/{id}",
    request_body = UpdatePermissionRequest,
    responses((status = 200, description = "Permiso actualizado", body = crate::core::models::user::Permission))
)]
pub async fn update_permission(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<AxumJson<crate::core::models::user::Permission>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

    let service = state.role_service();
    let permission = service.update_permission(id, payload).await?;
    Ok(AxumJson(permission))
}
