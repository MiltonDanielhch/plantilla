# 🚀 Roadmap V4.0: Sintonía 2026 Dashboard Edition

> **Fecha de Planificación:** 15 Feb 2026  
> **Base:** V3.0 Enterprise Completada (Fases 0-34)  
> **Objetivo:** Transformar el frontend en un Dashboard profesional con UI Stack moderno

---

## 📊 Análisis del Estado Actual

### ✅ Lo que YA TIENES IMPLEMENTADO (Fases 0-34 Completadas)

**Backend (Rust + Axum) - Enterprise Ready:**
- ✅ **Arquitectura Hexagonal** - core/data/api separados
- ✅ **Autenticación JWT** - Cookies HttpOnly + SameSite
- ✅ **Autorización RBAC** - Admin/User roles con middleware
- ✅ **Repository Pattern** - UserRepository trait + SqliteRepository
- ✅ **Errores Tipados** - AppError con IntoResponse
- ✅ **Configuración Jerárquica** - Crate config con entornos
- ✅ **Observabilidad** - Tracing JSON + request IDs
- ✅ **Rate Limiting** - 10 req/seg con Governor
- ✅ **Paginación** - LIMIT/OFFSET implementado (Fase 27)
- ✅ **Búsqueda/Filtros** - UserSearch con query params (Fase 18)
- ✅ **API Versionada** - /api/v1
- ✅ **Swagger/OpenAPI** - Utoipa con UI
- ✅ **Graceful Shutdown** - Señales SIGTERM (Fase 28)
- ✅ **Health Checks** - Verificación DB (Fase 28)
- ✅ **Auditoría** - Tabla audit_logs completa (Fase 17)
- ✅ **SQLx + SQLite** - Migrations automáticas
- ✅ **CORS** - Configurado para localhost:4321

**Testing & DevOps:**
- ✅ **Tests Unitarios** - Backend >80% coverage (Fase 22)
- ✅ **Tests Integración** - SQLx + test database (Fase 22)
- ✅ **Tests E2E Configurados** - Playwright listo (Fase 24)
- ✅ **CI/CD** - GitHub Actions workflow (Fase 23)
- ✅ **Docker Multi-Stage** - Optimizado para prod (Fase 19)
- ✅ **Justfile** - Comandos unificados run-backend, run-frontend, check

**Frontend (Astro) - Básico Funcional:**
- ✅ **Estructura Astro** - Proyecto base
- ✅ **Páginas** - index, login, register, dashboard
- ✅ **Componentes** - LoginForm, UserForm, UserList, AuditTable, LogoutButton
- ✅ **Dashboard con Tabs** - Users/Audit para admins
- ✅ **Integración Backend** - Fetch con credentials:include

---

## ⚠️ Lo que REALMENTE FALTA (Gap Analysis)

### Frontend - UI/UX (La mayor prioridad)
- ❌ **Tailwind CSS** - No instalado (solo estilos inline)
- ❌ **Sistema de Diseño** - No hay componentes reutilizables
- ❌ **Tema Claro/Oscuro** - Actualmente solo modo oscuro hardcodeado
- ❌ **Layout Profesional** - Falta sidebar, header, breadcrumbs
- ❌ **Tablas Avanzadas** - UserList actual es muy básico (sin sorting visual)
- ❌ **Notificaciones/Toasts** - Sin feedback visual
- ❌ **Modales** - Confirmaciones inline en lugar de dialogs
- ❌ **Iconos** - Sin sistema de iconos (Lucide)
- ❌ **Loading States** - Sin skeletons
- ❌ **Formularios** - Sin validación visual ni error handling
- ❌ **Estado Global** - Auth manejado localmente en cada componente

### Backend - Features Premium (Nice to have)
- ❌ **Refresh Tokens** - JWT actual sin rotación
- ❌ **Exportación CSV** - Endpoint para exportar users/logs
- ❌ **Avatar Upload** - Carga de imágenes de perfil
- ❌ **WebSockets/SSE** - Notificaciones en tiempo real
- ❌ **Caché Redis** - Cache de sesiones/queries

### DevOps - Producción Real
- ❌ **Tests E2E Reales** - Playwright configurado pero sin tests implementados
- ❌ **Docker Compose** - Solo Dockerfile, falta compose.yml
- ❌ **Deployment** - No hay despliegue automatizado
- ❌ **Monitoring** - Sin Sentry/LogRocket
- ❌ **Backups DB** - Automatización de backups

---

## 🎨 FASE 35: El Nuevo Stack de UI (Sintonía 2026)

