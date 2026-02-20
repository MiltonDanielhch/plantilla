#!/bin/bash
#===============================================================================
# VIGILANTE - Auto-Restart Script
#===============================================================================
# Revisa si el proceso de la app está corriendo y lo reinicia si no.
# Trigger: Cron cada minuto (* * * * * /path/to/vigilante.sh)
#===============================================================================

set -e

# Configuración
APP_NAME="${APP_NAME:-mi_app_principal}"
APP_COMMAND="${APP_COMMAND:-cargo run}"
APP_DIR="${APP_DIR:-.}"
LOG_FILE="${LOG_FILE:-logs/vigilante.log}"
MAX_RESTARTS="${MAX_RESTARTS:-5}"
RESTART_COOLDOWN="${RESTART_COOLDOWN:-60}"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

check_process() {
    if pgrep -f "$APP_NAME" > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

restart_app() {
    log "${YELLOW}⚠️ La app cayó. Reiniciando...${NC}"
    
    cd "$APP_DIR"
    
    # Contador de reinicios
    RESTART_COUNT_FILE="/tmp/${APP_NAME}_restarts"
    CURRENT_RESTARTS=0
    
    if [ -f "$RESTART_COUNT_FILE" ]; then
        CURRENT_RESTARTS=$(cat "$RESTART_COUNT_FILE")
    fi
    
    # Verificar cooldown
    LAST_RESTART_FILE="/tmp/${APP_NAME}_last_restart"
    if [ -f "$LAST_RESTART_FILE" ]; then
        LAST_RESTART=$(cat "$LAST_RESTART_FILE")
        NOW=$(date +%s)
        if [ $((NOW - LAST_RESTART)) -lt "$RESTART_COOLDOWN" ]; then
            log "${RED}⏳ En cooldown. Esperando...${NC}"
            return 1
        fi
    fi
    
    # Incrementar contador
    CURRENT_RESTARTS=$((CURRENT_RESTARTS + 1))
    echo "$CURRENT_RESTARTS" > "$RESTART_COUNT_FILE"
    echo "$(date +%s)" > "$LAST_RESTART_FILE"
    
    if [ "$CURRENT_RESTARTS" -gt "$MAX_RESTARTS" ]; then
        log "${RED}🚨 Demasiados reinicios ($CURRENT_RESTARTS/$MAX_RESTARTS). Deteniendo.${NC}"
        echo "0" > "$RESTART_COUNT_FILE"
        return 1
    fi
    
    # Reiniciar
    log "🔄 Reinicio #$CURRENT_RESTARTS"
    nohup $APP_COMMAND > /dev/null 2>&1 &
    
    sleep 5
    
    if check_process; then
        log "${GREEN}✅ App reiniciada exitosamente${NC}"
        echo "0" > "$RESTART_COUNT_FILE"
        
        # Notificar por Telegram si está configurado
        if [ -n "$TELEGRAM_BOT_TOKEN" ] && [ -n "$TELEGRAM_CHAT_ID" ]; then
            python3 -c "
import requests
msg = '🔄 *SINTONÍA 3026*: La app *$APP_NAME* fue reiniciada automáticamente.'
requests.post(f'https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage',
    json={'chat_id': '$TELEGRAM_CHAT_ID', 'text': msg, 'parse_mode': 'Markdown'})
"
        fi
    else
        log "${RED}❌ Fallo al reiniciar${NC}"
    fi
}

# Main
main() {
    log "👁️ Vigilante revisando: $APP_NAME"
    
    if ! check_process; then
        restart_app
    else
        log "✅ App corriendo normalmente"
    fi
}

main "$@"
