// frontend/src/config.ts

// Usamos una ruta relativa para que las peticiones pasen por el proxy de Astro.
// Esto evita problemas de CORS y cookies con el backend en un puerto diferente.
export const API_BASE_URL = '/api/v1';