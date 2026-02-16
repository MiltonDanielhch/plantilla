// Tipos compartidos del backend

export interface User {
  id: number
  username: string
  role: 'Admin' | 'User'
}

export interface UserDetails extends User {
  email?: string
  created_at?: string
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