> **Prioridad: ALTA** - Transformar el frontend en un dashboard profesional

### 1. shadcn/ui para Astro 🎯

**¿Por qué shadcn/ui?**
- **Ownership total:** Tú eres el dueño del código
- **Minimalista industrial:** Perfecto para tu estética actual
- **Actualizado semanalmente:** Mantenimiento activo
- **Tailwind-first:** Integración perfecta
- **Accesible:** a11y incluido
- **Type-safe:** TypeScript first

**Instalación paso a paso:**

```bash
# 1. Entrar al frontend
cd frontend

# 2. Instalar Tailwind CSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 3. Instalar dependencias shadcn/ui
npm install -D @tailwindcss/typography class-variance-authority clsx tailwind-merge
npm install lucide-astro  # Iconos para Astro (no React)

# 4. Instalar nanostores para estado global
npm install nanostores

# 5. Configurar tailwind.config.mjs
cat > tailwind.config.mjs << 'EOF'
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
        secondary: {
          DEFAULT: 'hsl(var(--secondary))',
          foreground: 'hsl(var(--secondary-foreground))',
        },
        destructive: {
          DEFAULT: 'hsl(var(--destructive))',
          foreground: 'hsl(var(--destructive-foreground))',
        },
        muted: {
          DEFAULT: 'hsl(var(--muted))',
          foreground: 'hsl(var(--muted-foreground))',
        },
        accent: {
          DEFAULT: 'hsl(var(--accent))',
          foreground: 'hsl(var(--accent-foreground))',
        },
        popover: {
          DEFAULT: 'hsl(var(--popover))',
          foreground: 'hsl(var(--popover-foreground))',
        },
        card: {
          DEFAULT: 'hsl(var(--card))',
          foreground: 'hsl(var(--card-foreground))',
        },
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)',
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}
EOF

# 6. Crear globals.css con tema
mkdir -p src/styles
cat > src/styles/globals.css << 'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --card: 0 0% 100%;
    --card-foreground: 222.2 84% 4.9%;
    --popover: 0 0% 100%;
    --popover-foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
    --primary-foreground: 210 40% 98%;
    --secondary: 210 40% 96.1%;
    --secondary-foreground: 222.2 47.4% 11.2%;
    --muted: 210 40% 96.1%;
    --muted-foreground: 215.4 16.3% 46.9%;
    --accent: 210 40% 96.1%;
    --accent-foreground: 222.2 47.4% 11.2%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 210 40% 98%;
    --border: 214.3 31.8% 91.4%;
    --input: 214.3 31.8% 91.4%;
    --ring: 222.2 84% 4.9%;
    --radius: 0.5rem;
  }

  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    --card: 222.2 84% 4.9%;
    --card-foreground: 210 40% 98%;
    --popover: 222.2 84% 4.9%;
    --popover-foreground: 210 40% 98%;
    --primary: 210 40% 98%;
    --primary-foreground: 222.2 47.4% 11.2%;
    --secondary: 217.2 32.6% 17.5%;
    --secondary-foreground: 210 40% 98%;
    --muted: 217.2 32.6% 17.5%;
    --muted-foreground: 215 20.2% 65.1%;
    --accent: 217.2 32.6% 17.5%;
    --accent-foreground: 210 40% 98%;
    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 210 40% 98%;
    --border: 217.2 32.6% 17.5%;
    --input: 217.2 32.6% 17.5%;
    --ring: 212.7 26.8% 83.9%;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
  }
}
EOF

# 7. Actualizar Layout.astro para usar el tema
```

### 2. Estructura del Sistema de Diseño

```
frontend/src/
├── components/
│   ├── ui/                    # Componentes base (shadcn/ui style)
│   │   ├── button.astro
│   │   ├── input.astro
│   │   ├── card.astro
│   │   ├── dialog.astro
│   │   ├── dropdown-menu.astro
│   │   ├── table.astro
│   │   ├── tabs.astro
│   │   ├── toast.astro
│   │   ├── skeleton.astro
│   │   ├── badge.astro
│   │   ├── avatar.astro
│   │   └── select.astro
│   ├── layout/                # Layout components
│   │   ├── dashboard-layout.astro
│   │   ├── sidebar.astro
│   │   ├── header.astro
│   │   └── mobile-nav.astro
│   └── dashboard/             # Componentes específicos
│       ├── stats-cards.astro
│       ├── user-table.astro
│       ├── audit-timeline.astro
│       └── theme-toggle.astro
├── lib/
│   ├── utils.ts               # cn(), formatters
│   ├── api.ts                 # Cliente API tipado
│   └── auth.ts                # Helpers auth
├── stores/
│   ├── auth.ts                # Estado global auth
│   └── theme.ts               # Estado tema claro/oscuro
├── styles/
│   └── globals.css            # Variables CSS + Tailwind
└── types/
    └── index.ts               # Tipos TypeScript
```

