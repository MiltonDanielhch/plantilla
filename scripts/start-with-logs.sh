#!/bin/bash

# Script de inicio con logs para Sintonía 3026
# Uso: ./scripts/start-with-logs.sh

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Iniciando Sintonía 3026 con logging...${NC}"

# Crear directorio de logs
mkdir -p logs
echo -e "${GREEN}📁 Directorio logs/ creado${NC}"

# Fecha para los archivos
DATE=$(date +%Y-%m-%d)
TIME=$(date +%H-%M-%S)

# Archivos de log
BACKEND_LOG="logs/backend-${DATE}.log"
FRONTEND_LOG="logs/frontend-${DATE}.log"
COMBINED_LOG="logs/sistema-${DATE}.log"

echo -e "${YELLOW}📝 Logs del día:${NC}"
echo "   Backend:  $BACKEND_LOG"
echo "   Frontend: $FRONTEND_LOG"
echo "   Combined: $COMBINED_LOG"
echo ""

# Función para limpiar procesos al salir
cleanup() {
    echo -e "\n${RED}🛑 Deteniendo servicios...${NC}"
    if [ -n "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null || true
        echo "   Backend detenido (PID: $BACKEND_PID)"
    fi
    if [ -n "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null || true
        echo "   Frontend detenido (PID: $FRONTEND_PID)"
    fi
    echo -e "${GREEN}✅ Servicios detenidos. Logs guardados en logs/${NC}"
    exit 0
}

# Capturar señales de salida
trap cleanup INT TERM EXIT

# Iniciar Backend
echo -e "${BLUE}🔧 Iniciando Backend...${NC}"
cd backend
cargo run 2>&1 | tee "$BACKEND_LOG" &
BACKEND_PID=$!
echo -e "${GREEN}   Backend iniciado (PID: $BACKEND_PID)${NC}"
echo ""

# Esperar a que el backend esté listo
echo -e "${YELLOW}⏳ Esperando backend...${NC}"
sleep 3

# Iniciar Frontend
echo -e "${BLUE}🎨 Iniciando Frontend...${NC}"
cd ../frontend
npm run dev 2>&1 | tee "$FRONTEND_LOG" &
FRONTEND_PID=$!
echo -e "${GREEN}   Frontend iniciado (PID: $FRONTEND_PID)${NC}"
echo ""

# Crear log combinado (tail de ambos)
(tail -f "$BACKEND_LOG" "$FRONTEND_LOG" > "$COMBINED_LOG" 2>/dev/null) &
TAIL_PID=$!

echo -e "${GREEN}✅ Sintonía 3026 completamente activa!${NC}"
echo ""
echo -e "${BLUE}📊 Accesos:${NC}"
echo "   Frontend: http://localhost:4321"
echo "   Backend:  http://localhost:3000"
echo "   API Docs: http://localhost:3000/swagger-ui"
echo ""
echo -e "${YELLOW}💡 Comandos útiles:${NC}"
echo "   tail -f logs/backend-${DATE}.log    # Ver logs backend"
echo "   tail -f logs/frontend-${DATE}.log   # Ver logs frontend"
echo "   tail -f logs/sistema-${DATE}.log    # Ver logs combinados"
echo ""
echo -e "${RED}⚠️  Presiona Ctrl+C para detener todos los servicios${NC}"
echo ""

# Mantener script corriendo
wait
