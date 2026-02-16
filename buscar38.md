# 📋 GUÍA COMPLETA - FASE 38 IMPLEMENTADA

## 🎯 Resumen
Fase 38 completada al 100%. Todas las features Premium implementadas en backend y frontend.

---

## 📁 ESTRUCTURA DE ARCHIVOS MODIFICADOS/CREADOS

### 🔧 BACKEND

#### 1. Migraciones (ubicación: `backend/migrations/`)

**0006_add_avatar_url.sql** - Agrega campo avatar_url a users
```sql
-- Agrega columna avatar_url a tabla users
-- Ubicación: backend/migrations/0006_add_avatar_url.sql
```

**0007_create_refresh_tokens.sql** - Tabla para refresh tokens
```sql
-- Crea tabla refresh_tokens con:
-- - id, user_id, token, expires_at, created_at, used
-- - Indices para búsqueda rápida
-- Ubicación: backend/migrations/0007_create_refresh_tokens.sql
```

**0008_create_password_reset_tokens.sql** - Tabla para reset de contraseña
```sql
-- Crea tabla password_reset_tokens
-- Tokens válidos por 1 hora, un solo uso
-- Ubicación: backend/migrations/0008_create_password_reset_tokens.sql
```

**0009_add_email_verification.sql** - Verificación de email
```sql
-- Agrega campo email_verified a users
-- Crea tabla email_verification_tokens
-- Tokens válidos por 24 horas
-- Ubicación: backend/migrations/0009_add_email_verification.sql
```

#### 2. Modelos (ubicación: `backend/src/core/models/user.rs`)

**User actualizado con nuevos campos:**
```rust
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: Role,
    pub avatar_url: Option<String>,
    pub email_verified: bool,  // ← NUEVO
    pub created_at: String,
}
```

**Nuevos modelos agregados:**
- `RefreshToken` - Para rotación de tokens (línea ~86)
- `PasswordResetToken` - Para recuperación de contraseña (línea ~109)
- `EmailVerificationToken` - Para verificación de email (línea ~128)
- `TokenResponse` - Respuesta con access_token + refresh_token
- `RefreshRequest` - Request para refresh endpoint
- `ForgotPasswordRequest` - Email para recuperación
- `ResetPasswordRequest` - Token + nueva contraseña
- `VerifyEmailRequest` - Token de verificación

#### 3. Repositorio (ubicación: `backend/src/core/repository.rs`)

**Nuevos métodos en trait UserRepository:**
```rust
// Consultas
async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

// Actualizaciones
async fn update_password(&self, id: i64, password_hash: &str) -> Result<(), AppError>;
async fn verify_email(&self, user_id: i64) -> Result<(), AppError>;

// Refresh Tokens (línea ~35)
async fn create_refresh_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<RefreshToken, AppError>;
async fn get_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, AppError>;
async fn mark_refresh_token_used(&self, token_id: i64) -> Result<(), AppError>;
async fn revoke_user_refresh_tokens(&self, user_id: i64) -> Result<(), AppError>;

// Password Reset Tokens (línea ~41)
async fn create_password_reset_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<PasswordResetToken, AppError>;
async fn get_password_reset_token(&self, token: &str) -> Result<Option<PasswordResetToken>, AppError>;
async fn mark_password_reset_token_used(&self, token_id: i64) -> Result<(), AppError>;

// Email Verification Tokens (línea ~46)
async fn create_email_verification_token(&self, user_id: i64, token: &str, expires_at: &str) -> Result<EmailVerificationToken, AppError>;
async fn get_email_verification_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, AppError>;
async fn mark_email_verification_token_used(&self, token_id: i64) -> Result<(), AppError>;
```

#### 4. Implementación del Repositorio (ubicación: `backend/src/data/user_repository.rs`)

**SQL queries actualizados para incluir email_verified:**
- `create_user()` - Línea 23: Ahora retorna email_verified
- `get_by_username()` - Línea 48: Incluye email_verified
- `get_by_id()` - Línea 58: Incluye email_verified
- `get_all()` - Líneas 78, 83: Incluye email_verified
- `update_user()` - Línea 153: Resetea email_verified a FALSE al cambiar email
- `update_avatar()` - Línea 164: Incluye email_verified
- `get_by_email()` - Línea 216: Incluye email_verified

