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
    return this.request<{ user: User; token: string }>('/api/v1/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    })
  }

  async logout() {
    await this.request<void>('/api/v1/logout', { method: 'POST' })
    // Limpiar cookie de sesión (Frontend/SSR) y redirigir
    document.cookie = "session=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";
    window.location.href = '/login';
  }

  async getDashboard() {
    return this.request<{ message: string; user: User }>('/api/v1/dashboard')
  }

  // Users
  async getUsers(params?: UserSearch, token?: string) {
    const query = new URLSearchParams()
    if (params?.search) query.set('q', params.search) // Fix: Backend espera 'q', no 'search'
    if (params?.page) query.set('page', params.page.toString())
    if (params?.limit) query.set('limit', params.limit.toString())
    
    const queryString = query.toString()
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }

    const response = await this.request<UserDetails[] | PaginatedResponse<UserDetails>>(
      `/api/v1/users${queryString ? `?${queryString}` : ''}`,
      { headers }
    )

    // Normalizar respuesta si el backend devuelve array plano
    if (Array.isArray(response)) {
        return {
            data: response,
            meta: {
                total: response.length, // Temporal: backend no devuelve total real aún
                page: params?.page || 1,
                limit: params?.limit || 10,
                totalPages: 1 // Temporal
            }
        } as any; // Cast for compatibility
    }

    return response as PaginatedResponse<UserDetails>;
  }

  async getUser(id: number, token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<User>(`/api/v1/users/${id}`, { headers })
  }

  async createUser(data: CreateUserRequest) {
    return this.request<User>('/api/v1/users', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  }

  async deleteUser(id: number, token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<void>(`/api/v1/users/${id}`, {
      method: 'DELETE',
      headers,
    })
  }

  async updateUser(id: number, data: { email?: string }, token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<User>(`/api/v1/users/${id}/profile`, {
      method: 'PUT',
      headers,
      body: JSON.stringify(data)
    })
  }

  // Audit
  async getAuditLogs(params?: { page?: number; limit?: number }, token?: string) {
    const query = new URLSearchParams()
    if (params?.page) query.set('page', params.page.toString())
    if (params?.limit) query.set('limit', params.limit.toString())
    
    const queryString = query.toString()
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    const response = await this.request<AuditLog[] | PaginatedResponse<AuditLog>>(
      `/api/v1/audit-logs${queryString ? `?${queryString}` : ''}`,
      { headers }
    )

    if (Array.isArray(response)) {
        return {
            data: response,
            meta: {
                total: response.length,
                page: params?.page || 1,
                limit: params?.limit || 10,
                totalPages: 1
            }
        } as any;
    }
    return response as PaginatedResponse<AuditLog>;
  }

  // Stats (endpoint que necesitamos crear en backend)
  async getStats(token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<StatsData>('/api/v1/stats', { headers })
  }

  // Export CSV
  async exportUsers() {
    const url = `${API_BASE_URL}/api/v1/users/export`
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include',
    })
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw new ApiError(
        errorData.message || `Error ${response.status}`,
        response.status
      )
    }
    
    // Descargar archivo
    const blob = await response.blob()
    const downloadUrl = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = downloadUrl
    a.download = 'users_export.csv'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    window.URL.revokeObjectURL(downloadUrl)
  }

  async exportAuditLogs() {
    const url = `${API_BASE_URL}/api/v1/audit-logs/export`
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include',
    })
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw new ApiError(
        errorData.message || `Error ${response.status}`,
        response.status
      )
    }
    
    // Descargar archivo
    const blob = await response.blob()
    const downloadUrl = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = downloadUrl
    a.download = 'audit_logs_export.csv'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    window.URL.revokeObjectURL(downloadUrl)
  }

  // Avatar Upload
  async uploadAvatar(file: File): Promise<User> {
    const url = `${API_BASE_URL}/api/v1/users/avatar`
    const formData = new FormData()
    formData.append('avatar', file)
    
    const response = await fetch(url, {
      method: 'POST',
      credentials: 'include',
      body: formData,
      // No agregar Content-Type, el browser lo agrega automáticamente con boundary
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
}

export const api = new ApiClient()
