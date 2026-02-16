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

interface TokenResponse {
  user: User
  access_token: string
  refresh_token: string
  expires_in: number
  token_type: string
}

class ApiClient {
  private refreshPromise: Promise<void> | null = null

  private getRefreshToken(): string | null {
    return localStorage.getItem('refresh_token')
  }

  private setRefreshToken(token: string | null) {
    if (token) {
      localStorage.setItem('refresh_token', token)
    } else {
      localStorage.removeItem('refresh_token')
    }
  }

  private async doRefresh(): Promise<void> {
    const refreshToken = this.getRefreshToken()
    if (!refreshToken) {
      throw new ApiError('No refresh token available', 401)
    }

    try {
      const response = await fetch(`${API_BASE_URL}/api/v1/refresh`, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: refreshToken }),
      })

      if (!response.ok) {
        // Refresh failed, clear tokens
        this.setRefreshToken(null)
        throw new ApiError('Session expired', 401)
      }

      const data = await response.json()
      this.setRefreshToken(data.refresh_token)
    } catch (error) {
      this.setRefreshToken(null)
      throw error
    }
  }

  private async refreshAccessToken(): Promise<void> {
    // If already refreshing, wait for that promise
    if (this.refreshPromise) {
      return this.refreshPromise
    }

    // Create new refresh promise
    this.refreshPromise = this.doRefresh().finally(() => {
      this.refreshPromise = null
    })

    return this.refreshPromise
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`
    
    const makeRequest = async (): Promise<Response> => {
      return fetch(url, {
        ...options,
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
          ...options?.headers,
        },
      })
    }

    let response = await makeRequest()
    
    // If unauthorized, try to refresh token
    if (response.status === 401) {
      try {
        await this.refreshAccessToken()
        // Retry request with new token
        response = await makeRequest()
      } catch (refreshError) {
        // Refresh failed, redirect to login
        window.location.href = '/login/'
        throw new ApiError('Session expired', 401)
      }
    }
    
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
  async login(credentials: LoginRequest): Promise<TokenResponse> {
    const response = await this.request<TokenResponse>('/api/v1/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    })
    
    // Guardar refresh token
    this.setRefreshToken(response.refresh_token)
    
    return response
  }

  async logout() {
    await this.request<void>('/api/v1/logout', { method: 'POST' })
    // Limpiar refresh token y cookies
    this.setRefreshToken(null)
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