**Implementaciones nuevas (al final del archivo):**
- `update_password()` - Línea ~224
- `verify_email()` - Línea ~232
- `create_password_reset_token()` - Línea ~235
- `get_password_reset_token()` - Línea ~247
- `mark_password_reset_token_used()` - Línea ~257
- `create_email_verification_token()` - Línea ~272
- `get_email_verification_token()` - Línea ~284
- `mark_email_verification_token_used()` - Línea ~294

#### 5. Servicio de Email (ubicación: `backend/src/services/`)

**email.rs** (NUEVO ARCHIVO)
```rust
// Ubicación: backend/src/services/email.rs
// Propsito: Enviar emails SMTP con templates HTML

pub struct EmailService {
    mailer: AsyncSmtpTransport<lettre::Tokio1Executor>,
    from_email: String,
}

// Métodos:
pub async fn send_password_reset(&self, to_email: &str, reset_token: &str, username: &str) -> Result<(), String>
pub async fn send_email_verification(&self, to_email: &str, verification_token: &str, username: &str) -> Result<(), String>
pub fn create_email_service() -> Option<EmailService>  // Factory desde variables de entorno
```

**Variables de entorno necesarias:**
```bash
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tu-email@gmail.com
SMTP_PASS=tu-password
FROM_EMAIL=noreply@sintonia3026.com
```

**mod.rs** (NUEVO ARCHIVO)
```rust
// Ubicación: backend/src/services/mod.rs
pub mod email;
```

#### 6. Handlers de Usuario (ubicación: `backend/src/api/handlers/user.rs`)

**Endpoints modificados:**

1. **login()** - Línea ~134
   - ANTES: Generaba 1 token de 24 horas
   - AHORA: Genera access_token (15 min) + refresh_token (7 días)
   - Guarda refresh_token en base de datos
   - Retorna TokenResponse completo

2. **create_user()** - Línea ~40
   - AHORA: Si el usuario tiene email, genera token de verificación automáticamente
   - Envía email de verificación (o lo loguea en desarrollo)
   - Token válido por 24 horas

**Endpoints NUEVOS:**

3. **export_users()** - Línea ~347
   - GET /api/v1/users/export
   - Requiere: Admin
   - Retorna: CSV con todos los usuarios
   - Headers: Content-Type: text/csv, Content-Disposition: attachment

4. **export_audit_logs()** - Línea ~393
   - GET /api/v1/audit-logs/export
   - Requiere: Admin
   - Retorna: CSV con logs de auditoría

