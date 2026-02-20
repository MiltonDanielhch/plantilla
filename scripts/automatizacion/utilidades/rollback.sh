#!/bin/bash
#===============================================================================
# ROLLBACK - Reversión de Versiones
#===============================================================================
# Restaura la versión anterior del proyecto desde backup
# Trigger: Manual cuando hay un bug crítico
#===============================================================================

set -e

# Configuración
BACKUP_DIR="${BACKUP_DIR:-snapshots}"
CURRENT_DIR="$(pwd)"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
ROLLBACK_LOG="${ROLLBACK_LOG:-logs/rollback.log}"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$ROLLBACK_LOG"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}" | tee -a "$ROLLBACK_LOG"
    exit 1
}

# Listar backups disponibles
list_backups() {
    echo "📜 Backups disponibles:"
    echo "====================="
    
    if [ ! -d "$BACKUP_DIR" ]; then
        error "Directorio de backups no existe: $BACKUP_DIR"
    fi
    
    ls -lh "$BACKUP_DIR"/*.tar.gz 2>/dev/null || echo "No hay backups"
}

# Encontrar último backup
find_latest_backup() {
    LATEST=$(ls -t "$BACKUP_DIR"/backup_pre_deploy_*.tar.gz 2>/dev/null | head -1)
    
    if [ -z "$LATEST" ]; then
        error "No se encontró ningún backup en $BACKUP_DIR"
    fi
    
    echo "$LATEST"
}

# Crear backup de seguridad antes de rollback
backup_current() {
    log "📦 Creando backup de seguridad..."
    
    mkdir -p "$BACKUP_DIR"
    
    tar -czf "$BACKUP_DIR/rollback_${TIMESTAMP}.tar.gz" \
        --exclude='node_modules' \
        --exclude='target' \
        --exclude='.git' \
        --exclude='*.log' \
        . 2>/dev/null || true
    
    log "✅ Backup de seguridad: $BACKUP_DIR/rollback_${TIMESTAMP}.tar.gz"
}

# Restaurar desde backup
restore_backup() {
    BACKUP_FILE="$1"
    
    if [ ! -f "$BACKUP_FILE" ]; then
        error "Backup no encontrado: $BACKUP_FILE"
    fi
    
    log "🔄 Restaurando desde: $BACKUP_FILE"
    
    # Extraer
    tar -xzf "$BACKUP_FILE" -C /tmp/rollback_$$
    
    # Copiar archivos (sin sobrescribir .git)
    cp -rf /tmp/rollback_$$/* . 2>/dev/null || true
    cp -rf /tmp/rollback_$$/.* . 2>/dev/null || true
    
    # Limpiar
    rm -rf /tmp/rollback_$$
    
    log "✅ Restauración completada"
}

# Rollback con git
git_rollback() {
    COMMITS="$1"
    
    if [ -d ".git" ]; then
        log "🔄 Rollback con Git ($COMMITS commits)..."
        git reset --hard "HEAD~$COMMITS"
        log "✅ Rollback git completado"
    fi
}

# Main
main() {
    log "${YELLOW}⏪ ROLLBACK - Reversión de Versiones${NC}"
    
    if [ "$1" == "--list" ]; then
        list_backups
        return 0
    fi
    
    if [ "$1" == "--git" ]; then
        COMMITS="${2:-1}"
        log "Git rollback: $COMMITS commits"
        git_rollback "$COMMITS"
        return 0
    fi
    
    if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
        echo "Uso: rollback.sh [comando]"
        echo ""
        echo "Comandos:"
        echo "  (sin args)          - Rollback al último backup"
        echo "  --list              - Listar backups disponibles"
        echo "  --git [n]          - Rollback n commits en git (default: 1)"
        echo ""
        echo "Ejemplos:"
        echo "  ./rollback.sh                    # Restaurar último backup"
        echo "  ./rollback.sh --list            # Ver backups"
        echo "  ./rollback.sh --git 3           # Regresar 3 commits"
        return 0
    fi
    
    # Backup de seguridad actual
    backup_current
    
    # Encontrar y restaurar backup
    LATEST_BACKUP=$(find_latest_backup)
    restore_backup "$LATEST_BACKUP"
    
    log "${GREEN}✅ ROLLBACK COMPLETADO${NC}"
    log "📝 Si hay problemas, usa: $BACKUP_DIR/rollback_${TIMESTAMP}.tar.gz"
    
    return 0
}

main "$@"
