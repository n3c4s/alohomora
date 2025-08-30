# Makefile para Alohopass
# Gestor de contraseñas seguro inspirado en Alohomora

.PHONY: help install dev build clean test format lint check docker-build docker-run setup update docs release install-dev

# Variables
APP_NAME = alohopass
FRONTEND_DIR = frontend
BACKEND_DIR = src
BUILD_DIR = target
DIST_DIR = dist

# Detectar plataforma
ifeq ($(OS),Windows_NT)
    PLATFORM = windows
    SHELL_CMD = powershell -Command
    CD_CMD = cd $(FRONTEND_DIR); npm run dev
    BUILD_CMD = cd $(FRONTEND_DIR); npm run build
else
    PLATFORM = unix
    SHELL_CMD = 
    CD_CMD = cd $(FRONTEND_DIR) && npm run dev
    BUILD_CMD = cd $(FRONTEND_DIR) && npm run build
endif

# Colores para output
GREEN = \033[0;32m
YELLOW = \033[1;33m
RED = \033[0;31m
BLUE = \033[0;34m
CYAN = \033[0;36m
NC = \033[0m # No Color

help: ## Mostrar esta ayuda
	@echo "$(GREEN)Alohopass - Gestor de Contraseñas Seguro$(NC)"
	@echo "$(CYAN)Desarrollado por @n3c4s - $(NC)$(YELLOW)alohopass.com$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(BLUE)Comandos disponibles:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Instalar dependencias y configurar el proyecto
	@echo "$(GREEN)Instalando Alohopass...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		echo "$(YELLOW)Ejecutando script de instalación de PowerShell...$(NC)"; \
		powershell -ExecutionPolicy Bypass -File install.ps1; \
	else \
		echo "$(YELLOW)Instalando dependencias del frontend...$(NC)"; \
		cd $(FRONTEND_DIR) && npm install; \
		echo "$(YELLOW)Verificando dependencias de Rust...$(NC)"; \
		cargo check; \
	fi
	@echo "$(GREEN)✅ Instalación completada$(NC)"

dev: ## Ejecutar en modo desarrollo
	@echo "$(GREEN)🚀 Iniciando Alohopass en modo desarrollo...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Terminal 1: Frontend (npm run dev)$(NC)"
	@echo "$(YELLOW)Terminal 2: Backend (cargo tauri dev)$(NC)"
	@echo "$(BLUE)Abriendo frontend...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "$(CD_CMD)"; \
	else \
		$(CD_CMD); \
	fi

dev-backend: ## Ejecutar solo el backend en modo desarrollo
	@echo "$(GREEN)🔧 Iniciando backend de Alohopass...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	cargo tauri dev

dev-frontend: ## Ejecutar solo el frontend en modo desarrollo
	@echo "$(GREEN)🎨 Iniciando frontend de Alohopass...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "$(CD_CMD)"; \
	else \
		$(CD_CMD); \
	fi

build: ## Construir para producción
	@echo "$(GREEN)🔨 Construyendo Alohopass para producción...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Construyendo frontend...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "$(BUILD_CMD)"; \
	else \
		$(BUILD_CMD); \
	fi
	@echo "$(YELLOW)Construyendo aplicación Tauri...$(NC)"
	cargo tauri build
	@echo "$(GREEN)✅ Construcción completada$(NC)"

build-frontend: ## Construir solo el frontend
	@echo "$(YELLOW)Construyendo frontend...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "$(BUILD_CMD)"; \
	else \
		$(BUILD_CMD); \
	fi

build-backend: ## Construir solo el backend
	@echo "$(YELLOW)Construyendo backend...$(NC)"
	cargo build --release

clean: ## Limpiar archivos de construcción
	@echo "$(YELLOW)🧹 Limpiando archivos de construcción...$(NC)"
	rm -rf $(BUILD_DIR)
	rm -rf $(DIST_DIR)
	rm -rf $(FRONTEND_DIR)/node_modules
	rm -rf $(FRONTEND_DIR)/dist
	cargo clean
	@echo "$(GREEN)✅ Limpieza completada$(NC)"

test: ## Ejecutar tests
	@echo "$(GREEN)🧪 Ejecutando tests...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Tests del backend...$(NC)"
	cargo test
	@echo "$(YELLOW)Tests del frontend...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm test"; \
	else \
		cd $(FRONTEND_DIR) && npm test; \
	fi
	@echo "$(GREEN)✅ Tests completados$(NC)"

test-backend: ## Ejecutar tests del backend
	@echo "$(YELLOW)Tests del backend...$(NC)"
	cargo test

test-frontend: ## Ejecutar tests del frontend
	@echo "$(YELLOW)Tests del frontend...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm test"; \
	else \
		cd $(FRONTEND_DIR) && npm test; \
	fi

format: ## Formatear código
	@echo "$(GREEN)🎨 Formateando código...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Formateando Rust...$(NC)"
	cargo fmt
	@echo "$(YELLOW)Formateando TypeScript/JavaScript...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm run format"; \
	else \
		cd $(FRONTEND_DIR) && npm run format; \
	fi
	@echo "$(GREEN)✅ Formateo completado$(NC)"

lint: ## Ejecutar linter
	@echo "$(GREEN)🔍 Ejecutando linter...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Linting Rust...$(NC)"
	cargo clippy
	@echo "$(YELLOW)Linting TypeScript/JavaScript...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm run lint"; \
	else \
		cd $(FRONTEND_DIR) && npm run lint; \
	fi
	@echo "$(GREEN)✅ Linting completado$(NC)"

check: ## Verificar código sin compilar
	@echo "$(GREEN)✅ Verificando código...$(NC)"
	cargo check
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm run type-check"; \
	else \
		cd $(FRONTEND_DIR) && npm run type-check; \
	fi

docker-build: ## Construir imagen Docker
	@echo "$(GREEN)🐳 Construyendo imagen Docker...$(NC)"
	docker build -t $(APP_NAME) .
	@echo "$(GREEN)✅ Imagen Docker construida$(NC)"

docker-run: ## Ejecutar en Docker
	@echo "$(GREEN)🐳 Ejecutando Alohopass en Docker...$(NC)"
	docker run -p 3000:3000 $(APP_NAME)

setup: ## Configurar entorno de desarrollo
	@echo "$(GREEN)⚙️ Configurando entorno de desarrollo...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@if [ ! -f ".env" ]; then \
		echo "$(YELLOW)Creando archivo .env...$(NC)"; \
		echo "RUST_LOG=info" > .env; \
		echo "RUST_BACKTRACE=1" >> .env; \
		echo "NODE_ENV=development" >> .env; \
	fi
	@echo "$(GREEN)✅ Entorno configurado$(NC)"

update: ## Actualizar dependencias
	@echo "$(GREEN)🔄 Actualizando dependencias...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Actualizando Rust...$(NC)"
	rustup update
	@echo "$(YELLOW)Actualizando npm...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm update"; \
	else \
		cd $(FRONTEND_DIR) && npm update; \
	fi
	@echo "$(GREEN)✅ Dependencias actualizadas$(NC)"

docs: ## Generar documentación
	@echo "$(GREEN)📚 Generando documentación...$(NC)"
	@echo "$(YELLOW)Documentación de Rust...$(NC)"
	cargo doc --open
	@echo "$(GREEN)✅ Documentación generada$(NC)"

release: ## Crear release
	@echo "$(GREEN)🚀 Creando release...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Construyendo para todas las plataformas...$(NC)"
	cargo tauri build --target all
	@echo "$(GREEN)✅ Release creado$(NC)"

install-dev: ## Instalar dependencias de desarrollo
	@echo "$(GREEN)🔧 Instalando dependencias de desarrollo...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm install --save-dev"; \
	else \
		cd $(FRONTEND_DIR) && npm install --save-dev; \
	fi
	@echo "$(GREEN)✅ Dependencias de desarrollo instaladas$(NC)"

watch: ## Ejecutar en modo watch
	@echo "$(GREEN)👀 Ejecutando en modo watch...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Terminal 1: Frontend watch$(NC)"
	@echo "$(YELLOW)Terminal 2: Backend watch$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm run dev" &; \
	else \
		cd $(FRONTEND_DIR) && npm run dev &; \
	fi
	cargo watch -x "tauri dev"

security-check: ## Verificar seguridad del código
	@echo "$(GREEN)🔒 Verificando seguridad del código...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Auditoría de npm...$(NC)"
	@if [ "$(PLATFORM)" = "windows" ]; then \
		$(SHELL_CMD) "cd $(FRONTEND_DIR); npm audit"; \
	else \
		cd $(FRONTEND_DIR) && npm audit; \
	fi
	@echo "$(YELLOW)Verificando dependencias de Rust...$(NC)"
	cargo audit
	@echo "$(GREEN)✅ Verificación de seguridad completada$(NC)"

benchmark: ## Ejecutar benchmarks
	@echo "$(GREEN)⚡ Ejecutando benchmarks...$(NC)"
	cargo bench
	@echo "$(GREEN)✅ Benchmarks completados$(NC)"

profile: ## Generar perfil de rendimiento
	@echo "$(GREEN)📊 Generando perfil de rendimiento...$(NC)"
	cargo build --release
	@echo "$(YELLOW)Ejecuta la aplicación y usa herramientas de profiling$(NC)"

# Comandos específicos para Windows
windows-build: ## Construir específicamente para Windows
	@echo "$(GREEN)🪟 Construyendo para Windows...$(NC)"
	cargo tauri build --target x86_64-pc-windows-msvc

# Comandos específicos para macOS
macos-build: ## Construir específicamente para macOS
	@echo "$(GREEN)🍎 Construyendo para macOS...$(NC)"
	cargo tauri build --target x86_64-apple-darwin

# Comandos específicos para Linux
linux-build: ## Construir específicamente para Linux
	@echo "$(GREEN)🐧 Construyendo para Linux...$(NC)"
	cargo tauri build --target x86_64-unknown-linux-gnu

# Comando para mostrar información del sistema
info: ## Mostrar información del sistema
	@echo "$(GREEN)ℹ️ Información del sistema:$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(BLUE)Rust:$(NC) $(shell rustc --version 2>/dev/null || echo 'No instalado')"
	@echo "$(BLUE)Node.js:$(NC) $(shell node --version 2>/dev/null || echo 'No instalado')"
	@echo "$(BLUE)npm:$(NC) $(shell npm --version 2>/dev/null || echo 'No instalado')"
	@echo "$(BLUE)Cargo:$(NC) $(shell cargo --version 2>/dev/null || echo 'No instalado')"
	@echo "$(BLUE)Tauri:$(NC) $(shell cargo tauri --version 2>/dev/null || echo 'No instalado')"

# Comando para verificar el estado del proyecto
status: ## Verificar estado del proyecto
	@echo "$(GREEN)📋 Estado del proyecto:$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(BLUE)Archivos principales:$(NC)"
	@ls -la Cargo.toml tauri.conf.json 2>/dev/null || echo "$(RED)❌ Archivos principales faltantes$(NC)"
	@echo "$(BLUE)Frontend:$(NC)"
	@ls -la frontend/package.json 2>/dev/null || echo "$(RED)❌ Frontend no configurado$(NC)"
	@echo "$(BLUE)Backend:$(NC)"
	@ls -la src/main.rs 2>/dev/null || echo "$(RED)❌ Backend no configurado$(NC)"

# Comando para ejecutar Tauri dev multiplataforma
tauri-dev: ## Ejecutar Tauri dev (multiplataforma)
	@echo "$(GREEN)🚀 Ejecutando Tauri dev...$(NC)"
	@echo "$(BLUE)Plataforma detectada: $(YELLOW)$(PLATFORM)$(NC)"
	@echo "$(YELLOW)Iniciando aplicación...$(NC)"
	cargo tauri dev 