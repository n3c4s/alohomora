@echo off
REM Script de instalación del host nativo para AlohoPass en Windows
REM Ejecutar como administrador

echo 🔐 AlohoPass: Instalando host nativo para la extensión del navegador...

REM Detectar la ruta de Chrome
set CHROME_CONFIG_DIR=%LOCALAPPDATA%\Google\Chrome\User Data\NativeMessagingHosts
set FIREFOX_CONFIG_DIR=%APPDATA%\Mozilla\NativeMessagingHosts

echo 📱 Detectado: Windows

REM Buscar el ejecutable de AlohoPass
set ALOHOPASS_PATH=
if exist "alohopass.exe" (
    set ALOHOPASS_PATH=%CD%\alohopass.exe
) else if exist "target\debug\alohopass.exe" (
    set ALOHOPASS_PATH=%CD%\target\debug\alohopass.exe
) else if exist "target\release\alohopass.exe" (
    set ALOHOPASS_PATH=%CD%\target\release\alohopass.exe
) else (
    echo ❌ No se pudo encontrar el ejecutable de AlohoPass
    echo    Asegúrate de haber compilado la aplicación primero con 'cargo build'
    pause
    exit /b 1
)

echo 📍 Ruta de AlohoPass: %ALOHOPASS_PATH%

REM Crear directorios si no existen
if not exist "%CHROME_CONFIG_DIR%" mkdir "%CHROME_CONFIG_DIR%"
if not exist "%FIREFOX_CONFIG_DIR%" mkdir "%FIREFOX_CONFIG_DIR%"

REM Crear el archivo de configuración para Chrome
set CHROME_CONFIG=%CHROME_CONFIG_DIR%\com.alohopass.browser.json
echo {> "%CHROME_CONFIG%"
echo   "name": "com.alohopass.browser",>> "%CHROME_CONFIG%"
echo   "description": "AlohoPass Browser Extension Host",>> "%CHROME_CONFIG%"
echo   "path": "%ALOHOPASS_PATH%",>> "%CHROME_CONFIG%"
echo   "type": "stdio",>> "%CHROME_CONFIG%"
echo   "allowed_origins": [>> "%CHROME_CONFIG%"
echo     "chrome-extension://*",>> "%CHROME_CONFIG%"
echo     "moz-extension://*">> "%CHROME_CONFIG%"
echo   ]>> "%CHROME_CONFIG%"
echo }>> "%CHROME_CONFIG%"

echo ✅ Configuración de Chrome creada en: %CHROME_CONFIG%

REM Crear el archivo de configuración para Firefox
set FIREFOX_CONFIG=%FIREFOX_CONFIG_DIR%\com.alohopass.browser.json
echo {> "%FIREFOX_CONFIG%"
echo   "name": "com.alohopass.browser",>> "%FIREFOX_CONFIG%"
echo   "description": "AlohoPass Browser Extension Host",>> "%FIREFOX_CONFIG%"
echo   "path": "%ALOHOPASS_PATH%",>> "%FIREFOX_CONFIG%"
echo   "type": "stdio",>> "%FIREFOX_CONFIG%"
echo   "allowed_origins": [>> "%FIREFOX_CONFIG%"
echo     "chrome-extension://*",>> "%FIREFOX_CONFIG%"
echo     "moz-extension://*">> "%FIREFOX_CONFIG%"
echo   ]>> "%FIREFOX_CONFIG%"
echo }>> "%FIREFOX_CONFIG%"

echo ✅ Configuración de Firefox creada en: %FIREFOX_CONFIG%

echo.
echo 🎉 ¡Host nativo instalado exitosamente!
echo.
echo 📋 Pasos siguientes:
echo 1. Instala la extensión del navegador desde la carpeta browser-extension
echo 2. Reinicia tu navegador
echo 3. La extensión debería conectarse automáticamente a AlohoPass
echo.
echo 🔧 Para desinstalar, ejecuta:
echo    del "%CHROME_CONFIG%"
echo    del "%FIREFOX_CONFIG%"
echo.
echo 🔐 AlohoPass - Tu gestor de contraseñas seguro y privado
pause
