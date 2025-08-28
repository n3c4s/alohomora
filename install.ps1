# Script de instalación para Alohopass
# Ejecutar como administrador en PowerShell

Write-Host "Instalando Alohopass - Gestor de Contraseñas Seguro" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green

# Función para verificar si un comando existe
function Test-Command($cmdname) {
    return [bool](Get-Command -Name $cmdname -ErrorAction SilentlyContinue)
}

# Función para verificar versión mínima
function Test-MinVersion($command, $minVersion, $versionArg = "--version") {
    try {
        $output = & $command $versionArg 2>&1
        if ($LASTEXITCODE -eq 0) {
            $version = $output | Select-String -Pattern '\d+\.\d+\.\d+' | Select-Object -First 1
            if ($version) {
                $currentVersion = [Version]($version.Matches[0].Value)
                $minVersionObj = [Version]$minVersion
                return $currentVersion -ge $minVersionObj
            }
        }
        return $false
    }
    catch {
        return $false
    }
}

# Verificar y instalar Rust
Write-Host "`nVerificando Rust..." -ForegroundColor Yellow
if (-not (Test-Command "rustc")) {
    Write-Host "Rust no encontrado. Instalando..." -ForegroundColor Red
    try {
        winget install Rustlang.Rust.MSVC
        Write-Host "Rust instalado correctamente" -ForegroundColor Green
        Write-Host "IMPORTANTE: Reinicia la terminal después de la instalación" -ForegroundColor Yellow
        Write-Host "   Luego ejecuta este script nuevamente" -ForegroundColor Yellow
        exit 1
    }
    catch {
        Write-Host "Error al instalar Rust" -ForegroundColor Red
        Write-Host "Instala manualmente desde: https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
} else {
    $rustVersion = rustc --version
    Write-Host "Rust encontrado: $rustVersion" -ForegroundColor Green
}

# Verificar versión mínima de Rust
if (-not (Test-MinVersion "rustc" "1.70.0")) {
    Write-Host "Versión de Rust muy antigua. Actualiza con: rustup update" -ForegroundColor Red
    exit 1
}

# Verificar y instalar Node.js
Write-Host "`nVerificando Node.js..." -ForegroundColor Yellow
if (-not (Test-Command "node")) {
    Write-Host "Node.js no encontrado. Instalando..." -ForegroundColor Red
    try {
        winget install OpenJS.NodeJS
        Write-Host "Node.js instalado correctamente" -ForegroundColor Green
        Write-Host "IMPORTANTE: Reinicia la terminal después de la instalación" -ForegroundColor Yellow
        Write-Host "   Luego ejecuta este script nuevamente" -ForegroundColor Yellow
        exit 1
    }
    catch {
        Write-Host "Error al instalar Node.js" -ForegroundColor Red
        Write-Host "Instala manualmente desde: https://nodejs.org/" -ForegroundColor Red
        exit 1
    }
} else {
    $nodeVersion = node --version
    $npmVersion = npm --version
    Write-Host "Node.js encontrado: $nodeVersion" -ForegroundColor Green
    Write-Host "npm encontrado: $npmVersion" -ForegroundColor Green
}

# Verificar versión mínima de Node.js
if (-not (Test-MinVersion "node" "18.0.0")) {
    Write-Host "Versión de Node.js muy antigua. Actualiza desde: https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# Verificar versión mínima de npm
if (-not (Test-MinVersion "npm" "8.0.0")) {
    Write-Host "Versión de npm muy antigua. Actualiza con: npm install -g npm@latest" -ForegroundColor Red
    exit 1
}

# Instalar Tauri CLI
Write-Host "`nInstalando Tauri CLI..." -ForegroundColor Yellow
if (-not (Test-Command "cargo")) {
    Write-Host "Cargo no encontrado. Asegúrate de que Rust esté instalado correctamente" -ForegroundColor Red
    exit 1
}

try {
    cargo install tauri-cli
    Write-Host "Tauri CLI instalado correctamente" -ForegroundColor Green
}
catch {
    Write-Host "Error al instalar Tauri CLI" -ForegroundColor Red
    exit 1
}

# Verificar Tauri CLI
if (-not (Test-Command "cargo")) {
    Write-Host "Tauri CLI no se instaló correctamente" -ForegroundColor Red
    exit 1
}

# Instalar dependencias del frontend
Write-Host "`nInstalando dependencias del frontend..." -ForegroundColor Yellow
try {
    Set-Location alohopass/frontend
    npm install
    Write-Host "Dependencias del frontend instaladas" -ForegroundColor Green
    Set-Location ../..
}
catch {
    Write-Host "Error al instalar dependencias del frontend" -ForegroundColor Red
    exit 1
}

# Verificar dependencias de Rust
Write-Host "`nVerificando dependencias de Rust..." -ForegroundColor Yellow
try {
    Set-Location alohopass
    cargo check
    Write-Host "Dependencias de Rust verificadas" -ForegroundColor Green
    Set-Location ..
}
catch {
    Write-Host "Error al verificar dependencias de Rust" -ForegroundColor Red
    Write-Host "Ejecutando cargo build para descargar dependencias..." -ForegroundColor Yellow
    try {
        cargo build
        Write-Host "Dependencias de Rust descargadas" -ForegroundColor Green
    }
    catch {
        Write-Host "Error al descargar dependencias de Rust" -ForegroundColor Red
        exit 1
    }
}

# Crear archivo de configuración del entorno
Write-Host "`nCreando configuración del entorno..." -ForegroundColor Yellow
$envContent = @"
# Configuración de Alohopass
RUST_LOG=info
RUST_BACKTRACE=1
TAURI_PRIVATE_KEY=your_private_key_here
"@

$envContent | Out-File -FilePath "alohopass/.env" -Encoding UTF8
Write-Host "Archivo .env creado" -ForegroundColor Green

# Verificar estructura del proyecto
Write-Host "`nVerificando estructura del proyecto..." -ForegroundColor Yellow
$requiredFiles = @(
    "alohopass/Cargo.toml",
    "alohopass/tauri.conf.json",
    "alohopass/frontend/package.json",
    "alohopass/frontend/vite.config.ts",
    "alohopass/src/main.rs"
)

$missingFiles = @()
foreach ($file in $requiredFiles) {
    if (-not (Test-Path $file)) {
        $missingFiles += $file
    }
}

if ($missingFiles.Count -gt 0) {
    Write-Host "Archivos faltantes:" -ForegroundColor Red
    foreach ($file in $missingFiles) {
        Write-Host "   - $file" -ForegroundColor Red
    }
    exit 1
}

Write-Host "Estructura del proyecto verificada" -ForegroundColor Green

# Mostrar resumen de instalación
Write-Host "`nInstalación completada exitosamente!" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green

Write-Host "`nResumen de la instalación:" -ForegroundColor Cyan
Write-Host "Rust: $(rustc --version)" -ForegroundColor Green
Write-Host "Node.js: $(node --version)" -ForegroundColor Green
Write-Host "npm: $(npm --version)" -ForegroundColor Green
Write-Host "Tauri CLI: Instalado" -ForegroundColor Green
Write-Host "Dependencias del frontend: Instaladas" -ForegroundColor Green
Write-Host "Dependencias de Rust: Verificadas" -ForegroundColor Green

Write-Host "`nPara ejecutar Alohopass:" -ForegroundColor Cyan
Write-Host "1. Modo desarrollo:" -ForegroundColor White
Write-Host "   Terminal 1: cd alohopass/frontend && npm run dev" -ForegroundColor Gray
Write-Host "   Terminal 2: cd alohopass && cargo tauri dev" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Modo producción:" -ForegroundColor White
Write-Host "   cd alohopass && cargo tauri build" -ForegroundColor Gray

Write-Host "`nCaracterísticas de seguridad implementadas:" -ForegroundColor Cyan
Write-Host "Encriptación AES-256-GCM" -ForegroundColor Green
Write-Host "Derivación de claves Argon2" -ForegroundColor Green
Write-Host "Base de datos SQLite encriptada" -ForegroundColor Green
Write-Host "Generador de contraseñas seguras" -ForegroundColor Green
Write-Host "Autocompletado en navegadores" -ForegroundColor Green
Write-Host "Verificación de fortaleza de contraseñas" -ForegroundColor Green

Write-Host "`nDocumentación:" -ForegroundColor Cyan
Write-Host "README.md - Guía completa del proyecto" -ForegroundColor White
Write-Host "Makefile - Comandos útiles de desarrollo" -ForegroundColor White
Write-Host "tauri.conf.json - Configuración de Tauri" -ForegroundColor White

Write-Host "`nPróximos pasos:" -ForegroundColor Cyan
Write-Host "1. Ejecuta 'cargo tauri dev' para iniciar en modo desarrollo" -ForegroundColor White
Write-Host "2. Crea tu primera contraseña maestra" -ForegroundColor White
Write-Host "3. Comienza a usar Alohopass!" -ForegroundColor White

Write-Host "`nDisfruta usando tu gestor de contraseñas seguro!" -ForegroundColor Green
Write-Host "   Inspirado en el encantamiento Alohomora de Harry Potter" -ForegroundColor Magenta 