import { showToast } from './settings'

export function initThemeSelector() {
  const themeBtns = document.querySelectorAll('.theme-btn')
  
  const applyTheme = (theme: string) => {
    const root = document.documentElement
    const isDark = theme === 'dark' || 
      (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    
    if (isDark) root.classList.add('dark')
    else root.classList.remove('dark')
    
    localStorage.setItem('theme', theme)
  }

  const currentTheme = localStorage.getItem('theme') || 'dark'
  themeBtns.forEach(btn => {
    const theme = btn.getAttribute('data-theme')
    if (theme === currentTheme) {
      btn.classList.add('border-primary')
      btn.querySelector('.theme-indicator')?.classList.remove('hidden')
    } else {
      btn.classList.remove('border-primary')
      btn.querySelector('.theme-indicator')?.classList.add('hidden')
    }
  })
  
  themeBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const theme = btn.getAttribute('data-theme')
      if (!theme) return
      
      themeBtns.forEach(b => {
        b.classList.remove('border-primary')
        b.querySelector('.theme-indicator')?.classList.add('hidden')
      })
      btn.classList.add('border-primary')
      btn.querySelector('.theme-indicator')?.classList.remove('hidden')
      
      applyTheme(theme)
      
      const labels: Record<string, string> = {
        light: 'Claro', dark: 'Oscuro', system: 'Sistema'
      }
      showToast(`Tema ${labels[theme] || theme} activado`, 'success')
    })
  })
}
