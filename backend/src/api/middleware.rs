use crate::core::container::AppState;
use crate::core::models::user::{Claims, Role};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tower_cookies::Cookies;

/// Middleware que verifica autenticación
/// 
/// Extrae el token de las cookies y valida que sea válido
/// No requiere acceso a la base de datos, solo validación JWT
pub async fn auth_guard(
    State(state): State<AppState>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Verificamos si existe la cookie de sesión
    if let Some(cookie) = cookies.get("auth_token") {
        // Validar que el token sea real y no haya expirado
        let validation = decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_secret(state.container().jwt_secret().as_bytes()),
            &Validation::default(),
        );

        if validation.is_ok() {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

/// Middleware que verifica permisos de administrador
/// 
/// Extrae el token, lo valida y verifica que el rol sea Admin
/// No requiere acceso a la base de datos, solo validación JWT
pub async fn admin_guard(
    State(state): State<AppState>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = cookies.get("auth_token").map(|c| c.value().to_string());

    match token {
        Some(t) => {
            let token_data = decode::<Claims>(
                &t,
                &DecodingKey::from_secret(state.container().jwt_secret().as_bytes()),
                &Validation::default(),
            );

            match token_data {
                Ok(c) if c.claims.role == Role::Admin => Ok(next.run(req).await),
                _ => Err(StatusCode::FORBIDDEN), // 403: Prohibido (tiene token, pero no rango)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED), // 401: No hay token
    }
}
