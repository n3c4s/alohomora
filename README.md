# 🔐 Alohopass - Gestor de Contraseñas Seguro

> *"Alohomora!" - El encantamiento que abre puertas cerradas en Harry Potter*

**Alohopass** es un gestor de contraseñas de código abierto, altamente seguro y multiplataforma, inspirado en el encantamiento mágico Alohomora. Desarrollado con **Rust + Tauri** para máxima seguridad y **React + TypeScript** para una interfaz moderna y intuitiva.

## ✨ Características Principales

### 🔒 **Seguridad de Nivel Militar**
- **Encriptación AES-256-GCM** para máxima protección
- **Derivación de claves Argon2** con salt único por usuario
- **Base de datos SQLite encriptada** localmente
- **Nunca almacena contraseñas en texto plano**
- **Verificación de fortaleza de contraseñas** en tiempo real

### 🌐 **Autocompletado Inteligente en Navegadores**
- **Detección automática** de formularios de login
- **Sugerencias contextuales** basadas en la URL actual
- **Autocompletado con un clic** de usuario y contraseña
- **Guardado automático** de nuevas credenciales
- **Integración nativa** con todos los navegadores modernos

### 🎯 **Gestión Completa de Contraseñas**
- **CRUD completo** para entradas de contraseñas
- **Categorización inteligente** y etiquetas personalizables
- **Búsqueda avanzada** por título, usuario, URL o etiquetas
- **Historial de uso** y fechas de creación/actualización
- **Exportación/importación** segura de datos

### 🚀 **Generador de Contraseñas Avanzado**
- **Configuración personalizable** de longitud y caracteres
- **Exclusión de caracteres similares** (l, 1, I, O, 0)
- **Verificación de fortaleza** en tiempo real
- **Sugerencias de mejora** automáticas
- **Copiado al portapapeles** con un clic

### 📊 **Dashboard Inteligente**
- **Estadísticas de seguridad** en tiempo real
- **Identificación de contraseñas débiles**
- **Contraseñas recientes** y uso frecuente
- **Puntuación de seguridad** general
- **Acciones rápidas** para tareas comunes

## 🏗️ Arquitectura Técnica

### **Backend (Rust)**
```
src/
├── main.rs              # Punto de entrada y comandos Tauri
├── crypto/
│   ├── mod.rs           # Gestor de criptografía
│   ├── encryption.rs    # Encriptación AES-256-GCM
│   └── key_derivation.rs # Derivación de claves Argon2
├── database/
│   ├── mod.rs           # Gestor de base de datos
│   ├── connection.rs    # Conexiones SQLite
│   ├── migrations.rs    # Migraciones de esquema
│   └── repository.rs    # Operaciones CRUD
└── models/
    ├── mod.rs           # Estructuras de datos
    ├── password_entry.rs # Entrada de contraseña
    ├── category.rs      # Categorías
    └── user.rs          # Usuario y autenticación
```

### **Frontend (React + TypeScript)**
```
frontend/src/
├── components/
│   ├── Layout.tsx       # Layout principal de la aplicación
│   └── BrowserAutocomplete.tsx # Autocompletado en navegadores
├── pages/
│   ├── LoginPage.tsx    # Página de autenticación
│   ├── DashboardPage.tsx # Dashboard principal
│   ├── PasswordsPage.tsx # Gestión de contraseñas
│   ├── GeneratorPage.tsx # Generador de contraseñas
│   └── SettingsPage.tsx # Configuración
├── stores/
│   ├── authStore.ts     # Estado de autenticación
│   ├── passwordStore.ts # Estado de contraseñas
│   └── autocompleteStore.ts # Estado de autocompletado
└── App.tsx              # Componente raíz
```

## 🚀 Instalación y Configuración

### **Prerrequisitos**
- **Rust** 1.70.0 o superior
- **Node.js** 18.0.0 o superior
- **npm** 8.0.0 o superior
- **Tauri CLI** (se instala automáticamente)

### **Instalación Automática (Recomendada)**

#### **Windows (PowerShell como Administrador)**
```powershell
# Descargar y ejecutar el script de instalación
.\install.ps1
```

#### **Linux/macOS**
```bash
# Dar permisos de ejecución
chmod +x install.sh

# Ejecutar instalación
./install.sh
```

### **Instalación Manual**

#### **1. Clonar el repositorio**
```bash
git clone https://github.com/tuusuario/alohopass.git
cd alohopass
```

#### **2. Instalar dependencias del frontend**
```bash
cd frontend
npm install
cd ..
```

#### **3. Verificar dependencias de Rust**
```bash
cargo check
```

#### **4. Instalar Tauri CLI**
```bash
cargo install tauri-cli
```

## 🎮 Uso y Funcionalidades

