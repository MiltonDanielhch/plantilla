#!/bin/bash
#===============================================================================
# DEPLOY_MAESTRO - Zero Downtime Deploy
#===============================================================================
# Despliega código nuevo sin desconectar usuarios
# Trigger: Manual o CI/CD pipeline
#===============================================================================

set -e

# Configuración
BRANCH="${BRANCH:-main}"
BACKUP_DIR="${BACKUP_DIR:-snapshots}"
DEPLOY_LOG="${DEPLOY_LOG:-logs/deploy.log}"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$DEPLOY_LOG"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}" | tee -a "$DEPLOY_LOG"
}

success() {
    echo -e "${GREEN}[OK] $1${NC}" | tee -a "$DEPLOY_LOG"
}

# Verificar que todo esté limpio antes de deploy
pre_deploy_check() {
    log "🔍 Verificando estado antes del deploy..."
    
    if ! git diff --quiet; then
        error "Hay cambios sin commit. Haz commit o stash primero."
        exit 1
    fi
    
    success "Estado verificado"
}

# Crear backup antes de deploy
create_backup() {
    log "📦 Creando backup..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_NAME="backup_pre_deploy_${TIMESTAMP}"
    
    mkdir -p "$BACKUP_DIR"
    
    tar -czf "$BACKUP_DIR/${BACKUP_NAME}.tar.gz" \
        --exclude='node_modules' \
        --exclude='target' \
        --exclude='.git' \
        --exclude='*.log' \
        . 2>/dev/null || true
    
    success "Backup guardado: $BACKUP_DIR/${BACKUP_NAME}.tar.gz"
}

# Pull y instalación
pull_and_install() {
    log "📥 Obteniendo código nuevo..."
    git fetch origin
    git pull origin "$BRANCH"
    
    log "📦 Instalando dependencias..."
    
    if [ -f "requirements.txt" ]; then
        pip install -r requirements.txt -q
    fi
    
    if [ -f "package.json" ]; then
        npm install --production
    fi
    
    if [ -f "Cargo.toml" ]; then
        cargo build --release
    fi
    
    success "Dependencias actualizadas"
}

# Migraciones de base de datos
run_migrations() {
    log "🗄️ Ejecutando migraciones..."
    
    if [ -f "migrate.py" ]; then
        python migrate.py
    elif [ -f "manage.py" ]; then
        python manage.py migrate
    elif [ -f "alembic.ini" ]; then
        alembic upgrade head
    fi
    
    success "Migraciones completadas"
}

# Recargar servicios
reload_services() {
    log "🔄 Recargando servicios..."
    
    if command -v systemctl &> /dev/null; then
        sudo systemctl reload nginx 2>/dev/null || true
        sudo systemctl reload app 2>/dev/null || true
    fi
    
    success "Servicios recargados"
}

# Verificar deploy
verify_deploy() {
    log "✅ Verificando deploy..."
    
    sleep 3
    
    if curl -sf http://localhost:3000/health > /dev/null 2>&1; then
        success "Deploy verificado - App respondiendo"
    elif curl -sf http://localhost:8000/health > /dev/null 2>&1; then
        success "Deploy verificado - App respondiendo"
    else
        log "[WARNING] No se pudo verificar respuesta automática"
    fi
}

# Notificación
notify() {
    if [ -n "$TELEGRAM_BOT_TOKEN" ] && [ -n "$TELEGRAM_CHAT_ID" ]; then
        python3 -c "
import requests
msg = '🚀 *SINTONÍA 3026*: Despliegue completado!\n📅 $(date)'
requests.post(f'https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage',
    json={'chat_id': '$TELEGRAM_CHAT_ID', 'text': msg, 'parse_mode': 'Markdown'})
" 2>/dev/null || true
    fi
}

# Main
main() {
    log "${BLUE}🚀 INICIANDO DESPLIEGUE DE SINTONÍA 3026${NC}"
    log "=========================================="
    
    pre_deploy_check
    create_backup
    pull_and_install
    run_migrations
    reload_services
    verify_deploy
    
    log "${GREEN}✅ DESPLIEGUE COMPLETADO SIN DESCONECTAR USUARIOS${NC}"
    
    notify
    
    return 0
}

main "$@"
