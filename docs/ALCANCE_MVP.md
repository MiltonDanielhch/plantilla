# Alcance del Proyecto: Sintonía 3026

## Problema que resuelve
Establecer una "Célula Madre" de software (Boilerplate) que combine la seguridad y velocidad de Rust (Backend) con la versatilidad de Astro (Frontend), permitiendo el despliegue rápido de aplicaciones escalables sin deuda técnica inicial.

## Funcionalidades MUST (MVP)

### ✅ Fase 1-4: Cimientos y Conexión (Completado)
- **Arquitectura:** Monorepo estructurado (Clean Architecture).
- **Datos:** Persistencia en SQLite con migraciones (Tabla `users`).
- **API:** Endpoint de registro (`POST /users`).
- **UI:** Formulario de registro reactivo en Astro.

### 🚀 Fase 5: Expansión (Próximo Paso)
- **API:** Endpoint de lectura (`GET /users`).
- **UI:** Visualización de datos en tiempo real (Lista de Usuarios).

### 🔮 Futuro
- Autenticación y manejo de sesiones.
- Despliegue contenerizado (Docker).