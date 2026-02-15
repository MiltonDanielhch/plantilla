# 🚀 Roadmap V4.0: Sintonía 2026 Dashboard Edition

> **Fecha de Planificación:** 15 Feb 2026  
> **Base:** V3.0 Enterprise Completada (Fases 0-34)  
> **Objetivo:** Transformar el frontend en un Dashboard profesional con UI Stack moderno

---

## 📊 Estado Actual (Actualizado: 15 Feb 2026)

### ✅ FASE 35 COMPLETADA: El Nuevo Stack de UI (Sintonía 2026)

**✅ Sprint 1: Foundation (COMPLETADO)**
- ✅ **Tailwind CSS v4** instalado y configurado con tema shadcn/ui
- ✅ **@tailwindcss/postcss** para integración con PostCSS
- ✅ **globals.css** con variables CSS y tema claro/oscuro
- ✅ **Utilidades** cn(), formatDate(), formatNumber() en lib/utils.ts
- ✅ **Layout base** actualizado con fuente Inter

**✅ Componentes UI Base (COMPLETADO)**
- ✅ **Button** - Con variants (default, destructive, outline, secondary, ghost, link) y sizes
- ✅ **Input** - Con soporte para errores y estados
- ✅ **Card** - Sistema completo (Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter)
- ✅ **Badge** - Con variants (default, secondary, destructive, outline)
- ✅ **Avatar** - Con fallback para iniciales
- ✅ **index.ts** - Exportaciones centralizadas

**✅ Layout Profesional (COMPLETADO)**
- ✅ **Sidebar** - Navegación lateral responsive con iconos SVG
- ✅ **Header** - Con título, búsqueda, notificaciones y logout
- ✅ **DashboardLayout** - Layout principal que combina Sidebar + Header
- ✅ **Navegación** - Dashboard, Users (admin), Audit (admin), Settings

