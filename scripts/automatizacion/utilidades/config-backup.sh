#!/bin/bash
#===============================================================================
# CONFIG-BACKUP - Backup de Configuraciones
#===============================================================================
# Hace backup de archivos de configuración importantes
# Trigger: Antes de cambios o manual
#===============================================================================

set -e

BACKUP_DIR="${BACKUP_DIR:-backups/configs}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

CONFIG_FILES=(
    ".env"
    ".env.production"
    "docker-compose.yml"
    "docker-compose.production.yml"
    "nginx.conf"
    "nginx/default.conf"
    "Caddyfile"
    "justfile"
    "Justfile"
    "tailwind.config.js"
    "postcss.config.js"
    "tsconfig.json"
    ".github/workflows/*.yml"
)

echo "[CONFIG-BACKUP] Backup de Configuraciones"
echo "========================================"

mkdir -p "$BACKUP_DIR"

BACKUP_NAME="config_backup_${TIMESTAMP}.tar.gz"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}"

echo "[INFO] Archivos a respaldar:"

TEMP_DIR="/tmp/config_backup_$$"
mkdir -p "$TEMP_DIR"

for file in "${CONFIG_FILES[@]}"; do
    if ls $file 1> /dev/null 2>&1; then
        echo "   - $file"
        cp -r $file "$TEMP_DIR/" 2>/dev/null || true
    fi
done

tar -czf "$BACKUP_PATH" -C "$TEMP_DIR" .
rm -rf "$TEMP_DIR"

SIZE=$(du -h "$BACKUP_PATH" | cut -f1)

echo ""
echo "[OK] Backup guardado: $BACKUP_PATH"
echo "[OK] Tamaño: $SIZE"

if [ -d ".git" ]; then
    echo "[OK] git status:"
    git status --short
fi

echo ""
echo "[INFO] Para restaurar:"
echo "   tar -xzf $BACKUP_PATH"
