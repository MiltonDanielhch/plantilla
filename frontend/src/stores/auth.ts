import { atom, computed } from 'nanostores'
import type { User } from '../types'

// State
export const $user = atom<User | null>(null)
export const $isLoading = atom<boolean>(false)
export const $error = atom<string | null>(null)

// Computed
export const $isAuthenticated = computed($user, (user) => user !== null)
export const $isAdmin = computed($user, (user) => user?.role === 'Admin')

// Actions
export async function checkAuth(): Promise<boolean> {
  $isLoading.set(true)
  $error.set(null)
  
  try {
    const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000'
    const response = await fetch(`${API_BASE_URL}/api/v1/dashboard`, {
      method: 'GET',
      credentials: 'include',
    })

    if (response.ok) {
      const data = await response.json()
      $user.set(data.user)
      return true
    } else {
      $user.set(null)
      return false
    }
  } catch (err) {
    console.error('Auth check error:', err)
    $user.set(null)
    return false
  } finally {
    $isLoading.set(false)
  }
}

export async function login(credentials: { username: string; password: string }): Promise<boolean> {
  $isLoading.set(true)
  $error.set(null)
  
  try {
    const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000'
    const response = await fetch(`${API_BASE_URL}/api/v1/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(credentials),
      credentials: 'include',
    })
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      $error.set(errorData.message || 'Error al iniciar sesión')
      return false
    }
    
    const data = await response.json()
    $user.set(data.user)
    return true
  } catch (err) {
    console.error('Login error:', err)
    $error.set('Error de conexión con el servidor')
    return false
  } finally {
    $isLoading.set(false)
  }
}

export async function logout(): Promise<void> {
  try {
    const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000'
    await fetch(`${API_BASE_URL}/api/v1/logout`, { 
      method: 'POST', 
      credentials: 'include' 
    })
  } catch (err) {
    console.error('Logout error:', err)
  } finally {
    $user.set(null)
  }
}

export function clearError(): void {
  $error.set(null)
}
