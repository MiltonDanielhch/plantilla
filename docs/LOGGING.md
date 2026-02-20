# 📝 Logging en Sintonía 3026

## 📋 Sistema de Logs Implementado

### Características

✅ **Logs duales**: Consola + Archivo
✅ **Rotación diaria**: `logs/backend-YYYY-MM-DD.log`
✅ **Formato JSON**: En archivos para procesamiento
✅ **Formato legible**: En consola para desarrollo
✅ **Unificado**: Logs de backend y frontend separados

---

## 🚀 Uso Rápido

### Opción 1: Comando Just (Recomendado)

```bash
# Iniciar todo con logs
just run-with-logs

# Ver logs en tiempo real (en otra terminal)
just logs-backend    # Solo backend
just logs-frontend   # Solo frontend
just logs-all        # Combinados
```

### Opción 2: Script Bash

```bash
# Hacer ejecutable (primera vez)
chmod +x scripts/start-with-logs.sh

# Ejecutar
./scripts/start-with-logs.sh
```

### Opción 3: Manual (Desarrollo)

```bash
# Terminal 1: Backend
cd backend
cargo run 2>&1 | tee logs/backend-$(date +%Y-%m-%d).log

# Terminal 2: Frontend
cd frontend
npm run dev 2>&1 | tee logs/frontend-$(date +%Y-%m-%d).log
```

---

## 📁 Estructura de Logs

```
logs/
├── backend-2026-02-18.log      # Logs del backend (JSON)
├── frontend-2026-02-18.log     # Logs del frontend
├── sistema-2026-02-18.log      # Combinado (backend + frontend)
└── ...
```

### Formato Backend (JSON)

```json
{
  "timestamp": "2026-02-18T14:00:19.879160Z",
  "level": "INFO",
  "fields": {
    "message": "💾 Memoria conectada: sqlite://backend.db"
  },
  "target": "backend"
}
```

### Formato Frontend (Texto)

```
09:47:22 [WARN] [config] This project contains server-rendered routes...
09:47:22 [types] Generated 3ms
09:47:22 [content] Syncing content
```

---

## 🔧 Configuración

### Cambiar nivel de logs

**Backend** - `backend/config/default.toml`:
```toml
log_level = "info"  # error, warn, info, debug, trace
```

**Frontend** - No aplica (Astro usa su propio sistema)

### Rotación de logs

Por defecto: **Diaria** (`Rotation::DAILY`)

Opciones en `backend/src/main.rs`:
```rust
// Rotación por minuto (pruebas)
Rotation::MINUTELY

// Rotación por hora
Rotation::HOURLY

// Rotación por día (producción)
Rotation::DAILY
```

---

## 📊 Comandos Útiles

### Ver logs en tiempo real

```bash
# Backend
tail -f logs/backend-2026-02-18.log

# Frontend
tail -f logs/frontend-2026-02-18.log

# Ambos (combinado)
tail -f logs/sistema-2026-02-18.log
```

### Buscar en logs

```bash
# Buscar errores
grep "ERROR" logs/backend-2026-02-18.log

# Buscar requests específicos
grep "/api/v1/users" logs/backend-2026-02-18.log

# Buscar en frontend
grep "error" logs/frontend-2026-02-18.log
```

### Limpiar logs antiguos

```bash
# Borrar logs de hace más de 7 días
find logs/ -name "*.log" -mtime +7 -delete

# O con PowerShell
Get-ChildItem logs/ -Filter "*.log" | Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-7) } | Remove-Item
```

---

## 🐛 Troubleshooting

### Problema: "No se pudo crear directorio logs/"

**Solución:** Verificar permisos
```bash
# Linux/Mac
chmod 755 .
mkdir -p logs

# Windows (PowerShell Admin)
New-Item -ItemType Directory -Force -Path logs
```

### Problema: "Permiso denegado al escribir logs"

**Solución:** Verificar permisos de escritura
```bash
# Linux/Mac
chmod 755 logs

# O ejecutar desde directorio padre
cd C:\1proyecto\plantilla
just run-with-logs
```

### Problema: Logs muy grandes

**Solución:** La rotación diaria ya maneja esto, pero para limpiar:
```bash
# Comprimir logs antiguos
gzip logs/backend-2026-02-*.log

# Borrar logs de más de 30 días
find logs/ -name "*.log" -mtime +30 -delete
```

---

## 🎯 Monitoreo en Producción (VPS $5)

### Opción 1: Script simple con cron

```bash
# Agregar a crontab
crontab -e

# Rotar logs semanalmente (domingos a las 3am)
0 3 * * 0 cd /path/to/plantilla && find logs/ -name "*.log" -mtime +7 -delete

# Comprimir logs diariamente
0 2 * * * cd /path/to/plantilla && gzip logs/*.log.1 2>/dev/null
```

### Opción 2: Usar logrotate (Linux)

Crear `/etc/logrotate.d/sintonia-3026`:
```
/path/to/plantilla/logs/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 644 user user
}
```

### Opción 3: Systemd journal (Avanzado)

```bash
# Ver logs del servicio
sudo journalctl -u sintonia-backend -f

# Ver últimos 100 logs
sudo journalctl -u sintonia-backend -n 100

# Ver logs de hoy
sudo journalctl -u sintonia-backend --since today
```

---

## 📈 Mejores Prácticas

1. **Desarrollo**: Usar `just run-with-logs` para ver todo
2. **Debugging**: Buscar errores específicos con grep
3. **Producción**: Rotar logs automáticamente
4. **Backups**: Comprimir logs antes de borrarlos
5. **Monitoreo**: Revisar logs de errores diariamente

---

## 🔗 Comandos Rápidos

```bash
# Iniciar con logs
just run-with-logs

# Ver logs backend
just logs-backend

# Ver logs frontend
just logs-frontend

# Ver logs combinados
just logs-all

# Buscar errores
grep -i "error" logs/backend-$(date +%Y-%m-%d).log

# Ver últimas líneas
tail -20 logs/backend-$(date +%Y-%m-%d).log
```

---

**Documentación creada:** 2026-02-18
**Versión:** 1.0
**Sistema:** Sintonía 3026
