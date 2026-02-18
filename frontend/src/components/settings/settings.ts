import { api, API_BASE_URL } from '../../lib/api'
import type { User } from '../../types'

export function loadUserData(user: User) {
  const avatarImg = document.getElementById('profile-avatar-img') as HTMLImageElement
  const avatarFallback = document.getElementById('profile-avatar-fallback')
  const removeAvatarBtn = document.getElementById('remove-avatar-btn')
  const usernameInput = document.getElementById('username-input') as HTMLInputElement
  const emailInput = document.getElementById('email-input') as HTMLInputElement
  const roleInput = document.getElementById('role-input') as HTMLInputElement
  const idInput = document.getElementById('id-input') as HTMLInputElement
  const emailStatus = document.getElementById('email-verification-status')
  const verificationActions = document.getElementById('verification-actions')
  
  const avatarUrl = (user as any).avatar_url
  if (avatarUrl && avatarImg && avatarFallback) {
    avatarImg.src = avatarUrl.startsWith('http') ? avatarUrl : `${API_BASE_URL}${avatarUrl}`
    avatarImg.classList.remove('hidden')
    avatarFallback.classList.add('hidden')
    removeAvatarBtn?.classList.remove('hidden')
  } else if (avatarFallback) {
    avatarFallback.textContent = user.username.charAt(0).toUpperCase()
    avatarImg?.classList.add('hidden')
    avatarFallback.classList.remove('hidden')
    removeAvatarBtn?.classList.add('hidden')
  }
  
  if (usernameInput) usernameInput.value = user.username
  if (emailInput && (user as any).email) emailInput.value = (user as any).email
  if (roleInput) roleInput.value = user.role
  if (idInput) idInput.value = user.id.toString()
  
  if (emailStatus && (user as any).email) {
    const emailVerified = (user as any).email_verified
    emailStatus.classList.remove('hidden')
    
    if (emailVerified) {
      emailStatus.innerHTML = `
        <span class="inline-flex items-center text-xs text-green-600">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="mr-1">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
          Verificado
        </span>
      `
      verificationActions?.classList.add('hidden')
    } else {
      emailStatus.innerHTML = `
        <span class="inline-flex items-center text-xs text-amber-600">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="mr-1">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" x2="12" y1="8" y2="12"/>
            <line x1="12" x2="12.01" y1="16" y2="16"/>
          </svg>
          No verificado
        </span>
      `
      verificationActions?.classList.remove('hidden')
    }
  }
}

export function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  const toast = document.createElement('div')
  const colors = {
    success: 'bg-green-500',
    error: 'bg-destructive',
    info: 'bg-primary',
  }
  
  toast.className = `fixed bottom-4 right-4 ${colors[type]} text-white px-4 py-3 rounded-lg shadow-lg z-50 transition-all duration-300`
  toast.innerHTML = `<div class="flex items-center gap-2"><span>${message}</span></div>`
  
  document.body.appendChild(toast)
  
  setTimeout(() => {
    toast.style.opacity = '0'
    toast.style.transform = 'translateY(20px)'
    setTimeout(() => toast.remove(), 300)
  }, 3000)
}