### 3. Componentes Core (shadcn/ui para Astro)

Crear componentes base como Astro components:

```astro
---
// components/ui/button.astro
import { cn } from '../../lib/utils';

interface Props {
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
  size?: 'default' | 'sm' | 'lg' | 'icon';
  class?: string;
  href?: string;
  type?: 'button' | 'submit' | 'reset';
  disabled?: boolean;
}

const { 
  variant = 'default', 
  size = 'default', 
  class: className = '',
  href,
  type = 'button',
  disabled = false
} = Astro.props;

const variants = {
  default: 'bg-primary text-primary-foreground hover:bg-primary/90',
  destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
  outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
  secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
  ghost: 'hover:bg-accent hover:text-accent-foreground',
  link: 'text-primary underline-offset-4 hover:underline',
};

const sizes = {
  default: 'h-10 px-4 py-2',
  sm: 'h-9 rounded-md px-3',
  lg: 'h-11 rounded-md px-8',
  icon: 'h-10 w-10',
};

const classes = cn(
  'inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
  variants[variant],
  sizes[size],
  className
);
---

{href ? (
  <a href={href} class={classes}>
    <slot />
  </a>
) : (
  <button type={type} disabled={disabled} class={classes}>
    <slot />
  </button>
)}
```

### 4. Layout del Dashboard

```astro
---
// components/layout/dashboard-layout.astro
import Sidebar from './sidebar.astro';
import Header from './header.astro';
import { cn } from '../../lib/utils';

interface Props {
  title: string;
  description?: string;
}

const { title, description } = Astro.props;
---

<!DOCTYPE html>
<html lang="es" class="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width" />
  <title>{title} | Sintonía 3026</title>
  <meta name="description" content={description} />
</head>
<body class="min-h-screen bg-background font-sans antialiased">
  <div class="flex h-screen overflow-hidden">
    <Sidebar />
    <div class="flex flex-1 flex-col overflow-hidden">
      <Header title={title} />
      <main class="flex-1 overflow-auto p-6">
        <slot />
      </main>
    </div>
  </div>
</body>
</html>
```

### 5. Páginas Nuevas del Dashboard

Reemplazar las páginas actuales con versiones profesionales:

1. **/dashboard** - Overview con stats cards
2. **/dashboard/users** - User management con tabla avanzada
3. **/dashboard/audit** - Audit logs con timeline
4. **/dashboard/settings** - Perfil y preferencias

### 6. Estado Global con Nanostores

```typescript
// stores/auth.ts
import { atom, computed } from 'nanostores';

export interface User {
  id: number;
  username: string;
  role: 'Admin' | 'User';
}

export const $user = atom<User | null>(null);
export const $isLoading = atom(false);

export const $isAuthenticated = computed($user, user => user !== null);
export const $isAdmin = computed($user, user => user?.role === 'Admin');

export async function fetchUser() {
  $isLoading.set(true);
  try {
    const res = await fetch('/api/v1/dashboard', { credentials: 'include' });
    if (res.ok) {
      const data = await res.json();
      $user.set(data.user);
    } else {
      $user.set(null);
    }
  } catch {
    $user.set(null);
  } finally {
    $isLoading.set(false);
  }
}
```

---

## 🏗️ FASES RESTANTES DEL ROADMAP

### FASE 36: Backend Features Premium (Opcional)

**Prioridad: MEDIA** - Mejoras que agregan valor pero no son críticas

- [ ] **Refresh Tokens** - Rotación de tokens JWT
- [ ] **Export CSV Endpoint** - `/api/v1/users/export`
- [ ] **Avatar Upload** - Carga de imágenes de perfil
- [ ] **WebSockets/SSE** - Notificaciones en tiempo real
- [ ] **Cache Redis** - Mejora de performance

### FASE 37: Tests E2E con Playwright

**Prioridad: ALTA** - Completar los tests configurados

- [ ] Login flow (happy path)
- [ ] Login con credenciales inválidas
- [ ] Acceso a dashboard protegido
- [ ] CRUD de usuarios (admin)
- [ ] Visualización de audit logs
- [ ] Logout
- [ ] Responsive testing

### FASE 38: DevOps & Producción

