use crate::core::models::user::{DbRole, Permission, RolePermission};
use crate::core::repository::RbacRepository;
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::{error::ErrorKind, SqlitePool};

pub struct SqliteRbacRepository {
    pool: SqlitePool,
}

impl SqliteRbacRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RbacRepository for SqliteRbacRepository {
    async fn get_roles(&self) -> Result<Vec<DbRole>, AppError> {
        sqlx::query_as::<_, DbRole>("SELECT id, name, description, created_at FROM roles ORDER BY id")
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    async fn get_permissions(&self) -> Result<Vec<Permission>, AppError> {
        sqlx::query_as::<_, Permission>(
            "SELECT id, name, description, created_at FROM permissions ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    async fn get_role_permissions(&self) -> Result<Vec<RolePermission>, AppError> {
        sqlx::query_as::<_, RolePermission>("SELECT role_id, permission_id FROM role_permissions")
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    async fn create_role(&self, name: &str, description: Option<&str>, permissions: &[i64]) -> Result<DbRole, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        let role = sqlx::query_as::<_, DbRole>(
            "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id, name, description, created_at"
        )
        .bind(name)
        .bind(description)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    return AppError::Conflict("El nombre del rol ya existe".to_string());
                }
            }
            AppError::Database(e)
        })?;

        for perm_id in permissions {
            sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)")
                .bind(role.id)
                .bind(perm_id)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(role)
    }

    async fn update_role(&self, id: i64, name: Option<&str>, description: Option<&str>, permissions: Option<&[i64]>) -> Result<DbRole, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        if name.is_some() || description.is_some() {
            sqlx::query("UPDATE roles SET name = COALESCE($1, name), description = COALESCE($2, description) WHERE id = $3")
                .bind(name)
                .bind(description)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;
        }

        if let Some(perms) = permissions {
            sqlx::query("DELETE FROM role_permissions WHERE role_id = $1").bind(id).execute(&mut *tx).await.map_err(AppError::Database)?;
            for perm_id in perms {
                sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)").bind(id).bind(perm_id).execute(&mut *tx).await.map_err(AppError::Database)?;
            }
        }

        let role = sqlx::query_as::<_, DbRole>("SELECT id, name, description, created_at FROM roles WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;
        Ok(role)
    }

    async fn delete_role(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM roles WHERE id = $1").bind(id).execute(&self.pool).await.map_err(AppError::Database)?;
        Ok(())
    }

    async fn update_permission(&self, id: i64, description: &str) -> Result<Permission, AppError> {
        sqlx::query_as::<_, Permission>(
            "UPDATE permissions SET description = $1 WHERE id = $2 RETURNING id, name, description, created_at"
        )
        .bind(description)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
