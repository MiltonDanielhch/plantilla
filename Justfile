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

# 🧹 Verificar Calidad de Código (Lint & Format)
check:
    Write-Host "🦀 Verificando Backend..." -ForegroundColor Cyan
    cd backend; cargo fmt; cargo clippy
    Write-Host "🎨 Verificando Frontend..." -ForegroundColor Cyan
    cd frontend; npm run lint
    Write-Host "✅ Todo limpio." -ForegroundColor Green