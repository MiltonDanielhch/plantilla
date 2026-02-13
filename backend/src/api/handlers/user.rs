use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use sqlx::SqlitePool;
use crate::core::models::user::{CreateUserRequest, User};

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username) VALUES ($1) RETURNING id, username, created_at"
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            // En un sistema real, manejaríamos mejor los errores (ej. duplicados)
            tracing::error!("Error creando usuario: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al crear usuario").into_response()
        }
    }
}

pub async fn get_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = crate::data::user_repository::get_all(&pool).await.map_err(|e| {
        tracing::error!("Error de sintonía al leer usuarios: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Fallo en la matriz de datos".to_string())
    })?;

    Ok(Json(users))
}