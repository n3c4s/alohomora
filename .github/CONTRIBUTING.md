# 🤝 Contribuyendo a Alohopass

¡Gracias por tu interés en contribuir a Alohopass! Este documento te guiará a través del proceso de contribución.

## 🚀 Comenzando

### Prerrequisitos
- **Rust** 1.70.0 o superior
- **Node.js** 18.0.0 o superior
- **npm** 8.0.0 o superior
- **Tauri CLI** (instalado via `cargo install tauri-cli`)

### Configuración del Entorno
1. **Fork** el repositorio [n3c4s/alohomora](https://github.com/n3c4s/alohomora)
2. **Clone** tu fork localmente
3. **Instala** las dependencias:
   ```bash
   # Instalar dependencias del frontend
   cd frontend
   npm install
   
   # Verificar dependencias de Rust
   cargo check
   ```

## 🔧 Desarrollo

### Estructura del Proyecto
```
alohomora/
├── src/                    # Backend en Rust
│   ├── crypto/            # Módulo de criptografía
│   ├── database/          # Módulo de base de datos
│   └── models/            # Modelos de datos
├── frontend/              # Frontend en React/TypeScript
│   ├── src/               # Código fuente
│   ├── components/        # Componentes reutilizables
│   ├── pages/            # Páginas de la aplicación
│   └── stores/           # Estado global (Zustand)
└── docs/                  # Documentación
```

### Comandos de Desarrollo
```bash
# Desarrollo completo (backend + frontend)
cargo tauri dev

# Solo frontend
cd frontend && npm run dev

# Solo backend
cargo run

# Tests
cargo test
cd frontend && npm test

# Build
cargo tauri build
```

## 📝 Guías de Código

### Rust
- Usar `rustfmt` para formateo
- Seguir las convenciones de `clippy`
- Documentar funciones públicas con `///`
- Usar `anyhow` para manejo de errores
- Implementar tests para nueva funcionalidad

### TypeScript/React
- Usar ESLint y Prettier
- Seguir las convenciones de React Hooks
- Usar TypeScript strict mode
- Implementar tests con Jest/React Testing Library
- Usar Zustand para estado global

### Commits
Usar [Conventional Commits](https://www.conventionalcommits.org/):
```
feat: agregar autocompletado en navegadores
fix: corregir error de encriptación
docs: actualizar README
style: formatear código
refactor: reorganizar módulo de criptografía
test: agregar tests para generador de contraseñas
chore: actualizar dependencias
```

## 🐛 Reportando Bugs

1. **Busca** si el bug ya fue reportado
2. **Usa** la plantilla de bug report
3. **Incluye** información del sistema
4. **Proporciona** pasos de reproducción claros
5. **Adjunta** logs y capturas de pantalla

## ✨ Sugiriendo Features

1. **Verifica** si la feature ya fue solicitada
2. **Usa** la plantilla de feature request
3. **Describe** el caso de uso específico
4. **Considera** el impacto en la seguridad
5. **Proporciona** mockups si es posible

## 🔒 Seguridad

### Reportando Vulnerabilidades
- **NO** abras issues públicos para vulnerabilidades
- **Envía** un email a n3c4s@github.com
- **Incluye** detalles técnicos del problema
- **Espera** confirmación antes de divulgar

### Principios de Seguridad
- **Nunca** almacenar contraseñas en texto plano
- **Siempre** usar algoritmos criptográficos estándar
- **Validar** todas las entradas del usuario
- **Implementar** rate limiting donde sea apropiado

## 📚 Documentación

### Tipos de Documentación
- **README.md** - Visión general del proyecto
- **docs/** - Documentación técnica detallada
- **Código** - Comentarios y documentación inline
- **API** - Documentación de endpoints y funciones

### Estándares
- Usar Markdown para todos los documentos
- Incluir ejemplos de código
- Mantener actualizada con cambios del código
- Usar lenguaje claro y conciso

## 🧪 Testing

### Cobertura Mínima
- **Backend:** 80% de cobertura
- **Frontend:** 70% de cobertura
- **Integración:** Tests de flujos completos

### Tipos de Tests
- **Unitarios** - Funciones individuales
- **Integración** - Módulos y componentes
- **E2E** - Flujos de usuario completos
- **Seguridad** - Tests de vulnerabilidades

## 🚀 Pull Requests

### Proceso
1. **Crea** una rama descriptiva
2. **Implementa** tu cambio
3. **Agrega** tests apropiados
4. **Actualiza** documentación
5. **Ejecuta** tests localmente
6. **Envía** el PR con descripción clara

### Checklist del PR
- [ ] Código sigue las guías del proyecto
- [ ] Tests pasan localmente
- [ ] Documentación actualizada
- [ ] Commits siguen convenciones
- [ ] PR tiene descripción clara
- [ ] Cambios son seguros

## 🏷️ Releases

### Versionado
Usar [Semantic Versioning](https://semver.org/):
- **MAJOR** - Cambios incompatibles
- **MINOR** - Nuevas funcionalidades compatibles
- **PATCH** - Correcciones de bugs compatibles

### Proceso de Release
1. **Actualizar** versiones en archivos de configuración
2. **Generar** changelog
3. **Crear** tag de Git
4. **Subir** assets de release
5. **Actualizar** documentación

## 🆘 Obtener Ayuda

### Canales de Soporte
- **Issues** - Para bugs y features
- **Discussions** - Para preguntas generales
- **Email** - Para consultas privadas
- **Documentación** - Para guías técnicas

### Recursos
- [Documentación de Rust](https://doc.rust-lang.org/)
- [Documentación de Tauri](https://tauri.app/docs/)
- [Documentación de React](https://react.dev/)
- [Guías de seguridad OWASP](https://owasp.org/)

## 🙏 Reconocimientos

### Contribuidores
- **@n3c4s** - Desarrollador principal
- **Comunidad** - Contribuidores y usuarios

### Agradecimientos
- **Rust Team** - Lenguaje de programación
- **Tauri Team** - Framework multiplataforma
- **React Team** - Biblioteca de UI
- **Comunidad open source** - Dependencias y herramientas

---

**¿Listo para contribuir? ¡Comienza con un issue o un PR pequeño!**

**Desarrollado por [@n3c4s](https://github.com/n3c4s) - [alohopass.com](https://alohopass.com)** 