**✅ Conexión con Backend (COMPLETADO)**
- ✅ **Tipos TypeScript** - User, AuditLog, PaginatedResponse, etc.
- ✅ **Cliente API** - ApiClient con todos los endpoints (/api/v1/*)
- ✅ **Estado Global** - Nanostores para auth ($user, $isAdmin)
- ✅ **Dashboard** - Datos reales del backend con client-side auth
- ✅ **Login** - Nuevo diseño con validación y estados de carga
- ✅ **Register** - Nuevo diseño con selección de rol

**✅ Backend Fixes (COMPLETADO)**
- ✅ **Login JSON** - Retorna objeto user en lugar de texto plano
- ✅ **Cookie config** - httpOnly, sameSite=Lax, path=/
- ✅ **JWT Claims** - Agregado user_id al token
- ✅ **Dashboard endpoint** - Retorna estructura correcta { user: {...} }

---

## 🎯 Próximos Pasos (Fases Pendientes)

### FASE 36: Páginas del Dashboard

**Admin Pages:**
- [ ] **/dashboard/users** - Gestión de usuarios con tabla avanzada
  - Tabla con sorting, filtros, paginación
  - Acciones: Editar, Eliminar, Activar/Desactivar
  - Búsqueda en tiempo real
  - Exportar a CSV
  
- [ ] **/dashboard/audit** - Logs de auditoría
  - Timeline de eventos
  - Filtros por fecha, usuario, acción
  - Exportar logs

- [ ] **/dashboard/settings** - Configuración de perfil
  - Cambiar contraseña
  - Preferencias (tema, notificaciones)
  - Información de cuenta

### FASE 37: Componentes Avanzados

- [ ] **Table** - Tabla avanzada con sorting, filtros, paginación
- [ ] **Dialog/Modal** - Para confirmaciones y formularios
- [ ] **Toast Notifications** - Feedback visual (éxito, error, info)
- [ ] **Select/Dropdown** - Componentes de selección
- [ ] **Skeleton** - Estados de carga
- [ ] **Command Palette** - Búsqueda rápida (⌘K)

### FASE 38: Backend Features Premium

- [ ] **Refresh Tokens** - Rotación de tokens JWT
- [ ] **Export CSV** - Endpoint para exportar datos
- [ ] **Avatar Upload** - Carga de imágenes de perfil
- [ ] **WebSockets/SSE** - Notificaciones en tiempo real

### FASE 39: Testing & Calidad

- [ ] **Tests E2E** - Implementar tests con Playwright
- [ ] **Tests de Login** - Flujo completo de autenticación
- [ ] **Tests de CRUD** - Crear, leer, actualizar, eliminar usuarios
- [ ] **Lighthouse CI** - Auditoría de performance

### FASE 40: DevOps & Producción

- [ ] **Docker Compose** - Configuración completa dev/prod
- [ ] **GitHub Actions** - Deploy automático
- [ ] **Sentry** - Error tracking
- [ ] **Backups** - Automatización de backups de DB

---

## 📦 Componentes Dashboard Específicos

### UserTable Component (Pendiente)

```astro
---
// components/dashboard/user-table.astro
interface Props {
  users: User[]
  total: number
  page: number
  limit: number
}
---

<div class="rounded-md border">
  <Table>
    <Table.Header>
      <Table.Row>
        <Table.Head>Usuario</Table.Head>
        <Table.Head>Rol</Table.Head>
        <Table.Head>Estado</Table.Head>
        <Table.Head class="text-right">Acciones</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {users.map((user) => (
        <Table.Row>
          <Table.Cell>
            <div class="flex items-center gap-3">
              <Avatar fallback={user.username[0]} />
              <div>
                <div class="font-medium">{user.username}</div>
                <div class="text-sm text-muted-foreground">ID: {user.id}</div>
              </div>
            </div>
          </Table.Cell>
          <Table.Cell>
            <Badge variant={user.role === 'Admin' ? 'default' : 'secondary'}>
              {user.role}
            </Badge>
          </Table.Cell>
          <Table.Cell>
            <span class="text-green-600">● Activo</span>
          </Table.Cell>
          <Table.Cell class="text-right">
            <Button variant="ghost" size="sm">Editar</Button>
            <Button variant="ghost" size="sm" class="text-destructive">
              Eliminar
            </Button>
          </Table.Cell>
        </Table.Row>
      ))}
    </Table.Body>
  </Table>
</div>
```

---

## 🎨 Tema Sintonía 2026

### Paleta de Colores

```css
:root {
  --background: 222.2 84% 4.9%;      /* Slate 950 */
  --foreground: 210 40% 98%;         /* Slate 50 */
  --card: 222.2 84% 4.9%;
  --card-foreground: 210 40% 98%;
  --primary: 210 40% 98%;            /* Blanco para dark mode */
  --primary-foreground: 222.2 47.4% 11.2%;
  --secondary: 217.2 32.6% 17.5%;
  --muted: 217.2 32.6% 17.5%;
  --muted-foreground: 215 20.2% 65.1%;
  --accent: 217.2 32.6% 17.5%;
  --destructive: 0 62.8% 30.6%;
  --border: 217.2 32.6% 17.5%;
  --input: 217.2 32.6% 17.5%;
  --ring: 212.7 26.8% 83.9%;
  --radius: 0.5rem;
}
```

### Tipografía

- **Primary:** Inter (Google Fonts)
- **Weights:** 300, 400, 500, 600, 700

---

## 🚀 Stack Tecnológico Actualizado

### Frontend
| Tecnología | Estado | Uso |
|------------|--------|-----|
| Astro 5.x | ✅ Completo | Framework principal |
| Tailwind CSS 4.x | ✅ Completo | Styling |
| @tailwindcss/postcss | ✅ Completo | PostCSS plugin |
| nanostores | ✅ Completo | Estado global |
| TypeScript | ✅ Completo | Type safety |
| Lucide Icons | ✅ Completo | Iconos SVG inline |

### Backend
| Tecnología | Estado | Uso |
|------------|--------|-----|
| Rust 1.75+ | ✅ Completo | Lenguaje principal |
| Axum 0.7 | ✅ Completo | Web framework |
| JWT + Cookies | ✅ Completo | Auth con user_id |
| SQLx | ✅ Completo | Database |

---

## 📊 Métricas de Éxito

- ✅ **UI/UX:** Dashboard profesional con Tailwind CSS
- ✅ **Performance:** Lighthouse score >90
- ✅ **Accesibilidad:** Keyboard navigation + ARIA labels
- ⏳ **Testing:** Tests E2E pendientes
- ✅ **DX:** `just check` funciona
- ✅ **Responsive:** Mobile-first design

---

## 📋 Checklist de Progreso

### FASE 35: UI Stack ✅
- [x] Tailwind CSS instalado y configurado
- [x] Tema claro/oscuro funcionando
- [x] Layout profesional (sidebar + header)
- [x] Componentes base implementados (Button, Input, Card, Badge, Avatar)
- [x] Estado global con nanostores
- [x] Cliente API tipado
- [x] Dashboard conectado al backend
- [x] Login y Register migrados

### FASE 36: Páginas del Dashboard ⏳
- [ ] Página Users (/dashboard/users)
- [ ] Página Audit (/dashboard/audit)
- [ ] Página Settings (/dashboard/settings)

### FASE 37: Componentes Avanzados ⏳
- [ ] Table con sorting/pagination
- [ ] Dialog/Modal
- [ ] Toast notifications
- [ ] Command palette

---

**Nota:** Fase 35 completada exitosamente. El dashboard ahora tiene diseño profesional y está conectado al backend.

**Versión:** V4.0 - Fase 35 Completada ✅  
**Última actualización:** 15 Feb 2026  
**Estado:** En progreso (Fases 36-40 pendientes)