### **Primera Ejecución**
1. **Ejecutar en modo desarrollo:**
   ```bash
   # Terminal 1: Frontend
   cd frontend && npm run dev
   
   # Terminal 2: Backend
   cargo tauri dev
   ```

2. **Crear contraseña maestra:**
   - La aplicación detectará que es la primera vez
   - Crea una contraseña maestra segura
   - Esta contraseña nunca se almacena, solo se deriva

### **Gestión de Contraseñas**
- **Agregar nueva:** Botón "+" en la página de contraseñas
- **Editar:** Clic en el ícono de edición
- **Eliminar:** Clic en el ícono de eliminación
- **Buscar:** Barra de búsqueda en tiempo real
- **Copiar:** Botones de copia para usuario y contraseña

### **Generador de Contraseñas**
- **Configurar parámetros:** Longitud, tipos de caracteres
- **Generar:** Clic en "Generar Contraseña"
- **Verificar fortaleza:** Análisis automático con sugerencias
- **Copiar:** Botón de copia integrado

### **Autocompletado en Navegadores**
- **Detectar formularios:** Automático al navegar
- **Ver sugerencias:** Botón flotante de Alohopass
- **Autocompletar:** Clic en la sugerencia deseada
- **Guardar nueva:** Formulario integrado en el panel

## 🔧 Comandos de Desarrollo

### **Makefile (Recomendado)**
```bash
# Ver todos los comandos disponibles
make help

# Instalar dependencias
make install

# Modo desarrollo
make dev

# Construir para producción
make build

# Limpiar archivos de construcción
make clean

# Ejecutar tests
make test

# Formatear código
make format

# Verificar seguridad
make security-check
```

### **Comandos Directos**
```bash
# Desarrollo
cargo tauri dev          # Backend + Frontend
cd frontend && npm run dev  # Solo Frontend

# Construcción
cargo tauri build        # Construir aplicación
cargo build --release    # Construir backend optimizado

# Tests
cargo test               # Tests del backend
cd frontend && npm test  # Tests del frontend

# Limpieza
cargo clean              # Limpiar Rust
cd frontend && npm run clean  # Limpiar Frontend
```

## 🛡️ Características de Seguridad

### **Encriptación**
- **AES-256-GCM:** Algoritmo de encriptación simétrica de nivel militar
- **Claves de 256 bits:** Máxima seguridad disponible
- **Vectores de inicialización únicos:** Previene ataques de repetición

### **Derivación de Claves**
- **Argon2:** Algoritmo ganador del Password Hashing Competition
- **Salt único:** Cada usuario tiene su propio salt
- **Factor de trabajo configurable:** Ajustable según hardware

### **Almacenamiento Seguro**
- **Base de datos local:** No hay datos en la nube
- **Encriptación de campos sensibles:** Solo título, usuario y contraseña
- **Metadatos visibles:** URL, notas, etiquetas para funcionalidad

### **Autenticación**
- **Contraseña maestra:** Única entrada de credenciales
- **Verificación local:** Sin comunicación externa
- **Bloqueo automático:** Al cerrar la aplicación

## 🌍 Autocompletado en Navegadores

### **Funcionamiento**
1. **Detección automática** de formularios de login
2. **Análisis de URL** para encontrar contraseñas relevantes
3. **Panel flotante** con sugerencias contextuales
4. **Autocompletado con un clic** de usuario y contraseña
5. **Guardado automático** de nuevas credenciales

### **Características**
- **Integración nativa** con todos los navegadores
- **Detección inteligente** de campos de formulario
- **Sugerencias contextuales** basadas en el dominio
- **Interfaz no intrusiva** que no interfiere con la navegación
- **Seguridad máxima** sin comprometer la usabilidad

### **Configuración**
- **Activación automática** al navegar por sitios web
- **Personalización** de campos a detectar
- **Exclusiones** para sitios específicos
- **Historial** de autocompletado exitoso

## 📱 Plataformas Soportadas

### **Desktop**
- ✅ **Windows** 10/11 (x64)
- ✅ **macOS** 10.15+ (Intel/Apple Silicon)
- ✅ **Linux** Ubuntu 20.04+, Debian 11+, Arch Linux

### **Navegadores Web**
- ✅ **Chrome** 90+
- ✅ **Firefox** 88+
- ✅ **Safari** 14+
- ✅ **Edge** 90+

## 🧪 Testing y Calidad

### **Tests Automatizados**
```bash
# Tests del backend
cargo test

# Tests del frontend
cd frontend && npm test

# Tests de integración
cargo test --test integration

# Tests de rendimiento
cargo bench
```

### **Verificación de Calidad**
```bash
# Linting
make lint

# Formateo
make format

# Verificación de tipos
make check

# Auditoría de seguridad
make security-check
```

## 📦 Distribución