**Prioridad: MEDIA** - Preparar para producción real

- [ ] **Docker Compose** - Dev y prod environments
- [ ] **GitHub Actions** - Deploy automático
- [ ] **Fly.io/Railway** - Hosting gratuito/pago
- [ ] **Sentry** - Error tracking
- [ ] **Backups automáticos** - Base de datos
- [ ] **SSL/HTTPS** - Certificados

### FASE 39: Optimizaciones

**Prioridad: BAJA** - Nice to have

- [ ] **Storybook** - Documentación de componentes
- [ ] **Lighthouse CI** - Performance audit
- [ ] **Lazy loading** - Imágenes y componentes
- [ ] **Service Worker** - Offline support
- [ ] **Image optimization** - Sharp/Cloudinary

---

## 📦 Componentes Dashboard Específicos

### StatsCards Component

```astro
---
// components/dashboard/stats-cards.astro
import { Users, UserCheck, Shield, UserPlus } from 'lucide-astro';
import Card from '../ui/card.astro';

interface Props {
  stats: {
    totalUsers: number;
    activeUsers: number;
    adminUsers: number;
    newUsersToday: number;
  };
}

const { stats } = Astro.props;

const cards = [
  { title: 'Total Usuarios', value: stats.totalUsers, icon: Users, trend: '+12%', color: 'text-blue-600' },
  { title: 'Usuarios Activos', value: stats.activeUsers, icon: UserCheck, trend: '+5%', color: 'text-green-600' },
  { title: 'Administradores', value: stats.adminUsers, icon: Shield, trend: '0%', color: 'text-purple-600' },
  { title: 'Nuevos Hoy', value: stats.newUsersToday, icon: UserPlus, trend: '+8%', color: 'text-orange-600' },
];
---

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
  {cards.map((card) => (
    <Card class="p-6">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-medium text-muted-foreground">{card.title}</p>
          <p class="text-2xl font-bold">{card.value.toLocaleString()}</p>
          <p class="text-xs text-green-600 mt-1">{card.trend} desde ayer</p>
        </div>
        <div class={cn("p-3 rounded-full bg-muted", card.color)}>
          <card.icon class="w-5 h-5" />
        </div>
      </div>
    </Card>
  ))}
</div>
```

### UserTable Component Avanzado

```astro
---
// components/dashboard/user-table.astro
import Table from '../ui/table.astro';
import Badge from '../ui/badge.astro';
import Button from '../ui/button.astro';
import type { User } from '../../types';

interface Props {
  users: User[];
  total: number;
  page: number;
  limit: number;
}

const { users, total, page, limit } = Astro.props;
const totalPages = Math.ceil(total / limit);
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
              <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                <span class="font-medium">{user.username[0].toUpperCase()}</span>
              </div>
              <div>
                <div class="font-medium">{user.username}</div>
                <div class="text-sm text-muted-foreground">{user.email}</div>
              </div>
            </div>
          </Table.Cell>
          <Table.Cell>
            <Badge variant={user.role === 'Admin' ? 'default' : 'secondary'}>
              {user.role}
            </Badge>
          </Table.Cell>
          <Table.Cell>
            <div class="flex items-center gap-2">
              <span class={cn(
                "h-2 w-2 rounded-full",
                user.isActive ? "bg-green-500" : "bg-red-500"
              )} />
              {user.isActive ? 'Activo' : 'Inactivo'}
            </div>
          </Table.Cell>
          <Table.Cell class="text-right">
            <Button variant="ghost" size="sm">Editar</Button>
            <Button variant="ghost" size="sm" class="text-destructive">Eliminar</Button>
          </Table.Cell>
        </Table.Row>
      ))}
    </Table.Body>
  </Table>
  
  <!-- Pagination -->
  <div class="flex items-center justify-between px-4 py-4 border-t">
    <p class="text-sm text-muted-foreground">
      Mostrando {(page - 1) * limit + 1} a {Math.min(page * limit, total)} de {total} usuarios
    </p>
    <div class="flex gap-2">
      <Button 
        variant="outline" 
        size="sm" 
        disabled={page === 1}
        href={`?page=${page - 1}`}
      >
        Anterior
      </Button>
      <Button 
        variant="outline" 
        size="sm" 
        disabled={page === totalPages}
        href={`?page=${page + 1}`}
      >
        Siguiente
      </Button>
    </div>
  </div>
</div>
```

---

## 🎯 Plan de Implementación Sugerido

