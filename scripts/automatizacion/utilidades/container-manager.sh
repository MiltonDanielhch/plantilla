#!/bin/bash
#===============================================================================
# CONTAINER-MANAGER - Gestor de Docker
#===============================================================================
# Gestiona contenedores: ver estado, logs, restart, stop, etc.
# Trigger: Manual o monitoreo
#===============================================================================

set -e

COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"
LOG_FILE="logs/container-manager.log"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

case "$1" in
    status)
        echo "📊 Estado de Contenedores:"
        echo "========================"
        docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
        echo ""
        docker-compose -f "$COMPOSE_FILE" ps
        ;;
    
    logs)
        CONTAINER="${2:-app}"
        echo "📜 Logs de $CONTAINER (últimas 100 líneas):"
        docker logs --tail 100 "$CONTAINER"
        ;;
    
    restart)
        CONTAINER="${2:-app}"
        log "Reiniciando $CONTAINER..."
        docker-compose -f "$COMPOSE_FILE" restart "$CONTAINER"
        log "$CONTAINER reiniciado"
        ;;
    
    stop)
        CONTAINER="${2:-app}"
        log "Deteniendo $CONTAINER..."
        docker-compose -f "$COMPOSE_FILE" stop "$CONTAINER"
        log "$CONTAINER detenido"
        ;;
    
    start)
        CONTAINER="${2:-app}"
        log "Iniciando $CONTAINER..."
        docker-compose -f "$COMPOSE_FILE" start "$CONTAINER"
        log "$CONTAINER iniciado"
        ;;
    
    rebuild)
        log "Reconstruyendo contenedores..."
        docker-compose -f "$COMPOSE_FILE" down
        docker-compose -f "$COMPOSE_FILE" build --no-cache
        docker-compose -f "$COMPOSE_FILE" up -d
        log "Contenedores reconstruidos"
        ;;
    
    clean)
        log "Limpiando recursos..."
        docker system prune -f
        docker volume prune -f
        log "Limpieza completada"
        ;;
    
    stats)
        echo "📈 Estadísticas en tiempo real:"
        docker stats --no-stream
        ;;
    
    *)
        echo "Uso: container-manager.sh <comando> [container]"
        echo ""
        echo "Comandos:"
        echo "  status          - Ver estado de contenedores"
        echo "  logs [name]    - Ver logs de un contenedor"
        echo "  restart [name] - Reiniciar contenedor"
        echo "  stop [name]    - Detener contenedor"
        echo "  start [name]   - Iniciar contenedor"
        echo "  rebuild        - Reconstruir desde cero"
        echo "  clean          - Limpiar recursos no usados"
        echo "  stats          - Ver estadísticas"
        exit 1
        ;;
esac
