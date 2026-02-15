# 🚀 Roadmap V4.0: Sintonía 2026 Dashboard Edition

> **Fecha de Planificación:** 15 Feb 2026  
> **Base:** V3.0 Enterprise Completada  
> **Objetivo:** Transformar el boilerplate en un sistema de dashboard completo y reutilizable

---

## 📊 Análisis del Estado Actual

### ✅ Lo que tienes implementado (V3.0 Enterprise)

**Backend (Rust + Axum):**
- ✅ Arquitectura Hexagonal (core/data/api)
- ✅ Autenticación JWT con cookies HttpOnly
- ✅ Autorización RBAC (Admin/User)
- ✅ Logs de auditoría
- ✅ Repository Pattern (UserRepository trait)
- ✅ Errores tipados (AppError)
- ✅ Configuración jerárquica
- ✅ Observabilidad (tracing JSON)
- ✅ Rate limiting (Governor)
- ✅ API versionada (/api/v1)
- ✅ Swagger/OpenAPI documentation
- ✅ CORS configurado
- ✅ SQLx con SQLite

**Frontend (Astro):**
- ✅ Estructura base de Astro
- ✅ Componentes básicos (LoginForm, UserList, AuditTable, etc.)
- ✅ Páginas: index, login, register, dashboard
- ✅ Integración con backend

**DX (Developer Experience):**
- ✅ Justfile con comandos unificados
- ✅ Pre-commit hooks listos para configurar
- ✅ Tests de integración

### ⚠️ Lo que le falta para ser un Dashboard Enterprise completo

**Frontend - UI/UX:**
- ❌ No tiene Tailwind CSS configurado
- ❌ No tiene sistema de diseño (componentes reutilizables)
- ❌ No tiene sistema de iconos
- ❌ No tiene manejo de estado global
- ❌ No tiene formularios con validación robusta
- ❌ No tiene tablas avanzadas (sorting, filtering, pagination)
- ❌ No tiene notificaciones/toasts
- ❌ No tiene modales avanzados
- ❌ No tiene layout de dashboard (sidebar, header, content)
- ❌ No tiene tema claro/oscuro
- ❌ No tiene loading states y skeletons
- ❌ No tiene manejo de errores en UI

**Backend - Features:**
- ❌ No tiene refresh tokens
- ❌ No tiene paginación en endpoints
- ❌ No tiene búsqueda/filtros avanzados en API
- ❌ No tiene exportación de datos (CSV/Excel)
- ❌ No tiene carga de archivos
- ❌ No tiene notificaciones en tiempo real (WebSockets/SSE)
- ❌ No tiene jobs en background
- ❌ No tiene caché (Redis)
- ❌ No tiene health checks avanzados
- ❌ No tiene métricas (Prometheus)

**DevOps/Infra:**
- ❌ No tiene Docker/Docker Compose
- ❌ No tiene CI/CD pipeline
- ❌ No tiene tests E2E configurados
- ❌ No tiene despliegue automatizado

---

## 🎨 FASE 35: El Nuevo Stack de UI (Sintonía 2026)

### 1. shadcn/ui para Astro 🎯

**¿Por qué shadcn/ui?**
- **Ownership total:** Tú eres el dueño del código, no es una dependencia black-box
- **Minimalista e industrial:** Alineado con tu estética actual
- **Actualizado semanalmente:** Mantenimiento constante
- **Tailwind-first:** Integración perfecta con Tailwind CSS
- **Accesible:** Componentes con a11y incluido
- **Type-safe:** TypeScript first

**Implementación para Astro:**

```bash
# 1. Configurar Tailwind CSS en el frontend
cd frontend
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 2. Instalar dependencias de shadcn/ui
npm install -D @tailwindcss/typography class-variance-authority clsx tailwind-merge
npm install lucide-react  # Iconos

# 3. Configurar colores (tema Sintonía 2026)
# Slate/Zinc para estética industrial minimalista
```

### 2. Estructura del Sistema de Diseño

