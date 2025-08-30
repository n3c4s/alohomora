# 🔐 AlohoPass - Plugin de Navegador

Plugin de navegador para AlohoPass que permite autocompletar formularios de login y signup de forma segura.

## ✨ Características

- **🔍 Detección automática** de formularios de login y signup
- **⚡ Autocompletado inteligente** de contraseñas
- **🔄 Sincronización P2P** con la aplicación desktop
- **🔒 Seguridad máxima** - las contraseñas nunca salen de tu dispositivo
- **🎨 Interfaz moderna** y fácil de usar
- **🌐 Soporte multiidioma** (Español/Inglés)
- **🎯 Iconos nativos** de AlohoPass para una experiencia consistente

## 🚀 Instalación

### Chrome/Edge/Brave

1. Descarga o clona este repositorio
2. Abre Chrome y ve a `chrome://extensions/`
3. Activa el "Modo desarrollador" (esquina superior derecha)
4. Haz clic en "Cargar extensión sin empaquetar"
5. Selecciona la carpeta `browser-extension`
6. ¡Listo! El plugin aparecerá en tu barra de herramientas

### Firefox

1. Descarga o clona este repositorio
2. Abre Firefox y ve a `about:debugging`
3. Haz clic en "Este Firefox"
4. Haz clic en "Cargar complemento temporal"
5. Selecciona el archivo `manifest.json` de la carpeta `browser-extension`
6. ¡Listo! El plugin aparecerá en tu barra de herramientas

## 🔧 Configuración

### Requisitos

- **AlohoPass Desktop** debe estar ejecutándose en tu computadora
- **Puerto 5175** debe estar disponible para la comunicación local

### Conexión con la aplicación

1. Asegúrate de que AlohoPass Desktop esté ejecutándose
2. El plugin se conectará automáticamente usando Native Messaging
3. Verás un indicador verde cuando esté conectado

### Instalación del host nativo

#### macOS/Linux
```bash
cd browser-extension
chmod +x install-native-host.sh
./install-native-host.sh
```

#### Windows
```cmd
cd browser-extension
install-native-host.bat
```
*Ejecutar como administrador*

## 📱 Uso

### Autocompletado automático

1. Ve a cualquier sitio web con formulario de login/signup
2. Haz clic en un campo de usuario o contraseña
3. Aparecerá un overlay con sugerencias de contraseñas
4. Selecciona la contraseña que quieres usar
5. El formulario se autocompletará automáticamente

### Crear nuevas contraseñas

1. Cuando estés en un formulario de signup
2. Completa los campos manualmente
3. Al enviar el formulario, se creará automáticamente una nueva entrada
4. La contraseña estará disponible para futuros logins

### Gestión desde el popup

- **🖥️ Abrir AlohoPass**: Abre la aplicación desktop
- **🔄 Sincronizar**: Sincroniza con otros dispositivos
- **🔍 Buscar**: Busca contraseñas específicas
- **⚙️ Configuración**: Ajusta preferencias del plugin

## 🛡️ Seguridad

- **🔒 Encriptación local**: Todas las contraseñas están encriptadas en tu dispositivo
- **🌐 Sin servidores**: No hay servidores en la nube que puedan ser hackeados
- **🔐 Sincronización P2P**: Los datos se sincronizan directamente entre dispositivos
- **📱 Verificación**: El plugin solo funciona cuando la aplicación desktop está ejecutándose

## 🔧 Desarrollo

### Estructura del proyecto

```
browser-extension/
├── manifest.json              # Configuración del plugin
├── content.js                 # Script que se ejecuta en las páginas web
├── background.js              # Script de fondo para comunicación
├── popup.html                 # Interfaz del popup
├── popup.js                   # Lógica del popup
├── icons/                     # Iconos del plugin (usando Alohopass.png)
│   ├── icon16.png            # 16x16 (redimensionado automáticamente)
│   ├── icon32.png            # 32x32 (redimensionado automáticamente)
│   ├── icon48.png            # 48x48 (redimensionado automáticamente)
│   ├── icon128.png           # 128x128 (redimensionado automáticamente)
│   └── icon.ico              # Icono ICO para Windows
├── native-host.json           # Configuración del host nativo
├── install-native-host.sh     # Script de instalación (macOS/Linux)
├── install-native-host.bat    # Script de instalación (Windows)
├── test-extension.html        # Página de prueba del plugin
└── README.md                  # Esta documentación
```

