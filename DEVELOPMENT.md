# 🚀 Guía de Desarrollo Multiplataforma - Alohopass

Esta guía te explica cómo ejecutar y desarrollar Alohopass en diferentes plataformas (macOS, Linux y Windows) sin tener que cambiar manualmente la configuración.

## 📋 Prerrequisitos

### Todas las plataformas
- **Rust** (con Cargo) - [Instalar Rust](https://rustup.rs/)
- **Node.js** y **npm** - [Instalar Node.js](https://nodejs.org/)
- **Tauri CLI** - `cargo install tauri-cli --version "^1.5"`

### Windows específico
- **PowerShell** (incluido en Windows 10/11)
- **Git Bash** o **WSL** (opcional, para usar scripts bash)

## 🎯 Métodos para ejecutar la aplicación

### Método 1: Comando directo (Recomendado)

```bash
# En cualquier plataforma
cargo tauri dev
```

**✅ Ventajas:**
- Funciona igual en todas las plataformas
- No requiere scripts adicionales
- Comando estándar de Tauri

**⚠️ Requisito:** La configuración en `tauri.conf.json` debe usar comandos multiplataforma.

### Método 2: Script multiplataforma (Bash)

```bash
# En macOS/Linux
./dev.sh dev        # Modo desarrollo completo
./dev.sh frontend   # Solo frontend
./dev.sh backend    # Solo backend
./dev.sh build      # Construir para producción
./dev.sh install    # Instalar dependencias
./dev.sh clean      # Limpiar archivos
./dev.sh help       # Mostrar ayuda
```

**✅ Ventajas:**
- Detecta automáticamente la plataforma
- Comandos intuitivos
- Manejo automático de dependencias

### Método 3: Script de PowerShell (Windows)

```powershell
# En Windows PowerShell
.\dev.ps1 dev        # Modo desarrollo completo
.\dev.ps1 frontend   # Solo frontend
.\dev.ps1 backend    # Solo backend
.\dev.ps1 build      # Construir para producción
.\dev.ps1 install    # Instalar dependencias
.\dev.ps1 clean      # Limpiar archivos
.\dev.ps1 help       # Mostrar ayuda
```

**✅ Ventajas:**
- Nativo de Windows
- Comandos compatibles con PowerShell
- Manejo automático de dependencias

### Método 4: Makefile multiplataforma

```bash
# En cualquier plataforma
make tauri-dev       # Ejecutar Tauri dev
make dev             # Modo desarrollo completo
make build           # Construir para producción
make install         # Instalar dependencias
make clean           # Limpiar archivos
make help            # Mostrar ayuda
```

**✅ Ventajas:**
- Detecta automáticamente la plataforma
- Comandos estándar de Make
- Integración con herramientas de desarrollo

## 🔧 Configuración automática

### tauri.conf.json
El archivo está configurado para usar comandos multiplataforma:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "frontend/dist"
  }
}
```

**¿Por qué funciona?**
- `npm run dev` funciona en todas las plataformas
- No usa comandos específicos de Windows (`cd frontend && npm run dev`)
- No usa comandos específicos de Unix (`cd frontend; npm run dev`)

### package.json
Scripts multiplataforma en el frontend:

```json
{
  "scripts": {
    "dev": "vite",
    "dev:tauri": "vite",
    "build": "vite build",
    "build:tauri": "vite build"
  }
}
```

## 🌍 Detección automática de plataforma

### Scripts bash (dev.sh)
```bash
detect_platform() {
    case "$(uname -s)" in
        Darwin*)    echo "macos";;
        Linux*)     echo "linux";;
        CYGWIN*|MINGW*|MSYS*) echo "windows";;
        *)          echo "unknown";;
    esac
}
```

### Makefile
```makefile
# Detectar plataforma
ifeq ($(OS),Windows_NT)
    PLATFORM = windows
    SHELL_CMD = powershell -Command
    CD_CMD = cd $(FRONTEND_DIR); npm run dev
else
    PLATFORM = unix
    SHELL_CMD = 
    CD_CMD = cd $(FRONTEND_DIR) && npm run dev
endif
```

## 🚀 Flujo de trabajo recomendado

### 1. Primera vez
```bash
# Instalar dependencias
./dev.sh install        # macOS/Linux
# o
.\dev.ps1 install       # Windows
# o
make install            # Cualquier plataforma
```

### 2. Desarrollo diario
```bash
# Ejecutar en modo desarrollo
cargo tauri dev         # Recomendado
# o
./dev.sh dev           # macOS/Linux
# o
.\dev.ps1 dev          # Windows
# o
make tauri-dev         # Cualquier plataforma
```

### 3. Construcción
```bash
# Construir para producción
cargo tauri build       # Recomendado
# o
./dev.sh build         # macOS/Linux
# o
.\dev.ps1 build        # Windows
# o
make build             # Cualquier plataforma
```

## 🔍 Solución de problemas

### Error: "Port 5173 is already in use"
```bash
# En macOS/Linux
lsof -ti:5173 | xargs kill

# En Windows
netstat -ano | findstr :5173
taskkill /PID <PID> /F
```

### Error: "cd: frontend: No such file or directory"
- Verifica que estés en el directorio raíz del proyecto
- Asegúrate de que exista la carpeta `frontend/`

### Error: "npm: command not found"
- Instala Node.js desde [nodejs.org](https://nodejs.org/)
- Verifica que esté en el PATH del sistema

### Error: "cargo: command not found"
- Instala Rust desde [rustup.rs](https://rustup.rs/)
- Reinicia la terminal después de la instalación

## 📚 Comandos útiles

### Verificar estado del proyecto
```bash
make status              # Cualquier plataforma
./dev.sh help           # macOS/Linux
.\dev.ps1 help          # Windows
```

### Información del sistema
```bash
make info               # Cualquier plataforma
```

### Limpiar y reinstalar
```bash
make clean              # Cualquier plataforma
./dev.sh clean          # macOS/Linux
.\dev.ps1 clean         # Windows
```

## 🎉 ¡Listo!

Ahora puedes desarrollar Alohopass en cualquier plataforma usando los mismos comandos. La aplicación detectará automáticamente tu sistema operativo y usará los comandos apropiados.

**Comando principal recomendado:**
```bash
cargo tauri dev
```

**Scripts adicionales disponibles:**
- `./dev.sh` (macOS/Linux)
- `.\dev.ps1` (Windows)
- `make` (todas las plataformas)

¡Happy coding! 🚀
