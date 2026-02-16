# 📋 Lista de Faltantes y Mejoras - Proyecto Sintonía 3026

Este documento recopila todas las funcionalidades, mejoras y tareas pendientes detectadas en el sistema actual (V4.1).

## 🚀 Funcionalidades Backend (Prioridad Alta - Fase 38)
1.  **Exportación a CSV:**
    *   [ ] Endpoint `GET /api/v1/users/export`.
    *   [ ] Endpoint `GET /api/v1/audit-logs/export`.
    *   [ ] Conexión con botones "Exportar" en Frontend (Usuarios y Auditoría).
2.  **Subida de Archivos (Avatares):**
    *   [ ] Soporte `multipart/form-data` en Axum (Backend).
    *   [ ] Sistema de almacenamiento (carpeta `uploads/` local o AWS S3).
    *   [ ] Endpoint `POST /api/v1/users/avatar`.
    *   [ ] Actualizar UI de Settings para mostrar imagen real en lugar de iniciales.
3.  **Seguridad Avanzada:**
    *   [ ] **Refresh Tokens:** Implementar rotación de tokens para evitar sesiones largas inseguras (actualmente 24h fijas).
    *   [ ] **Recuperación de Contraseña:** Flujo de "Olvidé mi contraseña" (requiere envío de emails).
    *   [ ] **Verificación de Email:** Enviar correo de confirmación al registrarse para validar cuentas.

## 🎨 Mejoras Frontend / UI (Prioridad Media - Fase 37)
4.  **Componentes Faltantes:**
    *   [ ] **Command Palette (⌘K):** Búsqueda rápida global de acciones y navegación (estilo VS Code).
    *   [ ] **Skeletons:** Reemplazar spinners de carga por "esqueletos" visuales (cajas grises pulsantes) para mejor percepción de velocidad.
    *   [ ] **Dialogs/Modals:** Reemplazar las alertas nativas del navegador (`confirm()`, `alert()`) por modales estilizados (shadcn/ui) en acciones destructivas.
5.  **Feedback y Estados:**
    *   [ ] **Páginas de Error:** Diseños personalizados para 404 (No encontrado) y 500 (Error servidor).
    *   [ ] **Empty States:** Ilustraciones o mensajes amigables cuando las tablas están vacías.

## 🧪 Calidad y Testing (Fase 39)
6.  **Tests Automatizados:**
    *   [ ] **E2E (Playwright):** Tests automáticos que simulen un usuario real (Login -> Crear Usuario -> Logout).
    *   [ ] **Unitarios:** Aumentar cobertura en Backend (lógica de negocio y validaciones).
    *   [ ] **Integración:** Verificar que la API y la Base de Datos hablan correctamente en escenarios de error.

## ⚙️ Infraestructura y DevOps (Fase 40)
7.  **Producción:**
    *   [ ] **Docker Compose Prod:** Configuración optimizada (multi-stage build) para despliegue real.
    *   [ ] **CI/CD:** Pipeline de GitHub Actions para correr tests y deploy automático.
    *   [ ] **Backups:** Script automático de respaldo de la base de datos SQLite.
    *   [ ] **Monitoring:** Integración con Sentry o similar para rastrear errores en tiempo real.

## 📧 Comunicación (Sistema de Correos)
8.  **Email System:**
    *   [ ] Integrar crate `lettre` en Rust.
    *   [ ] Configurar servidor SMTP (SendGrid, Resend, o Gmail).
    *   [ ] Crear templates HTML para correos transaccionales (Bienvenida, Reset Password).

---
*Generado automáticamente basado en el análisis del estado actual del proyecto.*