### Tecnologías utilizadas

- **Manifest V3** - Estándar moderno de extensiones
- **Native Messaging** - Comunicación con aplicaciones desktop
- **Chrome Extensions API** - Funcionalidades del navegador
- **Vanilla JavaScript** - Sin dependencias externas
- **Iconos nativos** - Usando `Alohopass.png` existente

### Comandos de desarrollo

```bash
# Instalar dependencias (si las hubiera)
npm install

# Construir para producción
npm run build

# Desarrollo con hot reload
npm run dev
```

## 🧪 Pruebas

### Página de prueba incluida

El plugin incluye una página de prueba completa (`test-extension.html`) que puedes usar para verificar que todo funcione correctamente:

1. Abre `browser-extension/test-extension.html` en tu navegador
2. Verifica que el plugin detecte los formularios
3. Prueba el autocompletado en los campos
4. Revisa la consola para logs detallados

## 🐛 Solución de problemas

### El plugin no se conecta a la aplicación

1. Verifica que AlohoPass Desktop esté ejecutándose
2. Comprueba que el puerto 5175 esté disponible
3. Asegúrate de haber instalado el host nativo
4. Reinicia el plugin desde `chrome://extensions/`
5. Verifica los logs de la consola del navegador

### No aparecen sugerencias de contraseñas

1. Asegúrate de estar conectado a la aplicación
2. Verifica que tengas contraseñas guardadas para ese sitio
3. Comprueba que el formulario sea reconocido como login/signup
4. Revisa la consola del navegador para errores

### Problemas de permisos

1. Ve a `chrome://extensions/`
2. Encuentra AlohoPass y haz clic en "Detalles"
3. Verifica que todos los permisos estén habilitados
4. Si es necesario, desinstala y vuelve a instalar

### Problemas con el host nativo

1. Verifica que el script de instalación se ejecutó correctamente
2. Comprueba que los archivos de configuración se crearon
3. Asegúrate de que la ruta al ejecutable sea correcta
4. En Windows, ejecuta como administrador

## 📝 Logs y debugging

### Habilitar logs detallados

1. Abre la consola del navegador (F12)
2. Ve a la pestaña "Console"
3. Filtra por "AlohoPass" para ver solo los logs del plugin

### Logs del background script

1. Ve a `chrome://extensions/`
2. Encuentra AlohoPass y haz clic en "Detalles"
3. Haz clic en "service worker" para ver los logs del background

### Logs de la aplicación Tauri

1. Ejecuta `cargo tauri dev` en la terminal
2. Los logs del browser extension manager aparecerán en la consola

## 🤝 Contribuir

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver `LICENSE` para más detalles.

## 🆘 Soporte

Si tienes problemas o preguntas:

1. Revisa esta documentación
2. Busca en los issues del repositorio
3. Crea un nuevo issue con detalles del problema
4. Incluye logs de la consola y pasos para reproducir

## 🔄 Actualizaciones

Para actualizar el plugin:

1. Descarga la nueva versión
2. Reemplaza los archivos en la carpeta del plugin
3. Ve a `chrome://extensions/`
4. Haz clic en el botón de recarga del plugin
5. ¡Listo! Tendrás la nueva versión

## 🎨 Iconos

El plugin utiliza los iconos nativos de AlohoPass:
- **Alohopass.png** - Icono principal en diferentes tamaños
- **Alohopass.ico** - Icono para Windows
- **Consistencia visual** - Misma identidad que la aplicación desktop

---

**🔐 AlohoPass** - Tu gestor de contraseñas seguro y privado
