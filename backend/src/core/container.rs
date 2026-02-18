use crate::core::repository::Repository;
use crate::core::services::audit_service::AuditService;
use crate::core::services::auth_service::AuthService;
use crate::core::services::role_service::RoleService;
use crate::core::services::user_service::UserService;
use crate::data::SqliteRepository;
use sqlx::SqlitePool;

/// Container de servicios que gestiona todas las dependencias de la aplicación
///
/// Permite:
/// - Inyección de dependencias type-safe
/// - Fácil testing con mocks
/// - Lifecycle management de servicios
#[derive(Clone)]
pub struct ServiceContainer {
    pool: SqlitePool,
    jwt_secret: String,
}

impl ServiceContainer {
    pub fn new(pool: SqlitePool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    /// Crea el repositorio de usuarios
    pub fn user_repository(&self) -> SqliteRepository {
        SqliteRepository::new(self.pool.clone())
    }

    /// Crea el servicio de autenticación
    pub fn auth_service(&self) -> AuthService<SqliteRepository> {
        AuthService::new(
            SqliteRepository::new(self.pool.clone()),
            self.jwt_secret.clone(),
        )
    }

    /// Crea el servicio de usuarios
    pub fn user_service(&self) -> UserService<SqliteRepository> {
        UserService::new(SqliteRepository::new(self.pool.clone()))
    }

    /// Crea el servicio de roles
    pub fn role_service(&self) -> RoleService<SqliteRepository> {
        RoleService::new(SqliteRepository::new(self.pool.clone()))
    }

    /// Crea el servicio de auditoría
    pub fn audit_service(&self) -> AuditService<SqliteRepository> {
        AuditService::new(SqliteRepository::new(self.pool.clone()))
    }

    /// Obtiene el pool de conexiones
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Obtiene el JWT secret
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

/// Estado compartido de la aplicación para Axum
///
/// Este struct se pasa a todos los handlers mediante State
#[derive(Clone)]
pub struct AppState {
    container: ServiceContainer,
}

impl AppState {
    pub fn new(pool: SqlitePool, jwt_secret: String) -> Self {
        Self {
            container: ServiceContainer::new(pool, jwt_secret),
        }
    }

    pub fn container(&self) -> &ServiceContainer {
        &self.container
    }

    /// Helper para obtener el auth service
    pub fn auth_service(&self) -> AuthService<SqliteRepository> {
        self.container.auth_service()
    }

    /// Helper para obtener el user service
    pub fn user_service(&self) -> UserService<SqliteRepository> {
        self.container.user_service()
    }

    /// Helper para obtener el role service
    pub fn role_service(&self) -> RoleService<SqliteRepository> {
        self.container.role_service()
    }

    /// Helper para obtener el audit service
    pub fn audit_service(&self) -> AuditService<SqliteRepository> {
        self.container.audit_service()
    }

    /// Helper para obtener el pool
    pub fn pool(&self) -> &SqlitePool {
        self.container.pool()
    }
}

// Mocks para testing se pueden agregar aquí cuando se necesiten
// Ejemplo: MockRepository que implementa Repository
