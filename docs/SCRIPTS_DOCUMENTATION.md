# 📚 Laboratorio Master 3026 - Documentación Completa

> Guía completa de automatización para desarrolladores SRE/DevOps

---

## 🎯 Introducción

El **Laboratorio Master 3026** es un conjunto de scripts de automatización diseñados para cubrir todo el ciclo de vida del software: desde el desarrollo local hasta producción, pasando por seguridad, monitoreo y despliegue.

### Filosofía

| Principio | Descripción |
|-----------|-------------|
| **Limpieza** | Ningún script ensucia la raíz del proyecto |
| **Detección Activa** | Scripts que actúan sin intervención humana |
| **Seguridad First** | Shield verifica secretos antes de cada commit |
| **Enseñanza** | Cada script genera reportes para aprender |

---

## 📁 Estructura de Scripts

```
scripts/automatizacion/
├── MASTER_ORCHESTRATOR.py    # Cerebro central
├── auditoria/                 # Análisis de código
├── despliegue/               # Despliegue y build
├── monitoreo/               # Observadores y health
├── seguridad/               # Protección y vulnerabilidades
└── utilidades/              # Herramientas diversas
```

---

## 🛡️ Seguridad

### Shield - Guardian de Secretos

**¿Qué hace?** Escanea todo el código en busca de API keys, passwords, tokens y otros secretos expuestos.

**¿Para qué sirve?** Protege tu proyecto de accidentalmente subir contraseñas o claves API a GitHub.

**Trigger:** Git pre-commit hook (se ejecuta antes de cada commit)

```bash
just shield                    # Ejecutar manualmente
```

**Ejemplo de salida:**
```
[SHIELD] Guardian de Secretos
[ALERTA] Se detectaron 2 posible(s) secreto(s):
1. [Generic Secret]
   backend/src/models/user.rs:214
   password: "password123"
```

---

### Protector - Bloqueador de IPs

**¿Qué hace?** Analiza los logs de autenticación y bloquea IPs que intenten ataques de fuerza bruta.

**¿Para qué sirve?** Protege el servidor de ataques dictionary attack.

**Trigger:** Cron cada 5 minutos

```bash
just protector                  # Ejecutar manualmente
```

---

### Vulnerability Scanner

**¿Qué hace?** Escanea todas las dependencias contra bases de datos de vulnerabilidades conocidas (npm audit, pip audit, cargo audit).

**¿Para qué sirve?** Detectar librerías con agujeros de seguridad antes de que sean explotadas.

**Trigger:** Antes de cada deploy

```bash
just vuln-scan                 # Escanear vulnerabilidades
```

---

### SSL Renewer

**¿Qué hace?** Verifica la fecha de expiración de certificados SSL y los renueva automáticamente si están por vencer.

**¿Para qué sirve?** Mantener el candado verde HTTPS sin intervención manual.

**Trigger:** Cron semanal

```bash
just ssl-renew                 # Verificar y renovar SSL
```

---

## 🚀 Despliegue

### Deploy Maestro

**¿Qué hace?** 
1. Verifica que no haya cambios sin commit
2. Crea backup antes de desplegar
3.Hace git pull e instala dependencias
4.Ejecuta migraciones de BD
5.Recarga servicios sin downtime

**¿Para qué sirve?** Desplegar código nuevo sin interrumpir a los usuarios.

**Trigger:** Manual o CI/CD pipeline

```bash
just deploy                    # Despliegue completo
```

---

### Build Package

**¿Qué hace?** Compila el proyecto (Python, Node.js, Rust), minifica archivos y crea contenedores Docker.

**¿Para qué sirve?** Preparar el código para producción.

**Trigger:** Antes de deploy

```bash
just build                     # Compilar y empaquetar
```

---

### Bootstrap

**¿Qué hace?** Configura el entorno de desarrollo desde cero: verifica Python, Node.js, Rust, Docker, configura Git hooks.

**¿Para qué sirve?** Onboarding rápido de nuevos desarrolladores.

**Trigger:** Nueva máquina o reinstalación

```bash
just bootstrap                # Configurar entorno
```

---

## 👁️ Monitoreo

### Health Check

**¿Qué hace?** "Golpea" las URLs de la app cada minuto. Si no responde, cuenta fallos y alerta por Telegram.

**¿Para qué sirve?** Detectar cuando la app está caída antes que los usuarios lo noten.

**Trigger:** Cron cada 5 minutos

```bash
just health                   # Verificar salud
```

---

### Vigilante

**¿Qué hace?** Revisa si el proceso de la app está corriendo. Si no, lo reinicia automáticamente.

**¿Para qué sirve?** Auto-recuperación cuando la app falla.

**Trigger:** Cron cada minuto

```bash
just vigilante                # Iniciar vigilante
```

---

### Auto-Scaler

