use backend::create_app; // Importamos desde nuestra propia lib
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{net::SocketAddr, str::FromStr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Cargar variables de entorno
    dotenv::dotenv().ok();

    // 2. Inicializar Observabilidad (Logs avanzados)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
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

    // 4. Construir la aplicación e inyectar el pool
    let app = create_app(pool);

    // 5. Definir dirección y arrancar
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 Sintonía 3026 Activada en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Escucha señales de apagado (Ctrl+C o SIGTERM) para cerrar conexiones limpiamente
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