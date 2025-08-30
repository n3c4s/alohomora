#!/bin/bash

# Script para conectar la extensión del navegador con la aplicación Tauri
# Este script actúa como un proxy entre la extensión y el servidor TCP

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Puerto donde está escuchando la aplicación Tauri (se lee dinámicamente)
PORT_FILE="$PROJECT_DIR/.alohopass_port"
TAURI_HOST="127.0.0.1"

# Función para obtener el puerto
get_port() {
    if [[ -f "$PORT_FILE" ]]; then
        cat "$PORT_FILE"
    else
        echo "12345"  # Puerto por defecto
    fi
}

# Función para conectar al servidor TCP de Tauri
connect_to_tauri() {
    local port=$(get_port)
    
    echo "🔌 AlohoPass: Conectando a $TAURI_HOST:$port" >&2
    
    # Intentar conectar al puerto TCP
    if ! nc -z "$TAURI_HOST" "$port" 2>/dev/null; then
        echo "Error: No se puede conectar a $TAURI_HOST:$port" >&2
        echo "Asegúrate de que la aplicación Tauri esté ejecutándose" >&2
        echo "Puerto leído desde: $PORT_FILE" >&2
        exit 1
    fi

    echo "🔌 AlohoPass: Conectado exitosamente a $TAURI_HOST:$port" >&2
    
    # Conectar al puerto TCP y reenviar stdin/stdout
    exec nc "$TAURI_HOST" "$port"
}

# Función para mostrar ayuda
show_help() {
    echo "AlohoPass Browser Extension Connector"
    echo ""
    echo "Uso: $0 [opciones]"
    echo ""
    echo "Opciones:"
    echo "  -h, --help     Mostrar esta ayuda"
    echo "  -p, --port     Puerto de conexión (default: leído de .alohopass_port)"
    echo "  -H, --host     Host de conexión (default: $TAURI_HOST)"
    echo ""
    echo "Este script conecta la extensión del navegador con la aplicación Tauri"
    echo "a través del puerto TCP $TAURI_HOST:$(get_port)"
    echo ""
    echo "El puerto se lee automáticamente del archivo: $PORT_FILE"
}

# Procesar argumentos de línea de comandos
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -p|--port)
            # Sobrescribir el puerto del archivo
            echo "$2" > "$PORT_FILE"
            shift 2
            ;;
        -H|--host)
            TAURI_HOST="$2"
            shift 2
            ;;
        *)
            echo "Error: Opción desconocida $1" >&2
            show_help
            exit 1
            ;;
    esac
done

# Verificar que netcat esté disponible
if ! command -v nc &> /dev/null; then
    echo "Error: netcat (nc) no está instalado" >&2
    echo "Instala netcat para continuar:" >&2
    echo "  macOS: brew install netcat" >&2
    echo "  Ubuntu/Debian: sudo apt-get install netcat" >&2
    exit 1
fi

# Mostrar información de conexión
echo "🔌 AlohoPass: Iniciando conector de extensión" >&2
echo "🔌 AlohoPass: Puerto leído: $(get_port)" >&2
echo "🔌 AlohoPass: Host: $TAURI_HOST" >&2

# Intentar conectar
connect_to_tauri