```
frontend/src/
├── components/
│   ├── ui/                    # Componentes base shadcn/ui
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
│   │   ├── select.astro
│   │   ├── checkbox.astro
│   │   └── ...
│   ├── layout/                # Layout components
│   │   ├── dashboard-shell.astro
│   │   ├── sidebar.astro
│   │   ├── header.astro
│   │   ├── breadcrumbs.astro
│   │   └── footer.astro
│   └── dashboard/             # Componentes específicos del dashboard
│       ├── stats-cards.astro
│       ├── recent-activity.astro
│       ├── user-management.astro
│       └── audit-logs.astro
├── lib/
│   ├── utils.ts               # Utilidades (cn, formatters)
│   ├── api.ts                 # Cliente API con fetch interceptors
│   ├── auth.ts                # Helpers de autenticación
│   └── constants.ts           # Constantes globales
├── stores/
│   └── auth.ts                # Estado global (nanostores)
├── hooks/
│   ├── use-auth.ts
│   ├── use-fetch.ts
│   └── use-toast.ts
├── styles/
│   └── globals.css            # Variables CSS, tema claro/oscuro
└── types/
    └── index.ts               # Tipos TypeScript compartidos
```

### 3. Componentes Base Requeridos

#### Core UI Components

| Componente | Props | Descripción |
|------------|-------|-------------|
| `Button` | variant, size, loading, disabled | Botón con estados |
| `Input` | type, placeholder, error, icon | Input con validación |
| `Card` | title, description, footer, class | Contenedor flexible |
| `Dialog` | open, onOpenChange, title | Modal accesible |
| `Table` | data, columns, sorting, pagination | Tabla avanzada |
| `Tabs` | value, onValueChange, items | Navegación por tabs |
| `Toast` | type, message, duration | Notificaciones |
| `Badge` | variant, children | Etiquetas de estado |
| `Avatar` | src, fallback, size | Imagen de perfil |
| `Skeleton` | class | Loading placeholder |
| `Select` | options, value, onChange | Dropdown |
| `DropdownMenu` | trigger, items | Menú contextual |
| `Tooltip` | content, children | Ayuda contextual |
| `Switch` | checked, onChange | Toggle |
| `Calendar` | value, onChange | Selector de fecha |
| `Command` | placeholder, items | Búsqueda rápida (⌘K) |

#### Layout Components

```astro
---
// dashboard-shell.astro - Layout principal
interface Props {
  title: string;
  description?: string;
}
---

<div class="flex h-screen bg-background">
  <Sidebar />
  <div class="flex-1 flex flex-col overflow-hidden">
    <Header />
    <main class="flex-1 overflow-auto p-6">
      <slot />
    </main>
  </div>
</div>
```

### 4. Tema Sintonía 2026

#### Paleta de Colores (CSS Variables)

```css
:root {
  /* Industrial Minimalist Theme */
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
```

#### Tipografía

- **Primary:** Inter o Geist (moderna, legible)
- **Mono:** JetBrains Mono o Fira Code (para código/logs)
- **Tamaños:** Escala 1.25 (minor third)

### 5. Arquitectura del Dashboard

#### Layout Principal

```
┌─────────────────────────────────────────────────────┐
│  Logo    Search                    User   Bell   ⚙️  │ Header
├─────────────────────────────────────────────────────┤
│        │                                            │
│   Nav  │          Content Area                      │
│        │                                            │
│  Home  │  ┌──────────────────────────────────┐     │
│  Users │  │  Breadcrumbs > Page Title          │     │
│  Audit │  ├──────────────────────────────────┤     │
│        │  │                                    │     │
│        │  │   Cards / Stats / Charts           │     │
│        │  │                                    │     │
│        │  │   ┌─────┐ ┌─────┐ ┌─────┐        │     │
│        │  │   │Card1│ │Card2│ │Card3│        │     │
│        │  │   └─────┘ └─────┘ └─────┘        │     │
│        │  │                                    │     │
│        │  │   Data Table                       │     │
│        │  │   ┌────────────────────────────┐   │     │
│        │  │   │ ...                        │   │     │
│        │  │   └────────────────────────────┘   │     │
│        │  │                                    │     │
└────────┴──────────────────────────────────────────┘
```

#### Páginas del Dashboard

