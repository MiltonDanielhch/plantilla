use backend::{create_app, settings::Settings};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{net::SocketAddr, str::FromStr};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // 1. Cargar variables de entorno
    dotenv::dotenv().ok();

    // 1.1 Cargar Configuración Jerárquica
    let settings = Settings::new().expect("❌ Fallo al cargar configuración (config/default.toml)");

    // 2. Crear directorio de logs si no existe
    std::fs::create_dir_all("logs").expect("❌ No se pudo crear directorio logs/");

    // 3. Configurar Logging a Archivo y Consola
    // Appender con rotación diaria: logs/backend-YYYY-MM-DD.log
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "backend.log");
    
    // Layer para archivo (formato JSON)
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file_appender);
    
    // Layer para consola (formato legible)
    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(false);
    
    // Inicializar subscriber con ambos layers
    tracing_subscriber::registry()
        .with(EnvFilter::new(&settings.log_level))
        .with(file_layer)
        .with(console_layer)
        .init();

    tracing::info!("📝 Logs configurados: consola + archivo logs/backend-YYYY-MM-DD.log");

    // 4. Conexión a Base de Datos
    let db_url = settings.database_url;

    let connection_options = SqliteConnectOptions::from_str(&db_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("❌ Fallo al conectar a la Base de Datos");

    // 4.1 Ejecutar Migraciones
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("❌ Fallo al ejecutar migraciones");

    tracing::info!("💾 Memoria conectada: {}", db_url);

    // 5. Construir la aplicación
    let app = create_app(pool);

    // 6. Definir dirección y arrancar
    let addr = format!("{}:{}", settings.host, settings.port)
        .parse::<SocketAddr>()
        .expect("Dirección inválida");
    
    tracing::info!("🚀 Sintonía 3026 Activada en {}", addr);
    tracing::info!("📊 Stats disponibles en: http://{}/api/v1/stats", addr);
    tracing::info!("📚 API Docs en: http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

/// Escucha señales de apagado (Ctrl+C o SIGTERM)
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("falló al instalar el manejador Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("falló al instalar el manejador de señal")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🛑 Señal de apagado recibida, iniciando Graceful Shutdown...");
}
