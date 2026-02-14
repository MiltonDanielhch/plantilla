#!/bin/bash
# setup_server.sh - Script de Aprovisionamiento para Ubuntu/Debian

set -e # Detener si hay error

echo "🚀 Iniciando Protocolo de Despliegue Sintonía 3026..."

# 1. Actualizar Sistema
echo "📦 Actualizando paquetes..."
sudo apt-get update && sudo apt-get upgrade -y

# 2. Instalar Docker (si no existe)
if ! command -v docker &> /dev/null
then
    echo "🐳 Instalando Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    echo "⚠️ Docker instalado. Es posible que necesites reiniciar la sesión SSH."
else
    echo "✅ Docker ya está instalado."
fi

# 3. Lanzar Producción
# Asumimos que ya estamos dentro de la carpeta del repo clonado
if [ -d "infra/prod" ]; then
    echo "🔥 Desplegando contenedores..."
    cd infra/prod
    docker compose up -d --build
    echo "✅ ¡Despliegue Completado!"
    echo "🌍 Tu aplicación debería estar visible en el puerto 80/443."
else
    echo "❌ Error: No se encuentra la carpeta infra/prod. Asegúrate de estar en la raíz del proyecto."
    exit 1
fi