1. **Overview/Dashboard** (`/dashboard`)
   - Stats cards (usuarios totales, activos, admins)
   - Gráfico de actividad reciente
   - Tabla de últimas acciones
   - Alertas/notificaciones

2. **User Management** (`/dashboard/users`)
   - Tabla con filtros, sorting, pagination
   - Acciones: View, Edit, Delete, Activate
   - Búsqueda en tiempo real
   - Exportar a CSV

3. **Audit Logs** (`/dashboard/audit`)
   - Timeline de eventos
   - Filtros por fecha, usuario, acción
   - Exportar logs

4. **Settings** (`/dashboard/settings`)
   - Perfil de usuario
   - Preferencias (tema, notificaciones)
   - Configuración de cuenta

5. **System** (`/dashboard/system`) - Solo Admin
   - Estado del sistema
   - Métricas de performance
   - Configuración avanzada

### 6. Cliente API Avanzado

```typescript
// lib/api.ts
import type { User, AuditLog } from '../types';

class ApiClient {
  private baseUrl: string;
  
  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }
  
  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      credentials: 'include', // Para cookies HttpOnly
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new ApiError(error.message, response.status);
    }
    
    return response.json();
  }
  
  // Users
  async getUsers(params?: { search?: string; page?: number; limit?: number }) {
    const query = new URLSearchParams(params as Record<string, string>);
    return this.request<User[]>(`/api/v1/users?${query}`);
  }
  
  async createUser(data: CreateUserInput) {
    return this.request<User>('/api/v1/users', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }
  
  async deleteUser(id: string) {
    return this.request<void>(`/api/v1/users/${id}`, {
      method: 'DELETE',
    });
  }
  
  // Auth
  async login(credentials: LoginInput) {
    return this.request<{ user: User }>('/api/v1/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });
  }
  
  async logout() {
    return this.request<void>('/api/v1/logout', { method: 'POST' });
  }
  
  // Audit
  async getAuditLogs(params?: AuditLogParams) {
    const query = new URLSearchParams(params as Record<string, string>);
    return this.request<AuditLog[]>(`/api/v1/audit-logs?${query}`);
  }
}

export const api = new ApiClient(import.meta.env.PUBLIC_API_URL || 'http://localhost:3000');
```

### 7. Manejo de Estado (Nanostores)

```typescript
// stores/auth.ts
import { atom, computed } from 'nanostores';
import type { User } from '../types';

// State
export const $user = atom<User | null>(null);
export const $isLoading = atom(false);
export const $error = atom<string | null>(null);

// Computed
export const $isAuthenticated = computed($user, (user) => user !== null);
export const $isAdmin = computed($user, (user) => user?.role === 'Admin');

// Actions
export async function login(credentials: { username: string; password: string }) {
  $isLoading.set(true);
  $error.set(null);
  
  try {
    const response = await fetch('/api/v1/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(credentials),
      credentials: 'include',
    });
    
    if (!response.ok) throw new Error('Login failed');
    
    const data = await response.json();
    $user.set(data.user);
  } catch (err) {
    $error.set(err instanceof Error ? err.message : 'Unknown error');
  } finally {
    $isLoading.set(false);
  }
}

export function logout() {
  $user.set(null);
  fetch('/api/v1/logout', { method: 'POST', credentials: 'include' });
}
```

---

## 🏗️ FASES RESTANTES DEL ROADMAP

### FASE 36: Backend Features Avanzados

#### 36.1 Refresh Tokens
```rust
// Implementar rotating refresh tokens
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
```

#### 36.2 Paginación y Filtros
```rust
// En todos los endpoints de listado
pub struct PaginationParams {
    pub page: i64,
    pub limit: i64,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
```

#### 36.3 Búsqueda Full-Text
```rust
// Implementar con SQLite FTS5 o migrar a PostgreSQL
pub async fn search_users(
    &self,
    query: &str,
    pagination: PaginationParams,
) -> Result<PaginatedResponse<User>, AppError>;
```

#### 36.4 Exportación de Datos
```rust
// Exportar a CSV/Excel
pub async fn export_users_csv(&self, filters: UserFilters) -> Result<Vec<u8>, AppError>;
```

