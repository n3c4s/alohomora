#!/bin/bash

# Script de instalación del host nativo para AlohoPass
# Compatible con macOS y Linux

set -e

echo "🔐 AlohoPass: Instalando host nativo para la extensión del navegador..."

# Detectar el sistema operativo
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    CHROME_CONFIG_DIR="$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts"
    FIREFOX_CONFIG_DIR="$HOME/Library/Application Support/Mozilla/NativeMessagingHosts"
    echo "📱 Detectado: macOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    CHROME_CONFIG_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
    FIREFOX_CONFIG_DIR="$HOME/.mozilla/native-messaging-hosts"
    echo "🐧 Detectado: Linux"
else
    echo "❌ Sistema operativo no soportado: $OSTYPE"
    exit 1
fi

# Obtener la ruta del ejecutable de AlohoPass
ALOHOPASS_PATH=$(which alohopass 2>/dev/null || echo "")
if [ -z "$ALOHOPASS_PATH" ]; then
    # Buscar en el directorio actual
    if [ -f "./alohopass" ]; then
        ALOHOPASS_PATH="$(pwd)/alohopass"
    elif [ -f "./target/debug/alohopass" ]; then
        ALOHOPASS_PATH="$(pwd)/target/debug/alohopass"
    elif [ -f "./target/release/alohopass" ]; then
        ALOHOPASS_PATH="$(pwd)/target/release/alohopass"
    else
        echo "❌ No se pudo encontrar el ejecutable de AlohoPass"
        echo "   Asegúrate de haber compilado la aplicación primero con 'cargo build'"
        exit 1
    fi
fi

echo "📍 Ruta de AlohoPass: $ALOHOPASS_PATH"

# Crear directorios si no existen
mkdir -p "$CHROME_CONFIG_DIR"
mkdir -p "$FIREFOX_CONFIG_DIR"

# Crear el archivo de configuración para Chrome
CHROME_CONFIG="$CHROME_CONFIG_DIR/com.alohopass.browser.json"
cat > "$CHROME_CONFIG" << EOF
{
  "name": "com.alohopass.browser",
  "description": "AlohoPass Browser Extension Host",
  "path": "$ALOHOPASS_PATH",
  "type": "stdio",
  "allowed_origins": [
    "chrome-extension://*",
    "moz-extension://*"
  ]
}
EOF

echo "✅ Configuración de Chrome creada en: $CHROME_CONFIG"

# Crear el archivo de configuración para Firefox
FIREFOX_CONFIG="$FIREFOX_CONFIG_DIR/com.alohopass.browser.json"
cat > "$FIREFOX_CONFIG" << EOF
{
  "name": "com.alohopass.browser",
  "description": "AlohoPass Browser Extension Host",
  "path": "$ALOHOPASS_PATH",
  "type": "stdio",
  "allowed_origins": [
    "chrome-extension://*",
    "moz-extension://*"
  ]
}
EOF

echo "✅ Configuración de Firefox creada en: $FIREFOX_CONFIG"

# Hacer el ejecutable ejecutable
chmod +x "$ALOHOPASS_PATH"

echo ""
echo "🎉 ¡Host nativo instalado exitosamente!"
echo ""
echo "📋 Pasos siguientes:"
echo "1. Instala la extensión del navegador desde la carpeta browser-extension"
echo "2. Reinicia tu navegador"
echo "3. La extensión debería conectarse automáticamente a AlohoPass"
echo ""
echo "🔧 Para desinstalar, ejecuta:"
echo "   rm '$CHROME_CONFIG'"
echo "   rm '$FIREFOX_CONFIG'"
echo ""
echo "🔐 AlohoPass - Tu gestor de contraseñas seguro y privado"
