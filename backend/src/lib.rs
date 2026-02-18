pub mod api;
pub mod core;
pub mod data;
pub mod error;
pub mod services;
pub mod settings;

use axum::{
    extract::State,
    http::{header, Method, Request, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use core::container::AppState;
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
    services::ServeDir,
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
        api::handlers::users::create_user,
        api::handlers::users::get_users,
        api::handlers::auth::login,
        api::handlers::auth::logout,
        api::handlers::users::delete_user,
        api::handlers::audit::get_audit_logs,
        api::handlers::dashboard::dashboard,
        api::handlers::audit::get_stats,
        api::handlers::audit::export_users,
        api::handlers::audit::export_audit_logs,
        api::handlers::auth::change_password,
        api::handlers::roles::get_roles,
        api::handlers::roles::get_permissions,
        api::handlers::roles::get_role_permissions,
        api::handlers::roles::create_role,
        api::handlers::roles::update_role,
        api::handlers::roles::update_permission,
        api::handlers::auth::logout_all,
    ),
    components(schemas(
        core::models::user::User,
        core::models::user::CreateUserRequest,
        core::models::user::UpdateUserRequest,
        core::models::user::LoginRequest,
        core::models::user::Role,
        core::models::user::AuditLog,
        core::models::user::UserSearch,
        core::models::user::ChangePasswordRequest,
        core::models::user::DbRole,
        core::models::user::Permission,
        core::models::user::RolePermission,
        core::models::user::CreateRoleRequest,
        core::models::user::UpdateRoleRequest,
        core::models::user::UpdatePermissionRequest,
    ))
)]
pub struct ApiDoc;

pub fn create_app(pool: SqlitePool) -> Router {
    // Crear el estado de la aplicación con inyección de dependencias
    let state = AppState::new(pool, get_jwt_secret());
    
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:4321"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(200)
            .burst_size(500)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let public_routes = Router::new()
        .route("/login", post(api::handlers::auth::login))
        .route("/logout", post(api::handlers::auth::logout))
        .route("/refresh", post(api::handlers::auth::refresh_token))
        .route("/forgot-password", post(api::handlers::auth::forgot_password))
        .route("/reset-password", post(api::handlers::auth::reset_password))
        .route("/verify-email", get(api::handlers::auth::verify_email))
        .route("/users", post(api::handlers::users::create_user));

    let protected_routes = Router::new()
        .route("/users", get(api::handlers::users::get_users))
        .route("/users/:id", delete(api::handlers::users::delete_user))
        .route("/users/:id/profile", put(api::handlers::users::update_user))
        .route("/users/password", put(api::handlers::auth::change_password))
        .route("/users/avatar", post(api::handlers::users::upload_avatar))
        .route("/send-verification-email", post(api::handlers::auth::send_verification_email))
        .route("/dashboard", get(api::handlers::dashboard::dashboard))
        .route("/roles", get(api::handlers::roles::get_roles))
        .route("/permissions", get(api::handlers::roles::get_permissions))
        .route("/roles/permissions", get(api::handlers::roles::get_role_permissions))
        .route("/stats", get(api::handlers::audit::get_stats))
        .route("/logout-all", post(api::handlers::auth::logout_all))
        .route_layer(middleware::from_fn_with_state(state.clone(), api::middleware::auth_guard));

    let admin_routes = Router::new()
        .route("/users/:id", get(api::handlers::users::get_user_by_id))
        .route("/users/export", get(api::handlers::audit::export_users))
        .route("/roles", post(api::handlers::roles::create_role))
        .route("/roles/:id", put(api::handlers::roles::update_role).delete(api::handlers::roles::delete_role))
        .route("/permissions/:id", put(api::handlers::roles::update_permission))
        .route("/audit-logs", get(api::handlers::audit::get_audit_logs))
        .route("/audit-logs/export", get(api::handlers::audit::export_audit_logs))
        .route_layer(middleware::from_fn_with_state(state.clone(), api::middleware::admin_guard));

    let api_v1 = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api/v1", api_v1)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .layer(CookieManagerLayer::new())
        .layer(GovernorLayer { config: governor_conf })
        .layer(cors)
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
        .with_state(state)
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret".to_string())
}

async fn root() -> &'static str {
    "Sistema Semilla 3026: Online"
}

async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(state.pool()).await {
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
