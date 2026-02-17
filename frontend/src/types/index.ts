// Tipos compartidos del backend

export interface User {
  id: number
  username: string
  email?: string
  role: 'Admin' | 'User'
  created_at?: string
  avatar_url?: string
  email_verified?: boolean
}

export interface UserDetails extends User {
  // Hereda todas las propiedades de User
}

export interface Role {
  id: number
  name: string
  description?: string
  permissions?: Permission[]
}

export interface Permission {
  id: number
  name: string
  description?: string
}

export interface AuditLog {
  id: number
  action: string
  // Backend fields
  admin_username?: string
  target?: string
  timestamp?: string
  // Frontend mappings (legacy support)
  user_id?: number
  target_id?: number
  details?: string
  created_at?: string
  username?: string
}

export interface CreateUserRequest {
  username: string
  email?: string
  password: string
  role?: 'Admin' | 'User'
}

export interface LoginRequest {
  username: string
  password: string
}

export interface UserSearch {
  search?: string
  page?: number
  limit?: number
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  limit: number
  total_pages: number
}

export interface DashboardData {
  message: string
  user: User
}

export interface StatsData {
  total_users: number
  active_users: number
  admin_users: number
  new_users_today: number
}
