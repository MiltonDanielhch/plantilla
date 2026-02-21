# Configuración para PowerShell (Windows)
set shell := ["powershell", "-c"]

default:
    @just --list

# 🚀 Iniciar Backend (El Núcleo)
run-backend:
    cd backend; cargo run

# 🎨 Iniciar Frontend (La Vitrina)
run-frontend:
    cd frontend; npm run dev

# 🔍 Auditoría Maestro (Ejecuta tu script de consultoría)
audit:
    Write-Host "🌀 Activando Sintonía: Analizando arquitectura..." -ForegroundColor Magenta
    python infra/scripts/consultor.py

# 🧹 Verificar Calidad de Código (Lint & Format)
check:
    Write-Host "🦀 Verificando Backend..." -ForegroundColor Cyan
    cd backend; cargo fmt; cargo clippy
    Write-Host "🎨 Verificando Frontend..." -ForegroundColor Cyan
    cd frontend; npm run lint
    just audit
    Write-Host "✅ Todo limpio y auditado." -ForegroundColor Green

# 🛡️ Escudo de Seguridad (Shield - Pre-commit hook)
shield:
    Write-Host "🛡️ Escaneando secretos y API Keys..." -ForegroundColor Yellow
    python infra/scripts/shield.py

# 👻 Cazador de Código Basura (Auditoría)
ghost:
    Write-Host "👻 Ejecutando Ghost Hunter..." -ForegroundColor Cyan
    python infra/scripts/ghost_hunter.py

# 🚀 Ignition - Inicio de Proyecto
ignition:
    Write-Host "🚀 Iniciando nuevo proyecto..." -ForegroundColor Green
    python infra/scripts/ignition.py

# 📝 Iniciar con Logs (Backend + Frontend con logging a archivo)
run-with-logs:
    Write-Host "🚀 Iniciando Sintonía 3026 con logging..." -ForegroundColor Green
    Write-Host "📁 Logs se guardarán en: logs/" -ForegroundColor Cyan
    bash scripts/start-with-logs.sh

# ⚙️ Utilidades de Automatización
util-clean dry_run='--dry-run':
    Write-Host "🧹 Ejecutando Pruning-Script..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/Pruning-Script.py {{dry_run}}

util-bulk:
    Write-Host "📦 Ejecutando Bulk-Renamer..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/Bulk-Renamer-Replacer.py

# 👁️ Monitoreo (Observers)
monitor:
    Write-Host "👁️ Iniciando Watchdog..." -ForegroundColor Cyan
    python scripts/automatizacion/monitoreo/Watchdog.py

# 🧩 The Creator - Generador de Módulos
create name='':
    Write-Host "🧩 Creando nuevo módulo..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/The-Creator.py {{name}}

# 📸 Snapshot - Backup Rápido
snapshot:
    Write-Host "📸 Creando snapshot..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/Snapshot-Script.py

snapshot-list:
    python scripts/automatizacion/utilidades/Snapshot-Script.py --list

# 🔄 Transformer - Convertir Formatos
transform:
    Write-Host "🔄 Convirtiendo formatos..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/The-Transformer.py

# 🌐 Polyglot - Traducir Docs
translate:
    Write-Host "🌐 Traduciendo documentación..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/Polyglot.py

# 🧠 Deep Work - Registrar Tiempo
deepwork:
    Write-Host "🧠 Iniciando Deep Work Logger..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/Deep-Work-Logger.py

deepwork-report:
    python scripts/automatizacion/utilidades/Deep-Work-Logger.py --report 7

# 🚀 Bootstrap - Configurar Entorno
bootstrap:
    Write-Host "🚀 Ejecutando Bootstrap..." -ForegroundColor Green
    python scripts/automatizacion/despliegue/Bootstrap-Script.py

# 📦 Deploy Maestro - Despliegue sin Caída
deploy:
    Write-Host "🚀 Iniciando despliegue..." -ForegroundColor Green
    bash scripts/automatizacion/despliegue/deploy_maestro.sh

