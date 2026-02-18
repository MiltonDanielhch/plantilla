import { api } from '../../lib/api'
import { formatDate } from '../../lib/utils'
import type { AuditLog } from '../../types'

export const actionColors: Record<string, string> = {
  'LOGIN': 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400',
  'LOGOUT': 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400',
  'CREATE_USER': 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400',
  'DELETE_USER': 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
  'UPDATE_USER': 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400',
}

export const actionIcons: Record<string, string> = {
  'LOGIN': '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><polyline points="10 17 15 12 10 7"/><line x1="15" x2="3" y1="12" y2="12"/></svg>',
  'LOGOUT': '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/></svg>',
  'CREATE_USER': '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="20" x2="20" y1="8" y2="14"/><line x1="23" x2="17" y1="11" y2="11"/></svg>',
  'DELETE_USER': '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="18" x2="23" y1="11" y2="11"/></svg>',
  'UPDATE_USER': '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><path d="M17 8l5-5"/><path d="M22 8L17 3"/></svg>',
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

export function renderTimeline(
  logs: AuditLog[],
  currentPage: number,
  itemsPerPage: number,
  timelineContainer: HTMLElement,
  paginationContainer: HTMLElement,
  onPageChange: (page: number) => void
) {
  const start = (currentPage - 1) * itemsPerPage
  const end = start + itemsPerPage
  const pageLogs = logs.slice(start, end)
  
  if (pageLogs.length === 0) {
    timelineContainer.innerHTML = `
      <div class="flex flex-col items-center justify-center py-12 text-center animate-in fade-in-50">
        <div class="flex h-20 w-20 items-center justify-center rounded-full bg-muted/50 mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-10 w-10 text-muted-foreground"><path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"/><path d="M14 2v6h6"/><path d="m3 12.5 5 5"/><path d="m8 12.5-5 5"/></svg>
        </div>
        <h3 class="text-lg font-semibold tracking-tight">No se encontraron eventos</h3>
        <p class="mt-2 text-sm text-muted-foreground max-w-sm text-balance">
          No hay resultados que coincidan con los filtros seleccionados.
        </p>
      </div>
    `
    paginationContainer.innerHTML = ''
    return
  }
  
  const grouped = pageLogs.reduce((acc, log) => {
    const dateStr = log.created_at || log.timestamp || ""
    const d = dateStr.includes(' ') ? new Date(dateStr.replace(' ', 'T')) : new Date(dateStr)
    
    const date = d.toLocaleDateString('es-ES', { 
      weekday: 'long', 
      year: 'numeric', 
      month: 'long', 
      day: 'numeric' 
    })
    if (!acc[date]) acc[date] = []
    acc[date].push(log)
    return acc
  }, {} as Record<string, AuditLog[]>)
  
  timelineContainer.innerHTML = Object.entries(grouped).map(([date, dayLogs]) => `
    <div class="relative pl-8 pb-8 last:pb-0">
      <div class="absolute left-3 top-2 bottom-0 w-px bg-border"></div>
      
      <div class="absolute left-0 top-1">
        <div class="h-6 w-6 rounded-full bg-primary/10 flex items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-primary"><rect width="18" height="18" x="3" y="4" rx="2" ry="2"/><line x1="16" x2="16" y1="2" y2="6"/><line x1="8" x2="8" y1="2" y2="6"/><line x1="3" x2="21" y1="10" y2="10"/></svg>
        </div>
      </div>
      
      <h3 class="font-semibold text-sm mb-4 capitalize">${date}</h3>
      
      <div class="space-y-3">
        ${dayLogs.map(log => `
          <div class="relative pl-6 pb-4 last:pb-0">
            <div class="absolute left-0 top-1">
              <div class="h-3 w-3 rounded-full bg-primary/30"></div>
            </div>
            
            <div class="rounded-lg border bg-card p-4 hover:bg-accent/50 transition-colors">
              <div class="flex items-start justify-between gap-4">
                <div class="flex items-start gap-3">
                  <div class="h-8 w-8 rounded-full ${actionColors[log.action] || 'bg-primary/10 text-primary'} flex items-center justify-center shrink-0">
                    ${actionIcons[log.action] || actionIcons['LOGIN']}
                  </div>
                  <div>
                    <p class="font-medium text-sm">${log.action.replace(/_/g, ' ')}</p>
                    <p class="text-xs text-muted-foreground mt-0.5">
                      Por: <span class="font-medium text-foreground">${log.username || log.admin_username || "Sistema"}</span>
                    </p>
                    ${(log.details || log.target) ? `
                      <p class="text-xs text-muted-foreground mt-1">
                        ${log.details || `Objetivo: ${log.target}`}
                      </p>
                    ` : ''}
                  </div>
                </div>
                <time class="text-xs text-muted-foreground shrink-0">
                  ${formatDate(log.created_at || log.timestamp || "")}
                </time>
              </div>
            </div>
          </div>
        `).join('')}
      </div>
    </div>
  `).join('')
  
  renderPagination(logs.length, currentPage, itemsPerPage, paginationContainer, onPageChange)
}

function renderPagination(
  totalItems: number,
  currentPage: number,
  itemsPerPage: number,
  container: HTMLElement,
  onPageChange: (page: number) => void
) {
  const totalPages = Math.ceil(totalItems / itemsPerPage)
  const start = (currentPage - 1) * itemsPerPage
  const end = start + itemsPerPage
  
  if (totalPages <= 1) {
    container.innerHTML = `
      <div class="text-sm text-muted-foreground">
        Mostrando ${totalItems} eventos
      </div>
      <div></div>
    `
    return
  }
  
  container.innerHTML = `
    <div class="text-sm text-muted-foreground">
      Mostrando ${start + 1} a ${Math.min(end, totalItems)} de ${totalItems} eventos
    </div>
    <div class="flex items-center gap-1">
      <button id="prev-page" class="inline-flex h-8 w-8 items-center justify-center rounded-md border text-sm font-medium transition-colors ${currentPage === 1 ? 'pointer-events-none opacity-50' : 'hover:bg-accent hover:text-accent-foreground'}">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m15 18-6-6 6-6"/></svg>
      </button>
      
      ${Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
        const pageNum = i + 1
        const isActive = pageNum === currentPage
        return `<button class="page-btn inline-flex h-8 w-8 items-center justify-center rounded-md text-sm font-medium transition-colors ${isActive ? 'bg-primary text-primary-foreground' : 'hover:bg-accent hover:text-accent-foreground'}" data-page="${pageNum}">${pageNum}</button>`
      }).join('')}
      
      ${totalPages > 5 ? '<span class="px-2 text-muted-foreground">...</span>' : ''}
      
      <button id="next-page" class="inline-flex h-8 w-8 items-center justify-center rounded-md border text-sm font-medium transition-colors ${currentPage === totalPages ? 'pointer-events-none opacity-50' : 'hover:bg-accent hover:text-accent-foreground'}">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m9 18 6-6-6-6"/></svg>
      </button>
    </div>
  `
  
  document.getElementById('prev-page')?.addEventListener('click', () => {
    if (currentPage > 1) onPageChange(currentPage - 1)
  })
  
  document.getElementById('next-page')?.addEventListener('click', () => {
    if (currentPage < totalPages) onPageChange(currentPage + 1)
  })
  
  document.querySelectorAll('.page-btn').forEach(btn => {
    btn.addEventListener('click', (e) => {
      const page = parseInt((e.currentTarget as HTMLElement).dataset.page!)
      onPageChange(page)
    })
  })
}
