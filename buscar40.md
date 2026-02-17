# 📋 GUÍA COMPLETA - FASE 40 (GESTIÓN AVANZADA)

## 🎯 Resumen
Fase 40 completada. Se ha transformado el sistema de una gestión de usuarios básica a una **Suite de Administración Enterprise**. Ahora es posible editar usuarios, cambiar contraseñas internamente y, lo más importante, gestionar Roles y Permisos (RBAC) de forma dinámica desde la base de datos y la UI.

---

## 📁 ESTRUCTURA DE ARCHIVOS MODIFICADOS/CREADOS

### 1. Gestión de Usuarios (CRUD Completo)

**frontend/src/pages/dashboard/users/[id].astro** (NUEVO)
- **Qué hace:** Página de edición de usuario individual.
- **Ubicación:** `/dashboard/users/123`
- **Lógica:** Carga datos del usuario (SSR) y permite editar email y rol mediante un formulario interactivo.
- **Clave:** Solo los administradores pueden ver el selector de "Rol".

**backend/src/api/handlers/user.rs** (MODIFICADO)
- **Cambio:** `update_user` ahora acepta el campo `role`.
- **Seguridad:** Se agregó validación para que solo un Admin pueda cambiar el rol de otro usuario.

### 2. Cambio de Contraseña Interno

**frontend/src/pages/dashboard/settings.astro** (MODIFICADO)
- **Cambio:** Se conectó el formulario de "Cambiar Contraseña" con la API real.
- **Uso:** Permite al usuario logueado rotar su clave sin usar el flujo de "Olvidé mi contraseña".

**backend/src/api/handlers/user.rs** (NUEVO ENDPOINT)
- **Endpoint:** `PUT /api/v1/users/password`
- **Lógica:** Verifica la contraseña actual antes de hashear y guardar la nueva. Revoca sesiones antiguas por seguridad.

### 3. Roles y Permisos (RBAC Dinámico)

#### Base de Datos & Modelos
**backend/migrations/0010_create_rbac_tables.sql** (NUEVO)
- **Qué hace:** Crea tablas `roles`, `permissions` y `role_permissions`.
- **Seed:** Inserta roles 'Admin' y 'User' y permisos base para compatibilidad.

**backend/src/core/models/user.rs** (MODIFICADO)
- **Nuevos Structs:** `DbRole`, `Permission`, `RolePermission`, `CreateRoleRequest`, `UpdateRoleRequest`.

#### Backend API
**backend/src/data/user_repository.rs** (MODIFICADO)
- **Métodos:** `get_roles`, `create_role`, `update_role`, `delete_role`, `get_permissions`.
- **Lógica:** Implementación SQL con transacciones para manejar la relación muchos-a-muchos.

**backend/src/api/handlers/user.rs** (NUEVO)
- **Endpoints:** CRUD completo para roles (`/api/v1/roles`).

#### Frontend UI
**frontend/src/components/dashboard/roles/roles-matrix.tsx** (NUEVO - React)
- **Qué hace:** Componente complejo que renderiza la matriz de permisos.
- **Interactividad:** Permite crear/editar roles y marcar/desmarcar permisos en un modal.

**frontend/src/pages/dashboard/roles.astro** (NUEVO)
- **Qué hace:** Página contenedora que carga los datos iniciales y monta la `RolesMatrix`.

**frontend/src/components/layout/sidebar.astro** (MODIFICADO)
- **Cambio:** Agregado enlace "Roles y Permisos" visible solo para Admins.

---

## 🧪 CÓMO PROBAR CADA FEATURE

### 1. Edición de Usuario
**Ubicación:** `/dashboard/users`
**Pasos:**
1. Entra como Admin.
2. Haz clic en "Editar" en cualquier usuario de la tabla.
3. **Prueba:** Cambia el email y el rol (ej: de User a Admin).
4. Guarda y verifica que al volver a la lista el cambio persiste.
5. **Prueba de Seguridad:** Intenta cambiar el rol de tu propio usuario (debería funcionar) o entra como usuario normal e intenta acceder a la URL de edición (debería redirigir o mostrar error).

### 2. Cambio de Contraseña
**Ubicación:** `/dashboard/settings`
**Pasos:**
1. Ve a la pestaña "Seguridad".
2. **Prueba Fallida:** Ingresa mal tu contraseña actual -> Debe mostrar error.
3. **Prueba Exitosa:** Ingresa bien la actual y una nueva -> Debe mostrar éxito.
4. Haz logout e intenta entrar con la nueva contraseña.

### 3. Gestión de Roles (La Joya de la Corona)
**Ubicación:** `/dashboard/roles`
**Pasos:**
1. **Visualización:** Verifica que ves la tabla con "Admin" y "User" y sus permisos marcados.
2. **Crear:**
   - Clic en "Nuevo Rol".
   - Nombre: "Auditor".
   - Permisos: Marca solo "audit:read" y "audit:export".
   - Guardar.
3. **Verificación:** El nuevo rol debe aparecer en la matriz como una nueva columna.
4. **Asignación:** Ve a `/dashboard/users`, edita un usuario y asígnale el rol "Auditor" (Nota: El backend actual usa un Enum `Role` hardcodeado para la lógica de negocio principal, por lo que este cambio es visual/preparatorio para la migración total a RBAC dinámico en el futuro, pero el dato se guarda en la DB).

---

## 🛠️ GUÍA DE MANTENIMIENTO

### ¿Cómo agregar un nuevo permiso al sistema?
1. **DB:** Inserta el permiso en la tabla `permissions` (puedes hacerlo vía SQL o crear un endpoint/UI para ello si lo deseas).
   ```sql
   INSERT INTO permissions (name, description) VALUES ('reports:view', 'Ver reportes financieros');
   ```
2. **Frontend:** Automáticamente aparecerá en la matriz de `/dashboard/roles` para ser asignado.

### ¿Cómo cambiar la lógica de un rol?
1. Ve a `/dashboard/roles`.
2. Edita el rol.
3. Marca/desmarca permisos.
4. **Backend:** En el futuro, usarás `user.permissions.contains('reports:view')` en lugar de `user.role == Role::Admin`.

---

## 📦 DEPENDENCIAS CLAVE

- **Backend:** `sqlx` (Transacciones complejas), `axum` (Ruteo).
- **Frontend:** `lucide-react` (Iconos), `radix-ui` (Dialogs), `astro` (SSR).
```

<!--
[PROMPT_SUGGESTION]Perfecto. Ahora pasemos a la Fase 41: Configuración de Playwright para pruebas E2E.[/PROMPT_SUGGESTION]
[PROMPT_SUGGESTION]Quiero crear el primer test E2E para verificar el Login (frontend/tests/login.spec.ts).[/PROMPT_SUGGESTION]
