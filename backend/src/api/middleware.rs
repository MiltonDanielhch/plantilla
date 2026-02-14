use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn auth_guard(cookies: Cookies, req: Request, next: Next) -> Result<Response, StatusCode> {
    // Verificamos si existe la cookie de sesión
    if cookies.get("auth_token").is_some() {
        // Si tiene pase, dejamos que la petición continúe hacia el handler
        Ok(next.run(req).await)
    } else {
        // Si no tiene pase, bloqueamos aquí mismo
        Err(StatusCode::UNAUTHORIZED)
    }
}