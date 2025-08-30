# 🔐 Alohopass - Secure Password Manager

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=black)](https://tauri.app)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org)

> A modern, secure password manager inspired by the Alohomora spell from Harry Potter. Built with Rust, Tauri, and React.

[🇪🇸 Leer en Español](#español) | [🇺🇸 Read in English](#english)

---

## 🌟 Features

- 🔒 **Military-grade encryption** using ChaCha20-Poly1305 and Argon2
- 🎨 **Modern UI** built with React, TypeScript, and Tailwind CSS
- 🚀 **Cross-platform** desktop application (Windows, macOS, Linux)
- 🔐 **Secure storage** with SQLite database and encrypted vaults
- ⌨️ **Global shortcuts** for quick access
- 🌍 **Multi-language support** (English/Spanish)
- 🔄 **Auto-sync** across devices
- 📱 **Responsive design** for all screen sizes

## 🚀 Quick Start

### Prerequisites

- **Rust** (with Cargo) - [Install Rust](https://rustup.rs/)
- **Node.js** and **npm** - [Install Node.js](https://nodejs.org/)
- **Tauri CLI** - `cargo install tauri-cli --version "^1.5"`

### Installation & Development

#### Method 1: Direct Command (Recommended)
```bash
# Clone the repository
git clone https://github.com/n3c4s/alohomora.git
cd alohomora

# Install frontend dependencies
cd frontend && npm install && cd ..

# Run in development mode
cargo tauri dev
```

#### Method 2: Multi-platform Scripts
```bash
# macOS/Linux
./dev.sh dev

# Windows PowerShell
.\dev.ps1 dev

# Any platform (Make)
make tauri-dev
```

#### Method 3: Step by Step
```bash
# 1. Install frontend dependencies
cd frontend
npm install
cd ..

# 2. Verify Rust dependencies
cargo check

# 3. Run the application
cargo tauri dev
```

### Building for Production

```bash
# Build for all platforms
cargo tauri build

# Build for specific platform
cargo tauri build --target x86_64-apple-darwin  # macOS
cargo tauri build --target x86_64-pc-windows-msvc  # Windows
cargo tauri build --target x86_64-unknown-linux-gnu  # Linux
```

## 🛠️ Development

### Project Structure
```
alohomora/
├── src/                    # Rust backend
│   ├── crypto/            # Encryption & key derivation
│   ├── database/          # SQLite database management
│   └── models/            # Data structures
├── frontend/              # React frontend
│   ├── src/               # React components
│   ├── components/        # Reusable UI components
│   └── pages/             # Application pages
├── icons/                 # Application icons
└── tauri.conf.json        # Tauri configuration
```

### Available Commands

| Command | Description | Platform |
|---------|-------------|----------|
| `cargo tauri dev` | Run in development mode | All |
| `./dev.sh dev` | Multi-platform script | macOS/Linux |
| `.\dev.ps1 dev` | PowerShell script | Windows |
| `make tauri-dev` | Makefile command | All |

### Development Scripts

```bash
# Frontend only
npm run dev              # In frontend/ directory
./dev.sh frontend        # Multi-platform

# Backend only
cargo run               # Rust backend
./dev.sh backend        # Multi-platform

# Testing
cargo test              # Rust tests
npm test                # Frontend tests (in frontend/)

# Linting & Formatting
cargo fmt               # Format Rust code
cargo clippy            # Lint Rust code
npm run lint            # Lint frontend code
```

## 🔧 Configuration

### Environment Variables
Create a `.env` file in the root directory:
```env
RUST_LOG=info
RUST_BACKTRACE=1
NODE_ENV=development
```

### Tauri Configuration
The `tauri.conf.json` file is automatically configured for multi-platform support:
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

## 🌍 Multi-language Support

Alohopass automatically detects your system language and displays the interface in:
- 🇺🇸 **English** (default)
- 🇪🇸 **Español**

The language detection is based on your operating system's locale settings.

## 📱 Cross-platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| 🪟 Windows | ✅ Supported | Windows 10/11, PowerShell |
| 🍎 macOS | ✅ Supported | Intel & Apple Silicon |
| 🐧 Linux | ✅ Supported | Most distributions |

## 🔒 Security Features

- **Encryption**: ChaCha20-Poly1305 for data encryption
- **Key Derivation**: Argon2 for password hashing
- **Secure Storage**: Encrypted SQLite database
- **Memory Protection**: Secure memory wiping
- **Audit Trail**: Comprehensive logging

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri** team for the amazing desktop app framework
- **Rust** community for the safe systems programming language
- **React** team for the powerful UI library
- **Harry Potter** universe for the magical inspiration

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/n3c4s/alohomora/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/n3c4s/alohomora/discussions)
- 📧 **Email**: n3c4s@github.com
- 🌐 **Website**: [alohopass.com](https://alohopass.com)

---

<div align="center">

**Made with ❤️ by [@n3c4s](https://github.com/n3c4s)**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/n3c4s)
[![Website](https://img.shields.io/badge/Website-FF6B6B?style=for-the-badge&logo=vercel&logoColor=white)](https://alohopass.com)

</div>

---

## 🇪🇸 Español

### 🔐 Alohopass - Gestor de Contraseñas Seguro

> Un gestor de contraseñas moderno y seguro inspirado en el encantamiento Alohomora de Harry Potter. Construido con Rust, Tauri y React.

### 🚀 Inicio Rápido

#### Prerrequisitos

- **Rust** (con Cargo) - [Instalar Rust](https://rustup.rs/)
- **Node.js** y **npm** - [Instalar Node.js](https://nodejs.org/)
- **Tauri CLI** - `cargo install tauri-cli --version "^1.5"`

#### Instalación y Desarrollo

```bash
# Clonar el repositorio
git clone https://github.com/n3c4s/alohomora.git
cd alohomora

# Instalar dependencias del frontend
cd frontend && npm install && cd ..

# Ejecutar en modo desarrollo
cargo tauri dev
```

#### Scripts Multiplataforma

```bash
# macOS/Linux
./dev.sh dev

# Windows PowerShell
.\dev.ps1 dev

# Cualquier plataforma (Make)
make tauri-dev
```

### 🌟 Características

- 🔒 **Encriptación de grado militar** usando ChaCha20-Poly1305 y Argon2
- 🎨 **Interfaz moderna** construida con React, TypeScript y Tailwind CSS
- 🚀 **Aplicación multiplataforma** (Windows, macOS, Linux)
- 🔐 **Almacenamiento seguro** con base de datos SQLite y bóvedas encriptadas
- ⌨️ **Atajos globales** para acceso rápido
- 🌍 **Soporte multiidioma** (Inglés/Español)
- 🔄 **Sincronización automática** entre dispositivos
- 📱 **Diseño responsivo** para todas las pantallas

### 🛠️ Desarrollo

#### Estructura del Proyecto
```
alohomora/
├── src/                    # Backend en Rust
│   ├── crypto/            # Encriptación y derivación de claves
│   ├── database/          # Gestión de base de datos SQLite
│   └── models/            # Estructuras de datos
├── frontend/              # Frontend en React
│   ├── src/               # Componentes React
│   ├── components/        # Componentes UI reutilizables
│   └── pages/             # Páginas de la aplicación
├── icons/                 # Iconos de la aplicación
└── tauri.conf.json        # Configuración de Tauri
```

#### Comandos Disponibles

| Comando | Descripción | Plataforma |
|---------|-------------|------------|
| `cargo tauri dev` | Ejecutar en modo desarrollo | Todas |
| `./dev.sh dev` | Script multiplataforma | macOS/Linux |
| `.\dev.ps1 dev` | Script PowerShell | Windows |
| `make tauri-dev` | Comando Makefile | Todas |

### 🌍 Soporte Multiidioma

Alohopass detecta automáticamente el idioma de tu sistema y muestra la interfaz en:
- 🇺🇸 **Inglés** (predeterminado)
- 🇪🇸 **Español**

La detección del idioma se basa en la configuración de idioma de tu sistema operativo.

### 📱 Soporte Multiplataforma

| Plataforma | Status | Notes |
|------------|--------|-------|
| 🪟 Windows | ✅ Soportado | Windows 10/11, PowerShell |
| 🍎 macOS | ✅ Soportado | Intel y Apple Silicon |
| 🐧 Linux | ✅ Soportado | La mayoría de distribuciones |

### 🔒 Características de Seguridad

- **Encriptación**: ChaCha20-Poly1305 para encriptación de datos
- **Derivación de Claves**: Argon2 para hash de contraseñas
- **Almacenamiento Seguro**: Base de datos SQLite encriptada
- **Protección de Memoria**: Borrado seguro de memoria
- **Auditoría**: Registro completo de actividades

### 🤝 Contribuir

¡Bienvenimos las contribuciones! Por favor, consulta nuestra [Guía de Contribución](CONTRIBUTING.md) para más detalles.

### 📞 Soporte

- 🐛 **Reportes de Errores**: [GitHub Issues](https://github.com/n3c4s/alohomora/issues)
- 💡 **Solicitudes de Funciones**: [GitHub Discussions](https://github.com/n3c4s/alohomora/discussions)
- 📧 **Email**: n3c4s@github.com
- 🌐 **Sitio Web**: [alohopass.com](https://alohopass.com)

---

<div align="center">

**Hecho con ❤️ por [@n3c4s](https://github.com/n3c4s)**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/n3c4s)
[![Website](https://img.shields.io/badge/Sitio_Web-FF6B6B?style=for-the-badge&logo=vercel&logoColor=white)](https://alohopass.com)

</div> 