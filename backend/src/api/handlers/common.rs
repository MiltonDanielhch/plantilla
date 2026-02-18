use crate::core::models::user::Claims;
use crate::error::AppError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{body::Body, http::header};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use sqlx::Error as SqlxError;
use std::io;

/// Extrae y valida claims del token JWT
pub fn extract_claims(cookie_value: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        cookie_value,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::AuthError("Token inválido".to_string()))
}

/// Construye respuesta JSON estándar
pub fn json_response<T: serde::Serialize>(status: StatusCode, data: T) -> impl IntoResponse {
    (status, axum::Json(data))
}

/// Construye respuesta de error simple
pub fn error_response(message: &str) -> impl IntoResponse {
    (
        StatusCode::BAD_REQUEST,
        axum::Json(json!({"error": message })),
    )
}

/// Crea respuesta CSV para exportación
pub fn csv_response(filename: &str, data: Vec<u8>) -> Result<Response, AppError> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(data))
        .map_err(|e| AppError::Database(SqlxError::Io(io::Error::new(io::ErrorKind::Other, e))))
}

/// Helper para convertir errores CSV
pub fn map_csv_error<E>(e: E) -> AppError
where
    E: std::fmt::Display,
{
    AppError::Database(SqlxError::Io(io::Error::new(
        io::ErrorKind::Other,
        e.to_string(),
    )))
}