# 👁️ Vigilante - Auto-Restart
vigilante:
    Write-Host "👁️ Iniciando vigilante..." -ForegroundColor Cyan
    bash scripts/automatizacion/monitoreo/vigilante.sh

# 🛡️ Protector - IP Baner
protector:
    Write-Host "🛡️ Ejecutando Protector..." -ForegroundColor Yellow
    python scripts/automatizacion/seguridad/protector.py

# 🧹 Limpieza - Log Cleaner
limpieza:
    Write-Host "🧹 Ejecutando limpieza..." -ForegroundColor Cyan
    bash scripts/automatizacion/utilidades/limpieza.sh

# 🔄 Log Rotator - Analizador de Logs
log-analyzer:
    Write-Host "🔄 Analizando logs..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/log-rotator.py

# 💚 Health-Check - Monitor de Salud
health:
    Write-Host "💚 Verificando salud..." -ForegroundColor Cyan
    python scripts/automatizacion/monitoreo/health-check.py

# 📜 Git Automator - Auto Commit/Push
git-auto:
    Write-Host "📜 Ejecutando Git Automator..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/git-automator.py

# 📚 Doc Generator - Generar Docs
docs:
    Write-Host "📚 Generando documentación..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/doc-generator.py

# 🩺 Doctor Setup - Verificar Entorno
doctor:
    Write-Host "🩺 Ejecutando Doctor Setup..." -ForegroundColor Green
    python scripts/automatizacion/utilidades/doctor-setup.py

# 💾 Database Backup - Backup de BD
db-backup:
    Write-Host "💾 Ejecutando backup de BD..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/database-backup.py

# 📸 Post-Mortem - Diagnóstico de Crisis
postmortem:
    Write-Host "📸 Generando diagnóstico..." -ForegroundColor Red
    python scripts/automatizacion/utilidades/post-mortem.py

# ⏪ Rollback - Reversión
rollback:
    Write-Host "⏪ Ejecutando rollback..." -ForegroundColor Red
    bash scripts/automatizacion/utilidades/rollback.sh

rollback-list:
    bash scripts/automatizacion/utilidades/rollback.sh --list

# 🧠 Master Orchestrator - Cerebro Central
orchestrator:
    Write-Host "🧠 Ejecutando Orchestrator..." -ForegroundColor Magenta
    python scripts/automatizacion/MASTER_ORCHESTRATOR.py

orchestrator-check:
    python scripts/automatizacion/MASTER_ORCHESTRATOR.py --check

orchestrator-dashboard:
    python scripts/automatizacion/MASTER_ORCHESTRATOR.py --dashboard

orchestrator-deploy:
    Write-Host "🧠 Ejecutando pipeline de deploy..." -ForegroundColor Magenta
    python scripts/automatizacion/MASTER_ORCHESTRATOR.py --deploy

# 🔐 SSL Renewer - Renovador de SSL
ssl-renew:
    Write-Host "🔐 Verificando certificados SSL..." -ForegroundColor Yellow
    python scripts/automatizacion/seguridad/ssl-renewer.py

# 📦 Build & Package - Compilación
build:
    Write-Host "📦 Compilando y empaquetando..." -ForegroundColor Cyan
    bash scripts/automatizacion/despliegue/build-package.sh

# 🔍 Vulnerability Scanner - Escáner de Vulnerabilidades
vuln-scan:
    Write-Host "🔍 Escaneando vulnerabilidades..." -ForegroundColor Yellow
    python scripts/automatizacion/seguridad/vulnerability-scanner.py

# 🗄️ Database Migrator - Migrador de BD
db-migrate:
    Write-Host "🗄️ Ejecutando migraciones..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/database-migrator.py

db-migrate-rollback:
    Write-Host "🗄️ Rollback de migración..." -ForegroundColor Yellow
    python scripts/automatizacion/utilidades/database-migrator.py --rollback

db-migrate-status:
    python scripts/automatizacion/utilidades/database-migrator.py --status

# ⚖️ Auto-Scaler - Autoescalado
auto-scale:
    Write-Host "⚖️ Verificando autoescalado..." -ForegroundColor Cyan
    python scripts/automatizacion/monitoreo/auto-scaler.py