#### 36.5 Carga de Archivos
```rust
// Avatar uploads, documentos, etc.
pub async fn upload_file(
    &self,
    user_id: i64,
    file: Multipart,
) -> Result<FileUpload, AppError>;
```

#### 36.6 WebSockets / SSE
```rust
// Notificaciones en tiempo real
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse;
```

### FASE 37: Testing y Calidad

- [ ] Tests unitarios (backend >80% coverage)
- [ ] Tests de integración (Playwright)
- [ ] Tests E2E (login flow, CRUD operations)
- [ ] Lighthouse CI (performance audit)
- [ ] Accessibility audit (WCAG 2.1)
- [ ] Load testing (k6)

### FASE 38: DevOps y Deployment

- [ ] Docker multi-stage build
- [ ] Docker Compose (dev/prod)
- [ ] GitHub Actions CI/CD
- [ ] Fly.io / Railway / Render deployment
- [ ] Database backups automatizados
- [ ] Monitoring (Sentry, LogRocket)

### FASE 39: Documentación

- [ ] Storybook para componentes UI
- [ ] Guía de contribución
- [ ] Documentación API (mejorar Swagger)
- [ ] Videos tutoriales (opcional)

### FASE 40: Optimizaciones

- [ ] Lazy loading de imágenes
- [ ] Code splitting en Astro
- [ ] Caché HTTP headers
- [ ] Service Worker para offline
- [ ] Image optimization (Sharp)

---

## 📦 Componentes Dashboard Específicos

### UserTable Component (Avanzado)

```astro
---
// components/dashboard/user-table.astro
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
    <TableHeader>
      <TableRow>
        <TableHead class="w-[100px]">ID</TableHead>
        <TableHead>Usuario</TableHead>
        <TableHead>Rol</TableHead>
        <TableHead>Estado</TableHead>
        <TableHead class="text-right">Acciones</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      {users.map((user) => (
        <TableRow key={user.id}>
          <TableCell class="font-medium">{user.id}</TableCell>
          <TableCell>
            <div class="flex items-center gap-2">
              <Avatar src={user.avatar} fallback={user.username[0]} />
              <div>
                <div class="font-medium">{user.username}</div>
                <div class="text-sm text-muted-foreground">{user.email}</div>
              </div>
            </div>
          </TableCell>
          <TableCell>
            <Badge variant={user.role === 'Admin' ? 'default' : 'secondary'}>
              {user.role}
            </Badge>
          </TableCell>
          <TableCell>
            <div class="flex items-center gap-2">
              <div class={`h-2 w-2 rounded-full ${user.isActive ? 'bg-green-500' : 'bg-red-500'}`} />
              {user.isActive ? 'Activo' : 'Inactivo'}
            </div>
          </TableCell>
          <TableCell class="text-right">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" size="sm">
                  <MoreHorizontal className="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem>Editar</DropdownMenuItem>
                <DropdownMenuItem>Ver detalles</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem class="text-red-600">
                  Eliminar
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </TableCell>
        </TableRow>
      ))}
    </TableBody>
  </Table>
  
  <div class="flex items-center justify-between px-4 py-4">
    <div class="text-sm text-muted-foreground">
      Mostrando {(page - 1) * limit + 1} a {Math.min(page * limit, total)} de {total} usuarios
    </div>
    <Pagination>
      <PaginationContent>
        <PaginationItem>
          <PaginationPrevious href={`?page=${page - 1}`} disabled={page === 1} />
        </PaginationItem>
        {Array.from({ length: totalPages }, (_, i) => (
          <PaginationItem key={i}>
            <PaginationLink href={`?page=${i + 1}`} isActive={page === i + 1}>
              {i + 1}
            </PaginationLink>
          </PaginationItem>
        ))}
        <PaginationItem>
          <PaginationNext href={`?page=${page + 1}`} disabled={page === totalPages} />
        </PaginationItem>
      </PaginationContent>
    </Pagination>
  </div>
</div>
```

### StatsCards Component