### Sprint 1: Foundation (3-4 días)
1. ✅ Instalar Tailwind CSS + configurar
2. ✅ Crear globals.css con tema claro/oscuro
3. ✅ Actualizar Layout.astro con estructura base
4. ✅ Crear lib/utils.ts con cn()
5. ✅ Implementar Button, Input, Card básicos

### Sprint 2: Layout Profesional (2-3 días)
1. ✅ Crear Sidebar component
2. ✅ Crear Header component
3. ✅ Crear DashboardLayout
4. ✅ Implementar ThemeToggle
5. ✅ Migrar dashboard.astro al nuevo layout

### Sprint 3: Componentes Core (3-4 días)
1. ✅ Implementar Table con sorting
2. ✅ Implementar Badge, Avatar
3. ✅ Implementar Dialog (modales)
4. ✅ Implementar Toast notifications
5. ✅ Implementar DropdownMenu

### Sprint 4: Estado Global y API (2-3 días)
1. ✅ Configurar nanostores
2. ✅ Crear stores/auth.ts
3. ✅ Crear stores/theme.ts
4. ✅ Crear lib/api.ts tipado
5. ✅ Refactorizar componentes para usar stores

### Sprint 5: Páginas del Dashboard (3-4 días)
1. ✅ Crear /dashboard/index con stats
2. ✅ Crear /dashboard/users con tabla avanzada
3. ✅ Crear /dashboard/audit con timeline
4. ✅ Crear /dashboard/settings
5. ✅ Actualizar login/register con nuevos componentes

### Sprint 6: Polish & Testing (2-3 días)
1. ✅ Responsive design
2. ✅ Loading skeletons
3. ✅ Error boundaries
4. ✅ Implementar tests E2E con Playwright
5. ✅ Performance audit con Lighthouse

**Total estimado:** 2-3 semanas de trabajo

---

## 🚀 Stack Tecnológico Actualizado

### Frontend (Actual + Nuevo)
| Tecnología | Estado | Uso |
|------------|--------|-----|
| Astro 5.x | ✅ Ya tienes | Framework principal |
| Tailwind CSS 3.x | ❌ Instalar | Styling |
| lucide-astro | ❌ Instalar | Iconos |
| nanostores | ❌ Instalar | Estado global |
| shadcn/ui patterns | ❌ Implementar | Componentes base |
| TypeScript | ✅ Ya tienes | Type safety |
| Playwright | ✅ Ya tienes | E2E testing |

### Backend (Ya Completo)
| Tecnología | Estado | Uso |
|------------|--------|-----|
| Rust 1.75+ | ✅ Completo | Lenguaje principal |
| Axum 0.7 | ✅ Completo | Web framework |
| SQLx | ✅ Completo | Database |
| JWT + Cookies | ✅ Completo | Auth |
| Swagger | ✅ Completo | Documentación |

---

## 📊 Métricas de Éxito V4.0

- **UI/UX:** Dashboard profesional con shadcn/ui
- **Performance:** Lighthouse score >90
- **Accesibilidad:** Keyboard navigation + ARIA labels
- **Testing:** Tests E2E pasando (>80%)
- **DX:** `just check` <30s, hot reload funcional
- **Responsive:** Mobile-first design

---

## 🎓 Recursos Rápidos

### shadcn/ui para Astro
```bash
# Ver ejemplos de componentes
# https://ui.shadcn.com/docs/components/accordion
# Adaptar a .astro files en lugar de .tsx
```

### Tailwind + Astro
- [Astro + Tailwind Guide](https://docs.astro.build/en/guides/integrations-guide/tailwind/)

### Nanostores
```typescript
// Ejemplo completo en stores/
// https://github.com/nanostores/nanostores
```

---

## 📋 Checklist de Cierre V4.0

- [ ] Tailwind CSS instalado y configurado
- [ ] Tema claro/oscuro funcionando
- [ ] Layout profesional (sidebar + header)
- [ ] Componentes base implementados (Button, Input, Card, Table, Dialog, Toast)
- [ ] Estado global con nanostores
- [ ] Cliente API tipado
- [ ] Dashboard pages migradas
- [ ] Responsive design
- [ ] Tests E2E implementados
- [ ] Lighthouse score >90

---

**Nota:** Este roadmap está enfocado principalmente en **mejorar el Frontend** porque el Backend ya está Enterprise-ready. Las mejoras del backend son opcionales (Fase 36+).

**Versión:** V4.0 Roadmap Actualizado  
**Última actualización:** 15 Feb 2026  
**Estado:** Fases 0-34 Completadas ✅  
**Próximo foco:** Fase 35 (UI Stack) 🎨
