pub mod api;
pub mod core;
pub mod data;
pub mod error;
pub mod settings;

use axum::{
    extract::State,
    http::{header, Method, Request, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::cors::CorsLayer;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
    #[openapi(
        info(
            title = "Sintonía 3026 API",
            version = "1.0.0",
            description = "Documentación viva del sistema Sintonía 3026"
        ),
        paths(
            api::handlers::user::create_user,
            api::handlers::user::get_users,
            api::handlers::user::login,
            api::handlers::user::logout,
            api::handlers::user::delete_user,
            api::handlers::user::get_audit_logs,
            api::handlers::user::dashboard,
            api::handlers::user::get_stats,
            api::handlers::user::export_users,
            api::handlers::user::export_audit_logs,
        ),
        components(schemas(
            core::models::user::User,
            core::models::user::CreateUserRequest,
            core::models::user::UpdateUserRequest,
            core::models::user::LoginRequest,
            core::models::user::Role,
            core::models::user::AuditLog,
            core::models::user::UserSearch,
        ))
    )]
pub struct ApiDoc;

pub fn create_app(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:4321"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    // Configuración de Rate Limiting
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            // --- PRODUCCIÓN (Descomentar al desplegar) ---
            // .per_second(20)  // Límite razonable para usuarios reales
            // .burst_size(50)  // Margen para ráfagas cortas
            // --- DESARROLLO (Actual: Alto para evitar bloqueos en SSR local) ---
            .per_second(200)
            .burst_size(500)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    // 1. Rutas Públicas (Login, Registro, Logout)
    let public_routes = Router::new()
        .route("/login", post(api::handlers::user::login))
        .route("/logout", post(api::handlers::user::logout))
        .route("/users", post(api::handlers::user::create_user)); // Registro público

    // 2. Rutas Protegidas (Requieren Auth)
    let protected_routes = Router::new()
        .route("/users", get(api::handlers::user::get_users))
        .route("/users/:id/profile", put(api::handlers::user::update_user))
        .route("/dashboard", get(api::handlers::user::dashboard))
        .route("/stats", get(api::handlers::user::get_stats))
        .route_layer(middleware::from_fn(api::middleware::auth_guard));

    // 3. Rutas Admin (Requieren Rol Admin)
    let admin_routes = Router::new()
        .route("/users/:id", 
            delete(api::handlers::user::delete_user)
            .get(api::handlers::user::get_user_by_id)) // Agregamos GET para ver detalle/editar
        .route("/users/export", get(api::handlers::user::export_users))
        .route("/audit-logs", get(api::handlers::user::get_audit_logs))
        .route("/audit-logs/export", get(api::handlers::user::export_audit_logs))
        .route_layer(middleware::from_fn(api::middleware::admin_guard));

    let api_v1 = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api/v1", api_v1)
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

async fn root() -> &'static str {
    "Sistema Semilla 3026: Online"
}
async fn health_check(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, "Sintonía 3026: Operativo (DB Conectada)"),
        Err(e) => {
            tracing::error!("Health Check Fallido: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Sintonía 3026: Error Crítico (DB Desconectada)",
            )
        }
    }
}
