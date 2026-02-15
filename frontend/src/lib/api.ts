import type { 
  User, 
  UserDetails, 
  AuditLog, 
  CreateUserRequest, 
  LoginRequest,
  UserSearch,
  PaginatedResponse,
  StatsData 
} from '../types'

const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000'

class ApiError extends Error {
  constructor(public message: string, public status: number) {
    super(message)
    this.name = 'ApiError'
  }
}

class ApiClient {
  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`
    
    const response = await fetch(url, {
      ...options,
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    })
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw new ApiError(
        errorData.message || `Error ${response.status}`,
        response.status
      )
    }
    
    return response.json()
  }

  // Auth
  async login(credentials: LoginRequest) {
    return this.request<{ user: User }>('/api/v1/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    })
  }

  async logout() {
    return this.request<void>('/api/v1/logout', { method: 'POST' })
  }

  async getDashboard() {
    return this.request<{ message: string; user: User }>('/api/v1/dashboard')
  }

  // Users
  async getUsers(params?: UserSearch) {
    const query = new URLSearchParams()
    if (params?.search) query.set('search', params.search)
    if (params?.page) query.set('page', params.page.toString())
    if (params?.limit) query.set('limit', params.limit.toString())
    
    const queryString = query.toString()
    return this.request<PaginatedResponse<UserDetails>>(
      `/api/v1/users${queryString ? `?${queryString}` : ''}`
    )
  }

  async createUser(data: CreateUserRequest) {
    return this.request<User>('/api/v1/users', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  }

  async deleteUser(id: number) {
    return this.request<void>(`/api/v1/users/${id}`, {
      method: 'DELETE',
    })
  }

  // Audit
  async getAuditLogs(params?: { page?: number; limit?: number }) {
    const query = new URLSearchParams()
    if (params?.page) query.set('page', params.page.toString())
    if (params?.limit) query.set('limit', params.limit.toString())
    
    const queryString = query.toString()
    return this.request<PaginatedResponse<AuditLog>>(
      `/api/v1/audit-logs${queryString ? `?${queryString}` : ''}`
    )
  }

  // Stats (endpoint que necesitamos crear en backend)
  async getStats() {
    // Temporalmente retornamos datos vacíos hasta crear el endpoint
    return {
      total_users: 0,
      active_users: 0,
      admin_users: 0,
      new_users_today: 0,
    } as StatsData
  }
}

export const api = new ApiClient()
