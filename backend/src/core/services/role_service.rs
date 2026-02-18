use crate::core::models::user::{CreateRoleRequest, DbRole, Permission, RolePermission, UpdatePermissionRequest, UpdateRoleRequest};
use crate::core::repository::Repository;
use crate::error::AppError;
use validator::Validate;

pub struct RoleService<R: Repository> {
    repository: R,
}

impl<R: Repository> RoleService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // ============ ROLES ============

    pub async fn get_roles(&self) -> Result<Vec<DbRole>, AppError> {
        self.repository.get_roles().await
    }

    pub async fn create_role(&self, request: CreateRoleRequest) -> Result<DbRole, AppError> {
        request.validate()
            .map_err(|e| AppError::Validation(format!("Datos inválidos: {}", e)))?;

        self.repository
            .create_role(&request.name, request.description.as_deref(), &request.permissions)
            .await
    }

    pub async fn update_role(
        &self,
        id: i64,
        request: UpdateRoleRequest,
    ) -> Result<DbRole, AppError> {
        self.repository
            .update_role(
                id,
                request.name.as_deref(),
                request.description.as_deref(),
                request.permissions.as_deref(),
            )
            .await
    }

    pub async fn delete_role(&self, id: i64) -> Result<(), AppError> {
        self.repository.delete_role(id).await
    }

    // ============ PERMISSIONS ============

    pub async fn get_permissions(&self) -> Result<Vec<Permission>, AppError> {
        self.repository.get_permissions().await
    }

    pub async fn update_permission(
        &self,
        id: i64,
        request: UpdatePermissionRequest,
    ) -> Result<Permission, AppError> {
        self.repository.update_permission(id, &request.description).await
    }

    // ============ ROLE-PERMISSIONS ============

    pub async fn get_role_permissions(&self) -> Result<Vec<RolePermission>, AppError> {
        self.repository.get_role_permissions().await
    }
}
