import { api } from '../../lib/api'
import { showToast } from './settings'

export function initEventListeners() {
  document.getElementById('save-profile-btn')?.addEventListener('click', async () => {
    const email = (document.getElementById('email-input') as HTMLInputElement).value
    const id = (document.getElementById('id-input') as HTMLInputElement).value
    
    try {
      await api.updateUser(parseInt(id), { email })
      showToast('Perfil actualizado correctamente', 'success')
    } catch (err: any) {
      showToast(err.message || 'Error al actualizar perfil', 'error')
    }
  })
  
  document.getElementById('change-password-btn')?.addEventListener('click', async () => {
    const current = (document.getElementById('current-password') as HTMLInputElement)?.value
    const newPass = (document.getElementById('new-password') as HTMLInputElement)?.value
    const confirm = (document.getElementById('confirm-password') as HTMLInputElement)?.value
    
    if (!current || !newPass || !confirm) {
      showToast('Por favor completa todos los campos', 'error')
      return
    }
    
    if (newPass !== confirm) {
      showToast('Las contraseñas no coinciden', 'error')
      return
    }
    
    if (newPass.length < 8) {
      showToast('La contraseña debe tener al menos 8 caracteres', 'error')
      return
    }
    
    const btn = document.getElementById('change-password-btn') as HTMLButtonElement
    const originalText = btn.textContent || 'Cambiar contraseña'
    
    try {
      btn.disabled = true
      btn.textContent = 'Actualizando...'
      await api.changePassword({ current_password: current, new_password: newPass })
      showToast('Contraseña actualizada correctamente', 'success')
      ;(document.getElementById('current-password') as HTMLInputElement).value = ''
      ;(document.getElementById('new-password') as HTMLInputElement).value = ''
      ;(document.getElementById('confirm-password') as HTMLInputElement).value = ''
    } catch (error: any) {
      showToast(error.message || 'Error al cambiar contraseña', 'error')
    } finally {
      btn.disabled = false
      btn.textContent = originalText
    }
  })
  
  document.getElementById('delete-account-btn')?.addEventListener('click', async () => {
    const id = (document.getElementById('id-input') as HTMLInputElement).value
    if (confirm('¿Estás seguro? Esta acción no se puede deshacer.')) {
      try {
        await api.deleteUser(parseInt(id))
        window.location.href = '/login'
      } catch (err: any) {
        showToast(err.message || 'Error al eliminar cuenta', 'error')
      }
    }
  })
  
  document.getElementById('logout-all-btn')?.addEventListener('click', async () => {
    if (confirm('¿Cerrar todas las sesiones? Tendrás que iniciar sesión nuevamente.')) {
      await api.logoutAll()
    }
  })
  
  const avatarInput = document.getElementById('avatar-input') as HTMLInputElement
  const uploadAvatarBtn = document.getElementById('upload-avatar-btn')
  const avatarLoading = document.getElementById('avatar-loading')
  
  uploadAvatarBtn?.addEventListener('click', () => avatarInput?.click())
  
  avatarInput?.addEventListener('change', async () => {
    const file = avatarInput.files?.[0]
    if (!file) return
    
    if (!file.type.startsWith('image/')) {
      showToast('El archivo debe ser una imagen', 'error')
      return
    }
    
    if (file.size > 2 * 1024 * 1024) {
      showToast('El archivo es demasiado grande (máximo 2MB)', 'error')
      return
    }
    
    try {
      avatarLoading?.classList.remove('hidden')
      avatarLoading?.classList.add('flex')
      await api.uploadAvatar(file)
      showToast('Avatar actualizado correctamente', 'success')
    } catch (err: any) {
      showToast(err.message || 'Error al subir avatar', 'error')
    } finally {
      avatarLoading?.classList.add('hidden')
      avatarLoading?.classList.remove('flex')
      avatarInput.value = ''
    }
  })
  
  document.getElementById('remove-avatar-btn')?.addEventListener('click', () => {
    showToast('Eliminación de avatar no implementada', 'info')
  })
  
  document.getElementById('resend-verification-btn')?.addEventListener('click', async () => {
    const btn = document.getElementById('resend-verification-btn') as HTMLButtonElement
    try {
      btn.disabled = true
      btn.textContent = 'Enviando...'
      await api.sendVerificationEmail()
      showToast('Email de verificación enviado. Revisa tu bandeja de entrada.', 'success')
      btn.textContent = 'Email enviado'
    } catch (err: any) {
      showToast(err.message || 'Error al enviar email', 'error')
      btn.disabled = false
      btn.textContent = 'Reenviar email de verificación'
    }
  })
}
