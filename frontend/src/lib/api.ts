import type { 
  User, 
  UserDetails, 
  AuditLog, 
  CreateUserRequest, 
  LoginRequest,
  UserSearch,
  PaginatedResponse,
  StatsData,
  Role,
  Permission
} from '../types'

export const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000'

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

  private isClient(): boolean {
    return typeof window !== 'undefined' && typeof localStorage !== 'undefined'
  }

  private getRefreshToken(): string | null {
    if (!this.isClient()) return null
    return localStorage.getItem('refresh_token')
  }

  private setRefreshToken(token: string | null) {
    if (!this.isClient()) return
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
    if (this.refreshPromise) {
      return this.refreshPromise
    }

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
    
    if (response.status === 401) {
      try {
        await this.refreshAccessToken()
        response = await makeRequest()
      } catch (refreshError) {
        if (this.isClient()) {
          window.location.href = '/login/'
        }
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

  async login(credentials: LoginRequest): Promise<TokenResponse> {
    const response = await this.request<TokenResponse>('/api/v1/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    })
    
    this.setRefreshToken(response.refresh_token)
    
    return response
  }

  async logout() {
    await this.request<void>('/api/v1/logout', { method: 'POST' })
    this.setRefreshToken(null)
    if (this.isClient()) {
      document.cookie = "session=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";
      window.location.href = '/login';
    }
  }

  async logoutAll() {
    await this.request<void>('/api/v1/logout-all', { method: 'POST' })
    this.setRefreshToken(null)
    if (this.isClient()) {
      document.cookie = "session=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";
      window.location.href = '/login';
    }
  }

  async getDashboard() {
    return this.request<{ message: string; user: User }>('/api/v1/dashboard')
  }

  async getUsers(params?: UserSearch, token?: string) {
    const query = new URLSearchParams()
    if (params?.search) query.set('q', params.search)
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

  async updateUser(id: number, data: { email?: string; role?: string }, token?: string) {
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

  async changePassword(data: { current_password: string; new_password: string }, token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<{ message: string }>('/api/v1/users/password', {
      method: 'PUT',
      headers,
      body: JSON.stringify(data)
    })
  }

  async getRoles(token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<Role[]>('/api/v1/roles', { headers })
  }

  async getRolePermissions(token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<any[]>('/api/v1/roles/permissions', { headers })
  }

  async createRole(data: { name: string; description?: string; permissions: number[] }) {
    return this.request<Role>('/api/v1/roles', {
      method: 'POST',
      body: JSON.stringify(data)
    })
  }

  async updateRole(id: number, data: { name?: string; description?: string; permissions?: number[] }) {
    return this.request<Role>(`/api/v1/roles/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    })
  }

  async deleteRole(id: number) {
    return this.request<void>(`/api/v1/roles/${id}`, {
      method: 'DELETE'
    })
  }

  async getPermissions(token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<Permission[]>('/api/v1/permissions', { headers })
  }

  async updatePermission(id: number, description: string) {
    return this.request<Permission>(`/api/v1/permissions/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ description })
    })
  }

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

  async getStats(token?: string) {
    const headers: Record<string, string> = {};
    if (token) {
        headers['Cookie'] = `auth_token=${token}`;
    }
    return this.request<StatsData>('/api/v1/stats', { headers })
  }

  async exportUsers() {
    if (!this.isClient()) throw new ApiError('Export only available in browser', 400)
    
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
    if (!this.isClient()) throw new ApiError('Export only available in browser', 400)
    
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

  async uploadAvatar(file: File): Promise<User> {
    if (!this.isClient()) throw new ApiError('Upload only available in browser', 400)
    
    const url = `${API_BASE_URL}/api/v1/users/avatar`
    const formData = new FormData()
    formData.append('avatar', file)
    
    const response = await fetch(url, {
      method: 'POST',
      credentials: 'include',
      body: formData,
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

  async forgotPassword(email: string): Promise<{ message: string }> {
    return this.request<{ message: string }>('/api/v1/forgot-password', {
      method: 'POST',
      body: JSON.stringify({ email }),
    })
  }

  async resetPassword(token: string, newPassword: string): Promise<{ message: string }> {
    return this.request<{ message: string }>('/api/v1/reset-password', {
      method: 'POST',
      body: JSON.stringify({ token, new_password: newPassword }),
    })
  }

  async sendVerificationEmail(): Promise<{ message: string }> {
    return this.request<{ message: string }>('/api/v1/send-verification-email', {
      method: 'POST',
    })
  }

  async verifyEmail(token: string): Promise<{ message: string }> {
    return this.request<{ message: string }>(`/api/v1/verify-email?token=${token}`, {
      method: 'GET',
    })
  }
}

export const api = new ApiClient()