5. **upload_avatar()** - Línea ~439
   - POST /api/v1/users/avatar
   - Content-Type: multipart/form-data
   - Validación: Solo imágenes (image/*), máximo 2MB
   - Guarda en: uploads/{user_id}_{timestamp}.{ext}
   - Retorna: User actualizado con avatar_url

6. **refresh_token()** - Línea ~577
   - POST /api/v1/refresh
   - Recibe: refresh_token en body
   - Valida: Token existe, no usado, no expirado
   - Marca token anterior como usado (rotación)
   - Genera nuevo access_token (15 min) + refresh_token (7 días)
   - Actualiza cookie auth_token

7. **forgot_password()** - Línea ~630
   - POST /api/v1/forgot-password
   - Recibe: email
   - Si email existe: Genera token, envía email
   - Si no existe: Retorna mismo mensaje (seguridad)
   - Token válido por 1 hora

8. **reset_password()** - Línea ~690
   - POST /api/v1/reset-password
   - Recibe: token, new_password
   - Valida: Token existe, no usado, no expirado
   - Actualiza contraseña con hash Argon2
   - Marca token como usado
   - Revoca todos los refresh tokens del usuario

9. **send_verification_email()** - Línea ~743
   - POST /api/v1/send-verification-email
   - Requiere: Autenticación
   - Verifica que usuario tenga email y no esté verificado
   - Genera token válido por 24 horas
   - Envía email con enlace de verificación

10. **verify_email()** - Línea ~791
    - GET /api/v1/verify-email?token=xxx
    - Valida: Token existe, no usado, no expirado
    - Marca email_verified = TRUE en usuario
    - Marca token como usado

#### 7. Router (ubicación: `backend/src/lib.rs`)

**Rutas públicas (línea ~87):**
```rust
.route("/forgot-password", post(api::handlers::user::forgot_password))
.route("/reset-password", post(api::handlers::user::reset_password))
.route("/verify-email", get(api::handlers::user::verify_email))
```

**Rutas protegidas (línea ~93):**
```rust
.route("/send-verification-email", post(api::handlers::user::send_verification_email))
```

**Archivos estáticos (línea ~121):**
```rust
.nest_service("/uploads", ServeDir::new("uploads"))
```

**Dependencias nuevas en Cargo.toml:**
- `csv = "1.3.0"` - Exportación CSV
- `uuid = { version = "1.10", features = ["v4"] }` - Generación de tokens únicos
- `lettre = { version = "0.11", features = ["tokio1-native-tls", "builder"] }` - Emails SMTP

---

### 🎨 FRONTEND

#### 1. ApiClient (ubicación: `frontend/src/lib/api.ts`)

**Auto-refresh implementado (líneas ~30-124):**
```typescript
class ApiClient {
  private refreshPromise: Promise<void> | null = null
  
  // Guarda refresh_token en localStorage
  private getRefreshToken(): string | null
  private setRefreshToken(token: string | null)
  
  // Realiza el refresh
  private async doRefresh(): Promise<void>
  
  // Maneja refresh concurrente (evita múltiples requests)
  private async refreshAccessToken(): Promise<void>
  
  // Request automático con retry en 401
  private async request<T>(endpoint: string, options?: RequestInit): Promise<T>
}
```

**Métodos NUEVOS:**

```typescript
// Línea ~127
async login(credentials: LoginRequest): Promise<TokenResponse>
// Guarda refresh_token en localStorage automáticamente

// Línea ~139
async logout()
// Limpia refresh_token de localStorage

// Línea ~176
async exportUsers()
// Descarga CSV de usuarios

// Línea ~203
async exportAuditLogs()
// Descarga CSV de auditoría

// Línea ~230
async uploadAvatar(file: File): Promise<User>
// Sube imagen de avatar

// Línea ~250
async forgotPassword(email: string): Promise<{ message: string }>
// Solicita recuperación de contraseña

// Línea ~256
async resetPassword(token: string, newPassword: string): Promise<{ message: string }>
// Restablece contraseña con token

// Línea ~262
async sendVerificationEmail(): Promise<{ message: string }>
// Reenvía email de verificación (requiere auth)

// Línea ~268
async verifyEmail(token: string): Promise<{ message: string }>
// Verifica email con token
```

#### 2. Páginas Nuevas

**forgot-password.astro** (NUEVO)
```
Ubicación: frontend/src/pages/forgot-password.astro
URL: http://localhost:4321/forgot-password/
Función: Formulario para solicitar recuperación de contraseña
Props: Ninguna
Estado: Pública (no requiere auth)
```

**reset-password.astro** (NUEVO)
```
Ubicación: frontend/src/pages/reset-password.astro
URL: http://localhost:4321/reset-password?token=xxx
Función: Formulario para crear nueva contraseña
Query Params: token (obligatorio)
Estado: Pública
Validaciones: Contraseña mínimo 8 caracteres, confirmación debe coincidir
```

**verify-email.astro** (NUEVO)
```
Ubicación: frontend/src/pages/verify-email.astro
URL: http://localhost:4321/verify-email?token=xxx
Función: Verifica email automáticamente al cargar
Query Params: token (obligatorio)
Estado: Pública
Comportamiento: Llama a api.verifyEmail() automáticamente al cargar la página
```

#### 3. Páginas Modificadas

**login.astro** (MODIFICADO)
```
Ubicación: frontend/src/pages/login.astro
Cambios:
- Línea ~48: Agregado enlace "¿Olvidaste tu contraseña?" que redirige a /forgot-password/
- Línea ~116: Cambiado response.token por response.access_token (nuevo formato de respuesta)
```

**settings.astro** (MODIFICADO)
```
Ubicación: frontend/src/pages/dashboard/settings.astro
Cambios:

1. Sección de Avatar (línea ~67):
   - Agregado <img> para mostrar avatar real
   - Agregado fallback con iniciales si no hay avatar
   - Agregado spinner de carga durante upload
   - Agregado botón "Eliminar avatar"

2. Campo de Email (línea ~109):
   - Agregado indicador de estado: Verificado (verde) / No verificado (ámbar)
   - Agregado botón "Reenviar email de verificación" (solo si no verificado)

3. Función loadUserData() (línea ~356):
   - Ahora maneja avatar_url
   - Muestra estado de verificación de email
   - Actualiza UI según email_verified

4. Event Listeners (línea ~639):
   - Agregado listener para resend-verification-btn
   - Agregado listener para upload-avatar-btn con validación de archivo
```

---

## 🧪 CÓMO PROBAR CADA FEATURE

### 1. Exportación CSV

**Backend:**
```bash
# Compilar
$ cd backend && cargo run

# Probar endpoint (requiere ser Admin)
$ curl -H "Cookie: auth_token=xxx" http://localhost:3000/api/v1/users/export
# Debería descargar users_export.csv

$ curl -H "Cookie: auth_token=xxx" http://localhost:3000/api/v1/audit-logs/export
# Debería descargar audit_logs_export.csv
```

**Frontend:**
```
1. Ir a http://localhost:4321/dashboard/users/
2. Click en "Exportar CSV" (arriba a la derecha)
3. Verificar que se descarga archivo CSV
4. Ir a http://localhost:4321/dashboard/audit/
5. Click en "Exportar CSV"
6. Verificar descarga
```

### 2. Avatar Upload

**Backend:**
```bash
# El directorio uploads/ se crea automáticamente
# Verificar que el archivo se guarda en: uploads/{user_id}_{timestamp}.{ext}
```

**Frontend:**
```
1. Ir a http://localhost:4321/dashboard/settings/
2. Click en "Cambiar avatar"
3. Seleccionar imagen (JPG, PNG, GIF, máximo 2MB)
4. Verificar:
   - Aparece spinner de carga
   - Se muestra la imagen (no las iniciales)
   - Aparece botón "Eliminar avatar"
5. Recargar página - imagen debe persistir
```

### 3. Refresh Tokens

**Backend:**
```bash
# Login y obtener tokens
$ curl -X POST http://localhost:3000/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"password"}'
# Respuesta: {"access_token":"...","refresh_token":"...","expires_in":900}

# Usar refresh token después de 15 minutos
$ curl -X POST http://localhost:3000/api/v1/refresh \
  -H "Content-Type: application/json" \
  -d '{"refresh_token":"xxx"}'
# Respuesta: Nuevos access_token y refresh_token
# El token anterior ya no debe funcionar (rotación)
```

**Frontend:**
```
1. Login normal
2. Verificar en LocalStorage: refresh_token debe existir
3. Esperar 15 minutos (o modificar backend a 1 minuto para testing)
4. Hacer cualquier acción (ej: navegar a Settings)
5. Verificar en Network tab:
   - Primera request: 401 Unauthorized
   - Segunda request: POST /api/v1/refresh
   - Tercera request: Original retry con nuevo token
6. Todo debe ser transparente para el usuario
```

### 4. Recuperación de Contraseña

**Backend:**
```bash
# Solicitar reset
$ curl -X POST http://localhost:3000/api/v1/forgot-password \
  -H "Content-Type: application/json" \
  -d '{"email":"usuario@example.com"}'
# Ver logs del backend - debe aparecer token y URL

# Verificar token en base de datos
sqlite3 sintonia.db "SELECT * FROM password_reset_tokens;"

# Restablecer contraseña
$ curl -X POST http://localhost:3000/api/v1/reset-password \
  -H "Content-Type: application/json" \
  -d '{"token":"xxx","new_password":"nueva12345"}'

# Intentar usar mismo token de nuevo - debe fallar
```

**Frontend:**
```
1. Ir a http://localhost:4321/login/
2. Click en "¿Olvidaste tu contraseña?"
3. Ingresar email de usuario existente
4. Revisar logs del backend para obtener URL con token
5. Abrir URL en navegador (ej: http://localhost:4321/reset-password?token=abc-123)
6. Ingresar nueva contraseña (mínimo 8 caracteres)
7. Confirmar contraseña
8. Ver mensaje de éxito
9. Intentar login con nueva contraseña
```

### 5. Verificación de Email

**Backend:**
```bash
# Al registrar usuario con email, se genera token automáticamente
# Ver logs del backend después del registro

# Verificar token
sqlite3 sintonia.db "SELECT * FROM email_verification_tokens;"

# Verificar endpoint manualmente
$ curl "http://localhost:3000/api/v1/verify-email?token=xxx"

# Reenviar email (requiere auth)
$ curl -X POST http://localhost:3000/api/v1/send-verification-email \
  -H "Cookie: auth_token=xxx"
```

**Frontend:**
```
1. Registrar nuevo usuario con email
2. Revisar logs del backend para obtener URL: http://localhost:4321/verify-email?token=xxx
3. Abrir URL (simula click en email)
4. Verificar automáticamente:
   - Página muestra "Verificando tu email..."
   - Luego muestra "¡Email verificado!"
5. Ir a Settings
6. Verificar que email muestra badge "Verificado" en verde

Caso 2 - Reenviar verificación:
1. Ir a Settings
2. Si email no está verificado, aparece badge "No verificado" en ámbar
3. Click en "Reenviar email de verificación"
4. Revisar logs para nueva URL
5. Repetir proceso de verificación
```

---

## 🔍 DÓNDE BUSCAR ERRORES

### Si export CSV no funciona:
- Revisar: backend/src/api/handlers/user.rs línea 347-390
- Verificar que el usuario tiene rol Admin
- Revisar logs de backend

### Si avatar no se sube:
- Revisar: backend/src/api/handlers/user.rs línea 439-513
- Verificar que directorio uploads/ tiene permisos de escritura
- Revisar Content-Type del request (debe ser multipart/form-data)
- Verificar tamaño del archivo (máximo 2MB)

### Si refresh tokens no funcionan:
- Revisar: frontend/src/lib/api.ts línea 30-124
- Verificar que refresh_token se guarda en localStorage
- Revisar que /api/v1/refresh retorna nuevos tokens
- Verificar que cookies se actualizan

### Si emails no se envían:
- Revisar: backend/src/services/email.rs
- Verificar variables de entorno SMTP_* configuradas
- En desarrollo, revisar logs - los tokens se imprimen ahí
- Verificar que lettre está compilado con feature "tokio1-native-tls"

### Si verificación de email no funciona:
- Revisar: backend/src/api/handlers/user.rs línea 743-825
- Verificar que usuario tiene email en base de datos
- Revisar que token no está expirado (24 horas)
- Verificar que token no fue usado antes

---

## 📝 NOTAS IMPORTANTES

### Seguridad implementada:
1. **Refresh Tokens**: Rotación obligatoria (token usado = invalidado)
2. **Password Reset**: Tokens de un solo uso, expiran en 1 hora
3. **Email Verification**: Tokens de un solo uso, expiran en 24 horas
4. **Avatar**: Validación de tipo y tamaño, nombres de archivo únicos
5. **CSV**: Solo accesible por Admin

### En desarrollo vs producción:
**Desarrollo:**
- Emails se loguean en consola (no se envían realmente)
- Tokens se imprimen para testing manual
- Expiraciones pueden ser modificadas para testing

**Producción:**
- Configurar variables SMTP_* para envío real
- Revisar tiempos de expiración (15min access, 7días refresh)
- Configurar servidor SMTP confiable

### Cambios en respuestas API:
**Login ahora retorna:**
```json
{
  "user": { ... },
  "access_token": "...",
  "refresh_token": "...",
  "expires_in": 900,
  "token_type": "Bearer"
}
```

**En lugar del antiguo:**
```json
{
  "user": { ... },
  "token": "...",
  "message": "Login exitoso"
}
```

**IMPORTANTE**: Frontend actualizado para usar access_token en lugar de token.

---

## 🎓 COMANDOS ÚTILES

```bash
# Limpiar tokens usados/expirados (manual)
sqlite3 sintonia.db "DELETE FROM refresh_tokens WHERE used = TRUE OR datetime(expires_at) < datetime('now');"
sqlite3 sintonia.db "DELETE FROM password_reset_tokens WHERE used = TRUE OR datetime(expires_at) < datetime('now');"
sqlite3 sintonia.db "DELETE FROM email_verification_tokens WHERE used = TRUE OR datetime(expires_at) < datetime('now');"

# Ver estado de un usuario
sqlite3 sintonia.db "SELECT id, username, email, email_verified FROM users WHERE username = 'test';"

# Contar tokens activos
sqlite3 sintonia.db "SELECT COUNT(*) FROM refresh_tokens WHERE used = FALSE;"

# Ver últimos logs
cd backend && cargo run 2>&1 | grep -E "(verification|reset|avatar|export)"
```

---

## ✅ CHECKLIST FINAL

- [ ] Todas las migraciones aplicadas (0006-0009)
- [ ] Backend compila sin errores
- [ ] Frontend compila sin errores
- [ ] Login funciona y genera refresh_token
- [ ] Export CSV funciona (Admin)
- [ ] Avatar upload funciona
- [ ] Auto-refresh funciona (esperar expiración o modificar tiempos)
- [ ] Password reset flujo completo funciona
- [ ] Email verification flujo completo funciona
- [ ] Indicadores de verificación visibles en Settings

---

**Fase 38 - 100% Completada** ✅
**Última actualización:** 2026-02-16
**Total de archivos modificados/creados:** 20+
**Total de líneas agregadas:** ~2000+
