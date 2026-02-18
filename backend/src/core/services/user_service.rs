use crate::core::models::user::{CreateUserRequest, Role, UpdateUserRequest, User, UserSearch};
use crate::core::repository::Repository;
use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};

pub struct UserService<R: Repository> {
    repository: R,
}

impl<R: Repository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // ============ CRUD OPERATIONS ============

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        let password_hash = self.hash_password(&request.password)?;

        let user = self
            .repository
            .create_user(&request.username, &password_hash, request.email.as_deref())
            .await?;

        // Si tiene email, crear token de verificación
        if let Some(ref _email) = request.email {
            self.create_verification_token(user.id).await.ok();
        }

        Ok(user)
    }

    pub async fn get_user(&self, id: i64) -> Result<User, AppError> {
        self.repository
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))
    }

    pub async fn get_users(
        &self,
        search: UserSearch,
    ) -> Result<(Vec<User>, i64, i64), AppError> {
        let (users, total) = self
            .repository
            .get_all(search.q, search.page, search.limit)
            .await?;

        let total_pages = (total as f64 / search.limit as f64).ceil() as i64;
        Ok((users, total, total_pages))
    }

    pub async fn update_user(
        &self,
        id: i64,
        request: UpdateUserRequest,
        requester_id: i64,
        requester_role: Role,
    ) -> Result<User, AppError> {
        // Verificar permisos
        self.check_update_permissions(requester_id, requester_role, id, request.role.is_some())?;

        self.repository
            .update_user(id, request.email.as_deref(), request.role)
            .await
    }

    pub async fn delete_user(
        &self,
        id: i64,
        requester_id: i64,
        requester_role: Role,
        requester_username: &str,
    ) -> Result<(), AppError> {
        // Verificar permisos
        self.check_delete_permissions(requester_role, requester_id, id)?;

        self.repository.delete_user(id, requester_username).await
    }

    // ============ AVATAR ============

    pub async fn update_avatar(&self, user_id: i64, avatar_data: AvatarData) -> Result<User, AppError> {
        let avatar_url = self.save_avatar_file(user_id, avatar_data).await?;
        self.repository.update_avatar(user_id, &avatar_url).await
    }

    async fn save_avatar_file(&self, user_id: i64, file: AvatarData) -> Result<String, AppError> {
        self.ensure_uploads_dir()?;

        let timestamp = Utc::now().timestamp();
        let extension = std::path::Path::new(&file.filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg");

        let new_filename = format!("{}_{}.{}", user_id, timestamp, extension);
        let file_path = std::path::Path::new("uploads").join(&new_filename);

        std::fs::write(&file_path, &file.data)
            .map_err(|e| AppError::Database(sqlx::Error::Io(e)))?;

        Ok(format!("/uploads/{}", new_filename))
    }

    fn ensure_uploads_dir(&self) -> Result<(), AppError> {
        let dir = std::path::Path::new("uploads");
        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(|e| AppError::Database(sqlx::Error::Io(e)))?;
        }
        Ok(())
    }

    // ============ VERIFICATION ============

    async fn create_verification_token(&self, user_id: i64) -> Result<String, AppError> {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .unwrap()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        self.repository
            .create_email_verification_token(user_id, &token, &expires_at)
            .await?;

        Ok(token)
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::AuthError(format!("Error de seguridad: {}", e)))
            .map(|h| h.to_string())
    }

    // ============ PERMISSIONS ============

    fn check_update_permissions(
        &self,
        requester_id: i64,
        requester_role: Role,
        target_id: i64,
        changing_role: bool,
    ) -> Result<(), AppError> {
        if requester_role != Role::Admin && requester_id != target_id {
            return Err(AppError::Forbidden("No puedes editar otros usuarios".to_string()));
        }

        if changing_role && requester_role != Role::Admin {
            return Err(AppError::Forbidden(
                "Solo los administradores pueden asignar roles".to_string(),
            ));
        }

        Ok(())
    }

    fn check_delete_permissions(
        &self,
        role: Role,
        requester_id: i64,
        target_id: i64,
    ) -> Result<(), AppError> {
        if role != Role::Admin && requester_id != target_id {
            return Err(AppError::Forbidden(
                "No tienes permiso para eliminar este usuario".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct AvatarData {
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}

impl AvatarData {
    pub fn validate(&self) -> Result<(), AppError> {
        if !self.content_type.starts_with("image/") {
            return Err(AppError::Validation("El archivo debe ser una imagen".to_string()));
        }

        if self.data.len() > 2 * 1024 * 1024 {
            return Err(AppError::Validation(
                "El archivo es demasiado grande (máximo 2MB)".to_string(),
            ));
        }

        Ok(())
    }
}
