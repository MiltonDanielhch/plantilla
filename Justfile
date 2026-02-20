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

# 🛡️ Escudo de Seguridad (Shield)
shield:
    Write-Host "🛡️ Escaneando secretos y API Keys..." -ForegroundColor Yellow
    python infra/scripts/shield.py

# 📝 Iniciar con Logs (Backend + Frontend con logging a archivo)
run-with-logs:
    Write-Host "🚀 Iniciando Sintonía 3026 con logging..." -ForegroundColor Green
    Write-Host "📁 Logs se guardarán en: logs/" -ForegroundColor Cyan
    bash scripts/start-with-logs.sh

# 📊 Ver logs en tiempo real
logs-backend:
    Get-Content logs/backend-(Get-Date -Format "yyyy-MM-dd").log -Wait

logs-frontend:
    Get-Content logs/frontend-(Get-Date -Format "yyyy-MM-dd").log -Wait

logs-all:
    Get-Content logs/sistema-(Get-Date -Format "yyyy-MM-dd").log -Wait