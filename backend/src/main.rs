use axum::{http::{header, Method}, middleware, routing::{get, post}, Router};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{net::SocketAddr, str::FromStr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;

// Declaración de módulos de la arquitectura
mod api;
mod core;
mod data;

#[tokio::main]
async fn main() {
    // 1. Cargar variables de entorno
    dotenv::dotenv().ok();

    // 2. Inicializar Observabilidad (Logs avanzados)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. Conexión a Base de Datos (Crear archivo si no existe)
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no configurada");
    
    let connection_options = SqliteConnectOptions::from_str(&db_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("❌ Fallo al conectar a la Base de Datos");

    // 3.1 Ejecutar Migraciones (Evolución de la DB)
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("❌ Fallo al ejecutar migraciones");

    tracing::info!("💾 Memoria conectada: {}", db_url);

    // Configuración CORS estricta para soportar cookies/credentials
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4321".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    // 4. Construir la aplicación e inyectar el pool
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/users", post(api::handlers::user::create_user).get(api::handlers::user::get_users))
        .route("/login", post(api::handlers::user::login))
        .route("/logout", post(api::handlers::user::logout))
        .route("/dashboard", 
            get(api::handlers::user::dashboard)
            .route_layer(middleware::from_fn(api::middleware::auth_guard))
        )
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .with_state(pool);

    // 5. Definir dirección y arrancar
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Sintonía 3026 Activada en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handlers básicos
async fn root() -> &'static str { "Sistema Semilla 3026: Online" }
async fn health_check() -> &'static str { "OK" }