### **Construcción para Producción**
```bash
# Construir para todas las plataformas
make release

# Construir para plataforma específica
make windows-build    # Windows
make macos-build      # macOS
make linux-build      # Linux
```

### **Formatos de Salida**
- **Windows:** `.exe` (instalador)
- **macOS:** `.dmg` (imagen de disco)
- **Linux:** `.AppImage` (aplicación portable)

## 🤝 Contribución

### **Cómo Contribuir**
1. **Fork** el repositorio
2. **Crea** una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. **Commit** tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. **Push** a la rama (`git push origin feature/AmazingFeature`)
5. **Abre** un Pull Request

### **Áreas de Contribución**
- 🐛 **Bug fixes** y mejoras de estabilidad
- ✨ **Nuevas funcionalidades** y características
- 🎨 **Mejoras de UI/UX** y diseño
- 📚 **Documentación** y ejemplos
- 🧪 **Tests** y cobertura de código
- 🔒 **Mejoras de seguridad** y auditorías

### **Estándares de Código**
- **Rust:** Seguir las convenciones de `rustfmt` y `clippy`
- **TypeScript:** Usar ESLint y Prettier configurados
- **Commits:** Usar [Conventional Commits](https://conventionalcommits.org/)
- **Tests:** Mantener cobertura mínima del 80%

## 📄 Licencia

Este proyecto está licenciado bajo la **MIT License** - ver el archivo [LICENSE](LICENSE) para detalles.

## 🙏 Agradecimientos

- **J.K. Rowling** por la inspiración de Alohomora
- **Rust Team** por el lenguaje de programación
- **Tauri Team** por el framework multiplataforma
- **React Team** por la biblioteca de UI
- **Comunidad open source** por las dependencias utilizadas

## 🆘 Soporte

### **Documentación**
- 📖 **README.md** - Este archivo
- 🔧 **Makefile** - Comandos de desarrollo
- ⚙️ **tauri.conf.json** - Configuración de Tauri
- 📦 **package.json** - Dependencias del frontend

### **Canales de Soporte**
- 🐛 **Issues de GitHub** - Reportar bugs y solicitar features
- 💬 **Discussions** - Preguntas y discusiones
- 📧 **Email** - Contacto directo para consultas privadas

### **Recursos Adicionales**
- 🔗 **Sitio web oficial** - [alohopass.com](https://alohopass.com)
- 📚 **Documentación completa** - [docs.alohopass.com](https://docs.alohopass.com)
- 🎥 **Tutoriales en video** - [YouTube](https://youtube.com/@alohopass)

## 🗺️ Roadmap

### **Versión 1.0 (Actual)**
- ✅ Sistema de autenticación completo
- ✅ Gestión básica de contraseñas
- ✅ Generador de contraseñas
- ✅ Dashboard con estadísticas
- ✅ Autocompletado en navegadores
- ✅ Encriptación AES-256-GCM
- ✅ Base de datos SQLite

### **Versión 1.1 (Próxima)**
- 🔄 Sincronización entre dispositivos
- 🔄 Backup en la nube (opcional)
- 🔄 Historial de cambios
- 🔄 Notificaciones de seguridad
- 🔄 Temas personalizables

### **Versión 1.2 (Futura)**
- 📱 Aplicación móvil nativa
- 🌐 Extensión de navegador
- 🔐 Autenticación de dos factores
- 📊 Reportes de seguridad avanzados
- 🔗 Integración con servicios externos

### **Versión 2.0 (Largo plazo)**
- 🤖 IA para sugerencias de contraseñas
- 🔍 Detección de contraseñas comprometidas
- 🌍 Soporte para múltiples idiomas
- 🔌 API para desarrolladores
- 🎯 Modo empresarial con gestión de equipos

## 👨‍💻 Autor

**Alohopass** es desarrollado y mantenido por [@n3c4s](https://github.com/n3c4s).

- 🌐 **Sitio web:** [alohopass.com](https://alohopass.com)
- 📧 **Contacto:** [n3c4s@github.com](mailto:n3c4s@github.com)
- 🐛 **Issues:** [GitHub Issues](https://github.com/n3c4s/alohomora/issues)
- 💬 **Discusiones:** [GitHub Discussions](https://github.com/n3c4s/alohomora/discussions)

---

## 🎯 **¡Comienza a usar Alohopass hoy mismo!**

**Alohopass** te ofrece la seguridad de nivel militar que necesitas para proteger tus credenciales digitales, con la simplicidad y elegancia que mereces.

> *"La seguridad no es un lujo, es una necesidad en el mundo digital de hoy"*

**¿Listo para abrir la puerta a la seguridad digital?** 🚪✨

---

*Desarrollado con ❤️ y 🦀 por [@n3c4s](https://github.com/n3c4s) y la comunidad Alohopass* 