auto-scale-status:
    python scripts/automatizacion/monitoreo/auto-scaler.py --status

# 📊 Metrics Collector - Recolector de Métricas
metrics:
    Write-Host "📊 Recolectando métricas..." -ForegroundColor Cyan
    python scripts/automatizacion/monitoreo/metrics-collector.py

metrics-stream:
    python scripts/automatizacion/monitoreo/metrics-collector.py --stream

# 🧪 Unit Test Runner - Ejecutor de Pruebas
test:
    Write-Host "🧪 Ejecutando pruebas..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/unit-test-runner.py

# 🗃️ Cache Manager - Gestor de Caché
cache:
    Write-Host "🗃️ Gestor de caché..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/cache-manager.py

cache-redis:
    python scripts/automatizacion/utilidades/cache-manager.py --redis

cache-python:
    python scripts/automatizacion/utilidades/cache-manager.py --python

cache-docker:
    python scripts/automatizacion/utilidades/cache-manager.py --docker

cache-stats:
    python scripts/automatizacion/utilidades/cache-manager.py --stats

# 🔔 Notification Center - Centro de Notificaciones
notify *args:
    Write-Host "🔔 Enviando notificación..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/notification-center.py {{args}}

# 🐳 Container Manager - Gestor de Docker
containers:
    Write-Host "🐳 Gestor de contenedores..." -ForegroundColor Cyan
    bash scripts/automatizacion/utilidades/container-manager.sh

containers-status:
    bash scripts/automatizacion/utilidades/container-manager.sh status

containers-logs:
    bash scripts/automatizacion/utilidades/container-manager.sh logs

containers-stats:
    bash scripts/automatizacion/utilidades/container-manager.sh stats

containers-restart:
    bash scripts/automatizacion/utilidades/container-manager.sh restart

# 💊 Database Health - Salud de BD
db-health:
    Write-Host "💊 Verificando salud de BD..." -ForegroundColor Cyan
    python scripts/automatizacion/utilidades/database-health.py

# 📋 Config Backup - Backup de Configs
config-backup:
    Write-Host "📋 Respaldando configuraciones..." -ForegroundColor Cyan
    bash scripts/automatizacion/utilidades/config-backup.sh

# 🤖 IA - Analizadores Automáticos
ai-analyze:
    Write-Host "🤖 Ejecutando análisis IA..." -ForegroundColor Magenta
    python scripts/automatizacion/auditoria/ai-assistant.py --analyze

ai-analyze-code:
    python scripts/automatizacion/auditoria/code-analyzer.py

ai-analyze-security:
    python scripts/automatizacion/seguridad/security-advisor.py

ai-analyze-performance:
    python scripts/automatizacion/auditoria/performance-profiler.py

ai-refactor:
    python scripts/automatizacion/auditoria/refactor-advisor.py

ai-heal:
    Write-Host "🤖 Ejecutando auto-reparación..." -ForegroundColor Magenta
    python scripts/automatizacion/utilidades/self-healer.py

ai-learn:
    Write-Host "🤖 Aprendiendo de patrones..." -ForegroundColor Magenta
    python scripts/automatizacion/utilidades/learning-logger.py --learn

ai-learn-dashboard:
    python scripts/automatizacion/utilidades/learning-logger.py --dashboard

ai-report:
    Write-Host "🤖 Generando reporte IA..." -ForegroundColor Magenta
    python scripts/automatizacion/auditoria/ai-assistant.py --report

ai-interactive:
    python scripts/automatizacion/auditoria/ai-assistant.py --interactive

# 📊 Ver logs en tiempo real
logs-backend:
    Get-Content logs/backend-(Get-Date -Format "yyyy-MM-dd").log -Wait

logs-frontend:
    Get-Content logs/frontend-(Get-Date -Format "yyyy-MM-dd").log -Wait

logs-all:
    Get-Content logs/sistema-(Get-Date -Format "yyyy-MM-dd").log -Wait