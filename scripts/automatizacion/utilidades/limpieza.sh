#!/bin/bash
#===============================================================================
# LIMPIEZA - Log Cleaner Script
#===============================================================================
# Borra archivos de log más viejos de X días
# Trigger: Cron (0 3 * * 0 para domingos a las 3AM)
#===============================================================================

set -e

# Configuración
LOG_DIR="${LOG_DIR:-logs}"
DAYS_TO_KEEP="${DAYS_TO_KEEP:-7}"
LOG_FILE="${LOG_FILE:-logs/limpieza.log}"

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

main() {
    log "${YELLOW}🧹 Iniciando limpieza de logs...${NC}"
    
    if [ ! -d "$LOG_DIR" ]; then
        log "📁 Directorio de logs no existe: $LOG_DIR"
        exit 0
    fi
    
    # Contar archivos antes
    FILES_BEFORE=$(find "$LOG_DIR" -type f -name "*.log" 2>/dev/null | wc -l)
    SIZE_BEFORE=$(du -sh "$LOG_DIR" 2>/dev/null | cut -f1)
    
    log "📊 Logs antes: $FILES_BEFORE archivos ($SIZE_BEFORE)"
    
    # Eliminar logs viejos
    DELETED=$(find "$LOG_DIR" -type f -name "*.log" -mtime +$DAYS_TO_KEEP -delete -print 2>/dev/null | wc -l)
    
    # Contar después
    FILES_AFTER=$(find "$LOG_DIR" -type f -name "*.log" 2>/dev/null | wc -l)
    SIZE_AFTER=$(du -sh "$LOG_DIR" 2>/dev/null | cut -f1)
    
    log "🗑️  Archivos eliminados: $DELETED"
    log "📊 Logs después: $FILES_AFTER archivos ($SIZE_AFTER)"
    log "${GREEN}✅ Limpieza completada${NC}"
}

main "$@"
