import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

/**
 * Combina clases de Tailwind CSS con soporte para condicionales
 * Usa clsx para manejar condicionales y tailwind-merge para evitar conflictos
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Formatea una fecha a string legible
 */
export function formatDate(date: string | Date | undefined | null): string {
  if (!date) return 'Fecha inválida';
  
  // Soporte para fechas SQL (YYYY-MM-DD HH:MM:SS) en Safari/Node
  if (typeof date === 'string' && date.includes(' ')) {
    date = date.replace(' ', 'T'); // Convertir a ISO 8601
  }

  const d = new Date(date);
  
  // Validar si es fecha válida
  if (isNaN(d.getTime())) {
    return String(date); // Retornar original si falla parseo
  }

  return new Intl.DateTimeFormat('es-ES', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(d)
}

/**
 * Formatea un número con separadores de miles
 */
export function formatNumber(num: number): string {
  return new Intl.NumberFormat('es-ES').format(num)
}
