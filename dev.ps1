# Script de PowerShell para ejecutar Alohopass en Windows
# Compatible con el script dev.sh de Unix

param(
    [string]$Command = "dev"
)

# Colores para output
$Green = "Green"
$Yellow = "Yellow"
$Red = "Red"
$Blue = "Blue"
$Cyan = "Cyan"

# Función para escribir con color
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

# Función para mostrar ayuda
function Show-Help {
    Write-ColorOutput "Alohopass - Gestor de Contraseñas Seguro" $Green
    Write-ColorOutput "Desarrollado por @n3c4s - alohopass.com" $Cyan
    Write-Host ""
    Write-ColorOutput "Uso:" $Blue
    Write-Host "  .\dev.ps1 [comando]"
    Write-Host ""
    Write-ColorOutput "Comandos disponibles:" $Blue
    Write-ColorOutput "  dev        - Ejecutar en modo desarrollo (frontend + backend)" $Yellow
    Write-ColorOutput "  frontend   - Ejecutar solo el frontend" $Yellow
    Write-ColorOutput "  backend    - Ejecutar solo el backend" $Yellow
    Write-ColorOutput "  build      - Construir para producción" $Yellow
    Write-ColorOutput "  install    - Instalar dependencias" $Yellow
    Write-ColorOutput "  clean      - Limpiar archivos de construcción" $Yellow
    Write-ColorOutput "  help       - Mostrar esta ayuda" $Yellow
    Write-Host ""
    Write-ColorOutput "Ejemplos:" $Blue
    Write-Host "  .\dev.ps1 dev        # Ejecutar en modo desarrollo"
    Write-Host "  .\dev.ps1 frontend   # Solo frontend"
    Write-Host "  .\dev.ps1 backend    # Solo backend"
}

# Función para ejecutar en modo desarrollo
function Start-Dev {
    Write-ColorOutput "🚀 Iniciando Alohopass en modo desarrollo..." $Green
    
    # Verificar que estemos en el directorio correcto
    if (-not (Test-Path "Cargo.toml") -or -not (Test-Path "tauri.conf.json")) {
        Write-ColorOutput "Error: No se encontró Cargo.toml o tauri.conf.json" $Red
        Write-ColorOutput "Asegúrate de estar en el directorio raíz del proyecto" $Yellow
        exit 1
    }
    
    # Verificar dependencias
    if (-not (Test-Path "frontend/node_modules")) {
        Write-ColorOutput "Instalando dependencias del frontend..." $Yellow
        Set-Location frontend
        npm install
        Set-Location ..
    }
    
    # Ejecutar Tauri dev
    Write-ColorOutput "Iniciando aplicación Tauri..." $Yellow
    cargo tauri dev
}

# Función para ejecutar solo frontend
function Start-Frontend {
    Write-ColorOutput "🎨 Iniciando solo el frontend..." $Green
    Set-Location frontend
    npm run dev
    Set-Location ..
}

# Función para ejecutar solo backend
function Start-Backend {
    Write-ColorOutput "🔧 Iniciando solo el backend..." $Green
    cargo tauri dev
}

# Función para construir
function Start-Build {
    Write-ColorOutput "🔨 Construyendo Alohopass para producción..." $Green
    
    Write-ColorOutput "Construyendo frontend..." $Yellow
    Set-Location frontend
    npm run build
    Set-Location ..
    
    Write-ColorOutput "Construyendo aplicación Tauri..." $Yellow
    cargo tauri build
    
    Write-ColorOutput "✅ Construcción completada" $Green
}

# Función para instalar dependencias
function Start-Install {
    Write-ColorOutput "📦 Instalando dependencias..." $Green
    
    Write-ColorOutput "Instalando dependencias del frontend..." $Yellow
    Set-Location frontend
    npm install
    Set-Location ..
    
    Write-ColorOutput "Verificando dependencias de Rust..." $Yellow
    cargo check
    
    Write-ColorOutput "✅ Instalación completada" $Green
}

# Función para limpiar
function Start-Clean {
    Write-ColorOutput "🧹 Limpiando archivos de construcción..." $Yellow
    
    # Limpiar Rust
    cargo clean
    
    # Limpiar frontend
    if (Test-Path "frontend/dist") {
        Remove-Item -Recurse -Force "frontend/dist"
    }
    
    if (Test-Path "frontend/node_modules") {
        Remove-Item -Recurse -Force "frontend/node_modules"
    }
    
    Write-ColorOutput "✅ Limpieza completada" $Green
}

# Función principal
function Main {
    Write-ColorOutput "Plataforma detectada: Windows" $Blue
    
    switch ($Command.ToLower()) {
        "dev" {
            Start-Dev
        }
        "frontend" {
            Start-Frontend
        }
        "backend" {
            Start-Backend
        }
        "build" {
            Start-Build
        }
        "install" {
            Start-Install
        }
        "clean" {
            Start-Clean
        }
        "help" {
            Show-Help
        }
        default {
            Write-ColorOutput "Comando desconocido: $Command" $Red
            Write-Host ""
            Show-Help
            exit 1
        }
    }
}

# Ejecutar función principal
Main