```astro
---
// components/dashboard/stats-cards.astro
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
  { title: 'Total Usuarios', value: stats.totalUsers, icon: Users, trend: '+12%' },
  { title: 'Usuarios Activos', value: stats.activeUsers, icon: UserCheck, trend: '+5%' },
  { title: 'Administradores', value: stats.adminUsers, icon: Shield, trend: '0%' },
  { title: 'Nuevos Hoy', value: stats.newUsersToday, icon: UserPlus, trend: '+8%' },
];
---

<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
  {cards.map((card) => (
    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">{card.title}</CardTitle>
        <card.icon className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{card.value.toLocaleString()}</div>
        <p class="text-xs text-muted-foreground">
          <span class={`${card.trend.startsWith('+') ? 'text-green-600' : 'text-red-600'}`}>
            {card.trend}
          </span>{' '}
          desde el mes pasado
        </p>
      </CardContent>
    </Card>
  ))}
</div>
```

---

## 🎯 Plan de Implementación Sugerido

### Sprint 1: Foundation (1 semana)
1. Configurar Tailwind CSS
2. Configurar sistema de colores y variables CSS
3. Crear utilidades base (cn, formatters)
4. Implementar layout principal (sidebar, header)
5. Configurar tema claro/oscuro

### Sprint 2: Core Components (1 semana)
1. Implementar Button, Input, Card
2. Implementar Table con sorting/pagination
3. Implementar Dialog, Dropdown
4. Implementar Toast notifications
5. Implementar Badge, Avatar, Skeleton

### Sprint 3: Dashboard Pages (1 semana)
1. Crear página Overview con stats
2. Refactorizar User Management con nueva tabla
3. Refactorizar Audit Logs con timeline
4. Crear página Settings
5. Implementar búsqueda y filtros

### Sprint 4: Backend Enhancements (1 semana)
1. Agregar paginación a endpoints
2. Implementar refresh tokens
3. Agregar filtros de búsqueda
4. Implementar exportación CSV
5. Mejorar manejo de errores

### Sprint 5: Polish & Testing (1 semana)
1. Responsive design
2. Loading states
3. Error boundaries
4. Tests E2E
5. Performance audit

---

## 🚀 Stack Tecnológico Completo

### Frontend
| Tecnología | Versión | Uso |
|------------|---------|-----|
| Astro | ^5.x | Framework principal |
| Tailwind CSS | ^3.x | Styling |
| shadcn/ui | - | Componentes base |
| Lucide Icons | ^0.x | Iconos |
| Nanostores | ^0.x | State management |
| TypeScript | ^5.x | Type safety |
| Playwright | ^1.x | E2E testing |

### Backend
| Tecnología | Versión | Uso |
|------------|---------|-----|
| Rust | 1.75+ | Lenguaje principal |
| Axum | ^0.7 | Web framework |
| SQLx | ^0.7 | Database |
| Tokio | ^1.x | Async runtime |
| Serde | ^1.x | Serialization |
| Validator | ^0.16 | Validation |
| Utoipa | ^4.x | OpenAPI |

### DevOps
| Tecnología | Uso |
|------------|-----|
| Docker | Containerización |
| GitHub Actions | CI/CD |
| Sentry | Error tracking |
| Fly.io/Railway | Hosting |

---

## 📊 Métricas de Éxito

- **Performance:** Lighthouse score >90
- **Accesibilidad:** WCAG 2.1 AA compliance
- **Coverage:** >80% backend, >60% frontend
- **UX:** <3s time to interactive
- **DX:** `just check` <30s

---

## 🎓 Recursos de Aprendizaje

### shadcn/ui
- [Documentación oficial](https://ui.shadcn.com)
- [Repositorio GitHub](https://github.com/shadcn-ui/ui)

### Astro
- [Astro Docs](https://docs.astro.build)
- [Astro Islands Architecture](https://docs.astro.build/en/concepts/islands/)

### Tailwind CSS
- [Tailwind Docs](https://tailwindcss.com/docs)
- [Tailwind UI](https://tailwindui.com) (referencia)

### Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

---

**Nota:** Este roadmap está diseñado para ser implementado incrementalmente. Cada fase puede ser completada independientemente y el sistema seguirá funcionando.

**Versión:** V4.0 Roadmap  
**Última actualización:** 15 Feb 2026  
**Autor:** Sintonía 3026
