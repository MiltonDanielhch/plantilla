# Laboratorio Master 3026 - Instrucciones para Agente

## Contexto Técnico

**Stack:**
- Backend: Rust (Cargo)
- Frontend: Astro
- Scripts de soporte: Python y Shell (.sh)

**Orquestador:** Justfile (PowerShell en local, Bash en servidor)

**Estructura:**
- Scripts de utilidad: `scripts/automatizacion/`
- Scripts de despliegue: `scripts/automatizacion/despliegue/`
- Scripts de infraestructura: `infra/scripts/`

## Principios de Diseño

1. **Limpieza:** Ningún script ensucia la raíz. Todo se organiza en `scripts/automatizacion/`.

2. **Detección Activa:** Prioriza scripts que usen 'Observers' o 'Hooks' para actuar sin intervención humana.

3. **Seguridad:** Antes de cualquier acción, verifica que no haya secretos expuestos (Shield-First).

4. **Enseñanza:** Cada script debe generar un reporte legible (.md) para facilitar el aprendizaje y la auditoría.

## Estructura de Scripts

```
scripts/automatizacion/
├── MASTER_ORCHESTRATOR.py    # Cerebro central
├── auditoria/      # Ghost-Inspector, análisis de código
├── despliegue/     # Bootstrap, Ignition, Build, Deploy
├── monitoreo/      # Watchdog, Health-Check, Vigilante, Auto-Scaler
├── seguridad/      # Shield, Protector, Vulnerability-Scanner, SSL-Renewer
└── utilidades/    # Pruning, Bulk-Renamer, Git-Automator, Doc-Generator, etc.
```

## Triggers (Disparadores)

| Tipo | Descripción | Ejemplo |
|------|-------------|---------|
| Git Hooks | Por acción de Git | Pre-commit: Shield |
| CronJobs | Por tiempo | Daily: Pruning a las 3AM |
| Observers | Por evento de archivo | On-save: Watchdog |
| Manual | Por usuario | `just deploy` |

## Lista Completa de Scripts

### Seguridad
| Script | Función | Trigger |
|--------|---------|---------|
| Shield | Detecta API keys/passwords expuestos | Pre-commit hook |
| Protector | Bloquea IPs con ataques fuerza bruta | Cron 5min |
| Vulnerability Scanner | Escanea vulnerabilidades en dependencias | Antes de deploy |
| SSL Renewer | Renueva certificados Let's Encrypt | Cron semanal |

### Despliegue
| Script | Función | Trigger |
|--------|---------|---------|
| Bootstrap | Configura entorno desde cero | Nueva máquina |
| Build Package | Compila y empaqueta (Python/Node/Rust + Docker) | Antes de deploy |
| Deploy Maestro | Despliegue sin downtime | Manual/CI-CD |

### Monitoreo
| Script | Función | Trigger |
|--------|---------|---------|
| Health Check | Verifica endpoints y alerta si está caído | Cron 5min |
| Vigilante | Auto-restart si la app cae | Cron 1min |
| Watchdog | Ejecuta verificación al guardar archivos | Observando |
| Auto-Scaler | Escala docker-compose por CPU/memoria | Cron 1min |
| Metrics Collector | Recolecta métricas del sistema | Cron 30s |

### Datos
| Script | Función | Trigger |
|--------|---------|---------|
| Database Backup | Dump de PostgreSQL/MySQL a Dropbox | Cron diario |
| Database Migrator | Ejecuta migraciones (Alembic/Django/Prisma) | Antes de deploy |
| Database Health | Verifica salud de la BD | Cron |
| Rollback | Restaura versión anterior desde backup | Manual |

### Utilidades
| Script | Función | Trigger |
|--------|---------|---------|
| Git Automator | Add + Commit + Push automático | Manual |
| Doc Generator | Genera docs HTML desde docstrings | Post-merge |
| Doctor Setup | Verifica entorno para nuevos miembros | Onboarding |
| The Creator | Genera boilerplate de módulos | Manual |
| Bulk Renamer | Busca y reemplaza en múltiples archivos | Manual |
| Pruning Script | Limpia __pycache__, node_modules, etc. | Cron semanal |
| Snapshot | Backup .zip rápido fuera de Git | Antes de cambios |
| Unit Test Runner | Ejecuta pruebas automáticamente | Pre-push |
| Cache Manager | Limpia Redis, Python, Node, Docker | Manual |
| Notification Center | Envía notificaciones multi-canal | Manual |
| Container Manager | Gestiona contenedores Docker | Manual |
| Config Backup | Backup de configuraciones | Manual |
| Post-Mortem | Captura estado del sistema en crisis | Manual |
| Log Analyzer | Resume errores en logs | Diario |
| Deep Work Logger | Registra tiempo de trabajo por archivo | Sesión activa |

### Orquestación
| Script | Función | Trigger |
|--------|---------|---------|
| Master Orchestrator | Coordina todos los scripts | Cron 5min / Manual |

## Tarea del Agente

Cuando se pida una automatización, entregar:

1. **Código del script** (Python o Bash) optimizado
2. **Línea para Justfile** (exacta)
3. **Trigger** (cuándo ejecutarse)
4. **Explicación** breve del valor añadido

## Comandos Disponibles

```bash
# Seguridad
just shield                  # Escanear secretos
just protector              # Bloquear IPs maliciosas
just vuln-scan             # Vulnerabilidades en deps
just ssl-renew             # Renovar SSL

# Despliegue
just deploy                # Desplegar
just build                # Compilar
just bootstrap            # Configurar entorno

# Monitoreo
just health                # Verificar servicios
just vigilante            # Auto-restart
just auto-scale           # Escalar
just monitor              # Observar archivos

# Datos
just db-backup            # Backup BD
just db-migrate           # Migrar BD
just rollback             # Revertir versión

# Utilidades
just git-auto             # Commit+Push
just docs                 # Generar docs
just doctor               # Diagnosticar entorno
just create               # Crear módulo
just util-clean           # Limpiar proyecto
just snapshot             # Backup rápido
just postmortem           # Diagnóstico crisis
just log-analyzer         # Analizar logs

# Orquestación
just orchestrator         # Dashboard
just orchestrator-check  # Verificación total
just orchestrator-deploy # Pipeline completo
```

## Cronjobs Recomendados

```bash
# Monitoreo
* * * * * just vigilante > /dev/null 2>&1
*/5 * * * * just health > /dev/null 2>&1

# Mantenimiento
0 3 * * * just db-backup > /dev/null 2>&1
0 3 * * 0 just util-clean > /dev/null 2>&1

# Verificación diaria
0 9 * * * just orchestrator-check > /dev/null 2>&1
0 9 * * * just vuln-scan > /dev/null 2>&1

# SSL
0 4 * * 0 just ssl-renew > /dev/null 2>&1
```

## Activación

Código de activación: **"Código 3026: Activar Sintonía"**
