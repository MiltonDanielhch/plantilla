# Sistema Semilla 3026

## Misión
Arquitectura de software universal, segura y escalable. Diseñada para liberar el potencial humano mediante un stack moderno y robusto.

## Capacidades del Sistema
### 🛡️ Seguridad y Autenticación
- **Hashing:** Argon2 para almacenamiento seguro de contraseñas.
- **Sesiones:** JWT (JSON Web Tokens) en Cookies `HttpOnly` y `SameSite`.
- **Protección:** Middleware de seguridad para rutas protegidas.

### 👑 Jerarquía y Roles (RBAC)
- **User:** Acceso básico al Dashboard.
- **Admin:** Acceso privilegiado con capacidades ejecutivas:
    - Eliminación de usuarios.
    - Visualización de bitácora de auditoría.

### 👁️ Auditoría (Trazabilidad)
- Registro inmutable de acciones administrativas en base de datos (`audit_logs`).
- Visualización integrada en el Dashboard.

## Requisitos Previos
- **Rust & Cargo**: [Instalar desde rustup.rs](https://rustup.rs/) (El cerebro)
- **Node.js**: [Instalar LTS](https://nodejs.org/) (La vitrina)
- **Python 3**: Para scripts de automatización.
- **SQLite**: Base de datos integrada (no requiere instalación de servidor).

## Inicio Rápido (Quickstart)

### 1. Backend (El Núcleo)
```bash
cd backend
cargo run
```
*El sistema aplicará migraciones automáticas y escuchará en `http://localhost:3000`.*

### 2. Frontend (La Vitrina)
```bash
cd frontend
npm install
npm run dev
```
*Accede a la interfaz en `http://localhost:4321`.*

## Reglas de Sintonía
- Código limpio y modular.
- **Arquitectura Hexagonal:** Separación clara entre `core` (modelos), `data` (repositorios) y `api` (handlers).
- **Tipado Fuerte:** Uso extensivo del sistema de tipos de Rust para evitar errores en tiempo de ejecución.
