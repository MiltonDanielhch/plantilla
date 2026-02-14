pub mod api;
pub mod core;
pub mod data;

use axum::{
    routing::{delete, get, post},
    Router, middleware,
    http::{header, Method},
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Sintonía 3026 API", version = "1.0.0", description = "Documentación viva del sistema Sintonía 3026"),
    paths(
        api::handlers::user::create_user,
        api::handlers::user::get_users,
        api::handlers::user::login,
        api::handlers::user::logout,
        api::handlers::user::delete_user,
        api::handlers::user::get_audit_logs,
        api::handlers::user::dashboard,
    ),
    components(
        schemas(
            core::models::user::User,
            core::models::user::CreateUserRequest,
            core::models::user::LoginRequest,
            core::models::user::Role,
            core::models::user::AuditLog,
            core::models::user::UserSearch,
        )
    )
)]
pub struct ApiDoc;

pub fn create_app(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4321".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/users", post(api::handlers::user::create_user).get(api::handlers::user::get_users))
        .route("/login", post(api::handlers::user::login))
        .route("/logout", post(api::handlers::user::logout))
        .route("/users/:id", delete(api::handlers::user::delete_user).route_layer(middleware::from_fn(api::middleware::admin_guard)))
        .route("/dashboard", get(api::handlers::user::dashboard).route_layer(middleware::from_fn(api::middleware::auth_guard)))
        .route("/audit-logs", get(api::handlers::user::get_audit_logs).route_layer(middleware::from_fn(api::middleware::admin_guard)))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .with_state(pool)
}

async fn root() -> &'static str { "Sistema Semilla 3026: Online" }
async fn health_check() -> &'static str { "OK" }