#!/bin/bash
#===============================================================================
# BUILD-PACKAGE - Compilación y Empaquetado
#===============================================================================
# Compila código, minifica y crea contenedores Docker
# Trigger: CI/CD o manual
#===============================================================================

set -e

PROJECT_NAME="${PROJECT_NAME:-sintonia3026}"
VERSION="${VERSION:-1.0.0}"
DOCKER_REGISTRY="${DOCKER_REGISTRY:-}"
BUILD_DIR="${BUILD_DIR:-dist}"
LOG_FILE="${LOG_FILE:-logs/build.log}"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Limpiar build anterior
clean() {
    log "${YELLOW}🧹 Limpiando build anterior...${NC}"
    rm -rf "$BUILD_DIR" 2>/dev/null || true
    rm -rf dist/ 2>/dev/null || true
    log "[OK] Limpieza completada"
}

# Build Python
build_python() {
    log "${BLUE}🐍 Build Python...${NC}"
    
    if [ -f "pyproject.toml" ]; then
        pip install build -q
        python -m build
    elif [ -f "setup.py" ]; then
        python setup.py sdist bdist_wheel
    fi
    
    log "[OK] Python build completado"
}

# Build Node.js
build_node() {
    log "${BLUE}🟢 Build Node.js...${NC}"
    
    if [ -f "package.json" ]; then
        npm run build
        
        if [ -f "tsconfig.json" ]; then
            npx tsc --noEmit
        fi
    fi
    
    log "[OK] Node.js build completado"
}

# Build Rust
build_rust() {
    log "${BLUE}🦀 Build Rust...${NC}"
    
    if [ -f "Cargo.toml" ]; then
        cargo build --release
    fi
    
    log "[OK] Rust build completado"
}

# Minificar
minify() {
    log "${BLUE}📦 Minificando...${NC}"
    
    if [ -d "frontend/dist" ]; then
        if command -v terser &> /dev/null; then
            find frontend/dist -name "*.js" -exec terser {} -o {} \;
        fi
        
        if command -v cssnano &> /dev/null; then
            find frontend/dist -name "*.css" -exec cssnano {} {} \;
        fi
    fi
    
    log "[OK] Minificación completada"
}

# Build Docker
docker_build() {
    log "${BLUE}🐳 Build Docker...${NC}"
    
    if [ -f "Dockerfile" ]; then
        IMAGE_NAME="${DOCKER_REGISTRY}${PROJECT_NAME}:${VERSION}"
        
        docker build -t "$IMAGE_NAME" .
        docker build -t "${PROJECT_NAME}:latest" .
        
        log "[OK] Docker build: $IMAGE_NAME"
        
        echo "$IMAGE_NAME" > "$BUILD_DIR/docker_image.txt"
    else
        log "[WARN] No se encontró Dockerfile"
    fi
}

# Package todo
package() {
    log "${YELLOW}📦 Empaquetando...${NC}"
    
    mkdir -p "$BUILD_DIR"
    
    tar -czf "$BUILD_DIR/${PROJECT_NAME}_${VERSION}.tar.gz" \
        --exclude='node_modules' \
        --exclude='target' \
        --exclude='.git' \
        --exclude='*.log' \
        . 2>/dev/null || true
    
    if [ -d "frontend/dist" ]; then
        cp -r frontend/dist "$BUILD_DIR/"
    fi
    
    if [ -d "backend/target/release" ]; then
        cp -r backend/target/release/* "$BUILD_DIR/"
    fi
    
    log "[OK] Empaquetado completado"
}

# Main
main() {
    log "${BLUE}🚀 BUILD-PACKAGE - Sintonía 3026${NC}"
    log "=========================================="
    
    clean
    build_python
    build_node
    build_rust
    minify
    docker_build
    package
    
    log "${GREEN}✅ BUILD COMPLETADO${NC}"
    log "   Versión: $VERSION"
    log "   Directorio: $BUILD_DIR"
    
    return 0
}

main "$@"
