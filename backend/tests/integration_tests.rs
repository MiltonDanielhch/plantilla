use backend::create_app;
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt; // Para llamar a app.oneshot()
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;

#[tokio::test]
async fn test_create_user_flow() {
    // 1. Configurar Base de Datos en Memoria (Aislada)
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Fallo al crear DB en memoria");

    // 2. Ejecutar Migraciones (Crear tablas)
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Fallo al migrar DB de test");

    // 3. Crear la App con el pool de prueba
    let app = create_app(pool);

    // 4. Simular Petición HTTP (POST /users)
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "username": "test_user_integration",
                    "password": "passwordSeguro123"
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // 5. Verificar Resultado
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_login_flow() {
    // 1. Setup
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Fallo DB Memoria");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Fallo Migrations");

    let app = create_app(pool);

    // 2. Crear Usuario (Usamos app.clone() porque oneshot consume la instancia)
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/users")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "login_user",
                "password": "password123"
            }).to_string()))
            .unwrap()
    ).await;

    // 3. Intentar Login
    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "login_user",
                "password": "password123"
            }).to_string()))
            .unwrap()
    ).await.unwrap();

    // 4. Verificar Éxito y Cookie
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().contains_key("set-cookie"));
}

#[tokio::test]
async fn test_delete_user_rbac_protection() {
    // 1. Setup
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Fallo DB Memoria");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Fallo Migrations");

    let app = create_app(pool);

    // 2. Crear Víctima (User ID 1)
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/users")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "victim",
                "password": "password123"
            }).to_string()))
            .unwrap()
    ).await;

    // 3. Crear Atacante (User ID 2 - Rol User por defecto)
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/users")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "attacker",
                "password": "password123"
            }).to_string()))
            .unwrap()
    ).await;

    // 4. Login Atacante para obtener Cookie
    let login_response = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "username": "attacker",
                "password": "password123"
            }).to_string()))
            .unwrap()
    ).await.unwrap();

    let cookie = login_response.headers().get("set-cookie").unwrap().to_str().unwrap().to_string();

    // 5. Intentar Borrar Víctima (ID 1) usando credenciales de Atacante
    let delete_response = app.oneshot(
        Request::builder()
            .method("DELETE")
            .uri("/users/1")
            .header("cookie", cookie)
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    // 6. Verificar que fue rechazado (Esperamos 403 Forbidden: Autenticado pero sin permisos)
    assert_eq!(delete_response.status(), StatusCode::FORBIDDEN);
}