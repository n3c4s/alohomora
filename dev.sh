#!/bin/bash

# Script multiplataforma para ejecutar Alohopass
# Funciona en macOS, Linux y Windows (con Git Bash/WSL)

set -e

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para detectar plataforma
detect_platform() {
    case "$(uname -s)" in
        Darwin*)    echo "macos";;
        Linux*)     echo "linux";;
        CYGWIN*|MINGW*|MSYS*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Función para ejecutar comando según plataforma
run_command() {
    local platform=$(detect_platform)
    local command="$1"
    
    echo -e "${BLUE}Plataforma detectada: ${YELLOW}${platform}${NC}"
    
    case "$platform" in
        "macos"|"linux")
            eval "$command"
            ;;
        "windows")
            # En Windows, usar PowerShell si está disponible
            if command -v powershell >/dev/null 2>&1; then
                powershell -Command "$command"
            else
                # Fallback a cmd
                cmd //c "$command"
            fi
            ;;
        *)
            echo -e "${RED}Plataforma no soportada${NC}"
            exit 1
            ;;
    esac
}

# Función para mostrar ayuda
show_help() {
    echo -e "${GREEN}Alohopass - Gestor de Contraseñas Seguro${NC}"
    echo -e "${CYAN}Desarrollado por @n3c4s - ${YELLOW}alohopass.com${NC}"
    echo ""
    echo -e "${BLUE}Uso:${NC}"
    echo "  ./dev.sh [comando]"
    echo ""
    echo -e "${BLUE}Comandos disponibles:${NC}"
    echo -e "  ${YELLOW}dev${NC}        - Ejecutar en modo desarrollo (frontend + backend)"
    echo -e "  ${YELLOW}frontend${NC}   - Ejecutar solo el frontend"
    echo -e "  ${YELLOW}backend${NC}    - Ejecutar solo el backend"
    echo -e "  ${YELLOW}build${NC}      - Construir para producción"
    echo -e "  ${YELLOW}install${NC}    - Instalar dependencias"
    echo -e "  ${YELLOW}clean${NC}      - Limpiar archivos de construcción"
    echo -e "  ${YELLOW}help${NC}       - Mostrar esta ayuda"
    echo ""
    echo -e "${BLUE}Ejemplos:${NC}"
    echo "  ./dev.sh dev        # Ejecutar en modo desarrollo"
    echo "  ./dev.sh frontend   # Solo frontend"
    echo "  ./dev.sh backend    # Solo backend"
}

# Función para ejecutar en modo desarrollo
run_dev() {
    echo -e "${GREEN}🚀 Iniciando Alohopass en modo desarrollo...${NC}"
    
    # Verificar que estemos en el directorio correcto
    if [ ! -f "Cargo.toml" ] || [ ! -f "tauri.conf.json" ]; then
        echo -e "${RED}Error: No se encontró Cargo.toml o tauri.conf.json${NC}"
        echo -e "${YELLOW}Asegúrate de estar en el directorio raíz del proyecto${NC}"
        exit 1
    fi
    
    # Verificar dependencias
    if [ ! -d "frontend/node_modules" ]; then
        echo -e "${YELLOW}Instalando dependencias del frontend...${NC}"
        run_command "cd frontend && npm install"
    fi
    
    # Ejecutar Tauri dev
    echo -e "${YELLOW}Iniciando aplicación Tauri...${NC}"
    cargo tauri dev
}

# Función para ejecutar solo frontend
run_frontend() {
    echo -e "${GREEN}🎨 Iniciando solo el frontend...${NC}"
    run_command "cd frontend && npm run dev"
}

# Función para ejecutar solo backend
run_backend() {
    echo -e "${GREEN}🔧 Iniciando solo el backend...${NC}"
    cargo tauri dev
}

# Función para construir
run_build() {
    echo -e "${GREEN}🔨 Construyendo Alohopass para producción...${NC}"
    
    echo -e "${YELLOW}Construyendo frontend...${NC}"
    run_command "cd frontend && npm run build"
    
    echo -e "${YELLOW}Construyendo aplicación Tauri...${NC}"
    cargo tauri build
    
    echo -e "${GREEN}✅ Construcción completada${NC}"
}

# Función para instalar dependencias
run_install() {
    echo -e "${GREEN}📦 Instalando dependencias...${NC}"
    
    echo -e "${YELLOW}Instalando dependencias del frontend...${NC}"
    run_command "cd frontend && npm install"
    
    echo -e "${YELLOW}Verificando dependencias de Rust...${NC}"
    cargo check
    
    echo -e "${GREEN}✅ Instalación completada${NC}"
}

# Función para limpiar
run_clean() {
    echo -e "${YELLOW}🧹 Limpiando archivos de construcción...${NC}"
    
    # Limpiar Rust
    cargo clean
    
    # Limpiar frontend
    if [ -d "frontend/dist" ]; then
        rm -rf frontend/dist
    fi
    
    if [ -d "frontend/node_modules" ]; then
        rm -rf frontend/node_modules
    fi
    
    echo -e "${GREEN}✅ Limpieza completada${NC}"
}

# Función principal
main() {
    local command="${1:-dev}"
    
    case "$command" in
        "dev")
            run_dev
            ;;
        "frontend")
            run_frontend
            ;;
        "backend")
            run_backend
            ;;
        "build")
            run_build
            ;;
        "install")
            run_install
            ;;
        "clean")
            run_clean
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            echo -e "${RED}Comando desconocido: $command${NC}"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Ejecutar función principal
main "$@"
