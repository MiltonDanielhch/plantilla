pub mod api;
pub mod core;
pub mod data;

use axum::{
    routing::{delete, get, post},
    Router, middleware,
    http::{header, Method, StatusCode, Request},
    extract::State,
    response::IntoResponse,
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use tower_http::{trace::TraceLayer, request_id::{MakeRequestUuid, SetRequestIdLayer}};
use tower_cookies::CookieManagerLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::SmartIpKeyExtractor};
use std::sync::Arc;

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

    // Configuración de Rate Limiting: 10 peticiones por segundo, ráfaga de 20
    let governor_conf = Arc::new(GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(20)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap());

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
        .layer(CookieManagerLayer::new())
        .layer(GovernorLayer { config: governor_conf })
        .layer(cors) // CORS debe ser el último (externo) para manejar errores del Governor
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = request.headers().get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    tracing::info_span!("http_request", request_id = %request_id, method = %request.method(), uri = %request.uri())
                })
                .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO))
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(pool)
}

async fn root() -> &'static str { "Sistema Semilla 3026: Online" }
async fn health_check(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, "Sintonía 3026: Operativo (DB Conectada)"),
        Err(e) => {
            tracing::error!("Health Check Fallido: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Sintonía 3026: Error Crítico (DB Desconectada)")
        }
    }
}