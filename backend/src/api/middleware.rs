use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_cookies::Cookies;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::core::models::user::Claims;

pub async fn auth_guard(cookies: Cookies, req: Request, next: Next) -> Result<Response, StatusCode> {
    // Verificamos si existe la cookie de sesión
    if let Some(cookie) = cookies.get("auth_token") {
        // Validar que el token sea real y no haya expirado
        let validation = decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default()
        );

        if validation.is_ok() {
            return Ok(next.run(req).await);
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}