**¿Qué hace?** Monitorea CPU y memoria. Si supera阈值, escala automáticamente docker-compose.

**¿Para qué sirve?** Manejar picos de tráfico sin intervención humana.

**Trigger:** Cron cada minuto

```bash
just auto-scale               # Verificar y escalar
just auto-scale-status        # Ver estado
```

---

### Watchdog

**¿Qué hace?** Observa cambios en archivos (.py, .js, .ts, .rs). Cuando guardas, ejecuta `just check` automáticamente.

**¿Para qué sirve?** Verificación continua mientras desarrollas.

**Trigger:** Se queda escuchando (no sale)

```bash
just monitor                  # Iniciar observador
```

---

## 📊 Datos

### Database Backup

**¿Qué hace?** Dump de PostgreSQL/MySQL, comprime y opcionalmente sube a Dropbox.

**¿Para qué sirve?** Backup automático diario de la base de datos.

**Trigger:** Cron (diario 3AM)

```bash
just db-backup               # Hacer backup
```

---

### Database Migrator

**¿Qué hace?** Ejecuta migraciones (Alembic, Django, Prisma, Knex) y guarda historial.

**¿Para qué sirve?** Mantener el schema de BD versionado.

**Trigger:** Antes de deploy

```bash
just db-migrate              # Ejecutar migraciones
just db-migrate-status      # Ver historial
just db-migrate-rollback   # Revertir última
```

---

### Rollback

**¿ qué hace?** Restaura la versión anterior desde el último backup.

**¿Para qué sirve?** Recuperarse rápido de un deploy malo.

**Trigger:** Manual cuando hay bug crítico

```bash
just rollback                # Rollback al último backup
just rollback --git 3       # Rollback 3 commits git
just rollback-list          # Ver backups disponibles
```

---

## 🛠️ Utilidades

### Git Automator

**¿Qué hace?** `git add -A`, commit con mensaje automático y push. Todo en uno.

**¿Para qué sirve?** Commit rápido sin memorizar comandos.

**Trigger:** Manual o post-commit hook

```bash
just git-auto                # Add + Commit + Push
just git-auto -m "mensaje"  # Con mensaje custom
just git-auto --no-push     # Solo commit
```

---

### Doc Generator

**¿Qué hace?** Lee todos los docstrings de Python/Rust y genera una página HTML con la documentación.

**¿Para qué sirve?** Documentación automática desde el código.

**Trigger:** Post-merge o manual

```bash
just docs                    # Generar docs
# Output: docs/generated/api_docs.html
```

---

### Doctor Setup

**¿Qué hace?** Verifica: Python, Node.js, Rust, Docker, Git, VS Code, estructura del proyecto, variables de entorno.

**¿Para qué sirve?** Diagnosticar por qué un nuevo desarrollador no puede correr el proyecto.

**Trigger:** Onboarding o debugging

```bash
just doctor                  # Diagnóstico completo
```

---

### The Creator

**¿Qué hace?** Crea la estructura completa de un nuevo módulo: carpetas, archivos base, tests, README.

**¿Para qué sirve?** No perder tiempo creando la misma estructura cada vez.

**Trigger:** Manual cuando necesitas un nuevo componente

```bash
just create usuarios         # Crear módulo usuarios
just create mi-componente astro  # Con tipo específico
```

---

### Bulk Renamer

**¿Qué hace?** Busca y reemplaza texto en múltiples archivos simultáneamente.

**¿Para qué sirve?** Refactorización masiva (ej: renombrar clase en 50 archivos).

**Trigger:** Manual

```bash
just util-bulk OldClass NewClass   # Reemplazar
just util-bulk foo bar --preview  # Solo previsualizar
```

---

### Pruning Script

**¿Qué hace?** Elimina: `__pycache__`, `.pyc`, `node_modules`, `.log`, `dist`, `target`.

**¿Para qué sirve?** Limpiar proyecto de archivos generados.

**Trigger:** Manual o cron semanal

```bash
just util-clean              # Limpiar
just util-clean --dry-run   # Solo previsualizar
```

---

### Snapshot

**¿Qué hace?** Crea un .zip con timestamp antes de cambios arriesgados.

**¿Para qué sirve?** Backup rápido fuera de Git.

**Trigger:** Antes de experimentos

```bash
just snapshot                # Crear snapshot
just snapshot-list          # Ver disponibles
```

---

### Post-Mortem

**¿Qué hace?** Cuando hay crisis: captura CPU, memoria, disco, procesos, logs recientes y variables de entorno. Genera reporte markdown.

**¿Para qué sirve?** Diagnosticar qué mató al servidor.

**Trigger:** Manual cuando hay error 500 o crash

```bash
just postmortem              # Generar diagnóstico
# Output: logs/postmortem/2026-02-19_15-30-00.md
```

---

### Log Analyzer

**¿Qué hace?** Escanea todos los logs, cuenta errores por tipo (ERROR, WARNING, 404, 500, OOM, timeout).

**¿Para qué sirve?** Resumen diario de qué está fallando.

**Trigger:** Diario o manual

```bash
just log-analyzer           # Analizar logs
# Output: logs/log_analysis.md
```

---

### Deep Work Logger

**¿Qué hace?** Registra en qué archivos trabajas y cuánto tiempo. Genera reportes semanales.

**¿Para qué sirve?** Métricas de productividad personal.

**Trigger:** Mientras trabajas

```bash
just deepwork               # Iniciar sesión
just deepwork-report       # Ver reporte semanal
```

---

## 🧠 Orquestador Maestro

### MASTER_ORCHESTRATOR

**¿Qué hace?** Es el "cerebro" que coordina todos los scripts:

- Verifica salud de servicios
- Revisa recursos (CPU, memoria, disco)
- Ejecuta Shield (secretos)
- Ejecuta Ghost Hunter (dependencias)
- Auto-recuperación si detecta problemas
- Notificaciones por Telegram

**¿Para qué sirve?** Monitoreo centralizado que decide qué hacer según el estado.

**Trigger:** Cron cada 5 minutos o manual

```bash
just orchestrator            # Dashboard general
just orchestrator-check     # Verificación completa
just orchestrator-dashboard # Solo ver estado
just orchestrator-deploy    # Pipeline de deploy completo
```

**Pipeline de deploy (`orchestrator-deploy`):**
1. Ghost Hunter - Limpiar dependencias
2. Shield - Verificar secretos
3. Si hay secretos → ABORTAR
4. Deploy Maestro
5. Health Check
6. Notificación Telegram

---

## ⏰ Triggers Recomendados (Cron)

```bash
# Monitoreo constante (en background)
* * * * * just vigilante > /dev/null 2>&1
*/5 * * * * just health > /dev/null 2>&1

# Mantenimiento (madrugada)
0 3 * * * just db-backup > /dev/null 2>&1
0 3 * * 0 just limpieza > /dev/null 2>&1

# Verificación diaria
0 9 * * * just orchestrator-check > /dev/null 2>&1
0 9 * * * just vuln-scan > /dev/null 2>&1

# SSL (semanal)
0 4 * * 0 just ssl-renew > /dev/null 2>&1
```

---

## 🎓 Aprendizaje: Cómo Contribuir

### Añadir un nuevo script

1. **Crear** el archivo en la carpeta correcta:
   - `seguridad/` - Para protección
   - `monitoreo/` - Para observación
   - `utilidades/` - Para herramientas
   - `desplegue/` - Para deployment

2. **Estructura recomendada:**
```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
NOMBRE - Descripción corta
Trigger: Cron o manual
"""
import sys
# ... código ...
def main():
    # ... lógica ...
    return 0

if __name__ == "__main__":
    sys.exit(main())
```

3. **Añadir al Justfile:**
```just
nombre:
    python scripts/automatizacion/carpeta/script.py
```

4. **Documentar** en este archivo

---

## 📋 Comandos Completos

| Comando | Descripción |
|---------|-------------|
| `just shield` | Escanear secretos |
| `just protector` | Bloquear IPs maliciosas |
| `just vuln-scan` | Vulnerabilidades en deps |
| `just ssl-renew` | Renovar SSL |
| `just deploy` | Desplegar |
| `just build` | Compilar |
| `just bootstrap` | Configurar entorno |
| `just health` | Verificar servicios |
| `just vigilante` | Auto-restart |
| `just auto-scale` | Escalar |
| `just monitor` | Observar archivos |
| `just db-backup` | Backup BD |
| `just db-migrate` | Migrar BD |
| `just rollback` | Revertir versión |
| `just git-auto` | Commit+Push |
| `just docs` | Generar docs |
| `just doctor` | Diagnosticar entorno |
| `just create` | Crear módulo |
| `just util-clean` | Limpiar proyecto |
| `just snapshot` | Backup rápido |
| `just postmortem` | Diagnóstico crisis |
| `just log-analyzer` | Analizar logs |
| `just orchestrator` | Dashboard |
| `just orchestrator-check` | Verificación total |
| `just test` | Ejecutar pruebas |
| `just metrics` | Métricas del sistema |
| `just cache` | Gestor de caché |
| `just notify` | Notificaciones |
| `just containers` | Gestor Docker |
| `just db-health` | Salud de BD |
| `just config-backup` | Backup configs |

---

## 🚀 Inicio Rápido

```bash
# 1. Ver todo lo disponible
just --list

# 2. Verificar seguridad
just shield

# 3. Verificar salud
just health

# 4. Ver estado general
just orchestrator-dashboard

# 5. Deploy completo
just orchestrator-deploy
```

---

*Documento generado por Laboratorio Master 3026*
*Actualizado: 2026-02-19*
