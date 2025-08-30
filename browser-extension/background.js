/**
 * Background Script para AlohoPass
 * Maneja la comunicación con la aplicación Tauri
 */

class AlohoPassBackground {
    constructor() {
        this.isConnected = false;
        this.connectionPort = null;
        this.init();
    }

    init() {
        console.log('🔐 AlohoPass: Background script iniciado');
        this.setupMessageListeners();
        this.connectToTauriApp();
    }

    /**
     * Configura los listeners de mensajes del content script
     */
    setupMessageListeners() {
        chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
            console.log('🔐 AlohoPass: Mensaje recibido:', request.action);
            
            switch (request.action) {
                case 'checkConnection':
                    sendResponse({ connected: this.isConnected });
                    break;
                    
                case 'getPasswords':
                    this.getPasswords(request.domain, request.formType, sendResponse);
                    return true; // Indica que la respuesta será asíncrona
                    
                case 'createPassword':
                    this.createPassword(request.entry, sendResponse);
                    return true;
                    
                case 'searchPasswords':
                    this.searchPasswords(request.query, sendResponse);
                    return true;
                    
                default:
                    sendResponse({ success: false, error: 'Acción no reconocida' });
            }
        });
    }

    /**
     * Conecta con la aplicación Tauri usando Native Messaging
     */
    connectToTauriApp() {
        try {
            // Intentar conectar usando Native Messaging
            this.connectionPort = chrome.runtime.connectNative('com.alohopass.browser');
            
            this.connectionPort.onMessage.addListener((message) => {
                console.log('🔐 AlohoPass: Mensaje de Tauri recibido:', message);
                this.handleTauriMessage(message);
            });
            
            this.connectionPort.onDisconnect.addListener(() => {
                console.log('🔐 AlohoPass: Conexión con Tauri perdida');
                this.isConnected = false;
                this.connectionPort = null;
                
                // Intentar reconectar después de un delay
                setTimeout(() => {
                    this.connectToTauriApp();
                }, 5000);
            });
            
            this.isConnected = true;
            console.log('🔐 AlohoPass: Conectado a la aplicación Tauri');
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al conectar con Tauri:', error);
            this.isConnected = false;
            
            // Intentar conectar usando protocolo personalizado como fallback
            this.connectUsingCustomProtocol();
        }
    }

    /**
     * Conecta usando protocolo personalizado como fallback
     */
    connectUsingCustomProtocol() {
        try {
            // Crear un iframe oculto para comunicarse con la app Tauri
            const iframe = document.createElement('iframe');
            iframe.style.display = 'none';
            iframe.src = 'alohopass://connect';
            document.body.appendChild(iframe);
            
            // Escuchar mensajes del iframe
            window.addEventListener('message', (event) => {
                if (event.origin === 'alohopass://') {
                    this.handleTauriMessage(event.data);
                }
            });
            
            this.isConnected = true;
            console.log('🔐 AlohoPass: Conectado usando protocolo personalizado');
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al conectar usando protocolo personalizado:', error);
            this.isConnected = false;
        }
    }

    /**
     * Maneja mensajes recibidos de la aplicación Tauri
     */
    handleTauriMessage(message) {
        switch (message.type) {
            case 'connection_status':
                this.isConnected = message.connected;
                break;
                
            case 'passwords_updated':
                // Notificar a las pestañas activas sobre la actualización
                this.notifyTabsAboutUpdate();
                break;
                
            case 'error':
                console.error('🔐 AlohoPass: Error de Tauri:', message.error);
                break;
                
            default:
                console.log('🔐 AlohoPass: Mensaje de Tauri no reconocido:', message);
        }
    }

    /**
     * Obtiene contraseñas para un dominio específico
     */
    async getPasswords(domain, formType, sendResponse) {
        if (!this.isConnected) {
            sendResponse({ 
                success: false, 
                error: 'No conectado a la aplicación Tauri',
                passwords: []
            });
            return;
        }

        try {
            // Enviar mensaje a Tauri para obtener contraseñas
            const message = {
                type: 'get_passwords',
                domain: domain,
                formType: formType
            };

            if (this.connectionPort) {
                this.connectionPort.postMessage(message);
                
                // Esperar respuesta (en una implementación real, esto sería más sofisticado)
                setTimeout(() => {
                    // Por ahora, retornar datos de ejemplo
                    sendResponse({
                        success: true,
                        passwords: this.getMockPasswords(domain)
                    });
                }, 100);
            } else {
                sendResponse({
                    success: false,
                    error: 'Puerto de conexión no disponible',
                    passwords: []
                });
            }
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al obtener contraseñas:', error);
            sendResponse({
                success: false,
                error: error.message,
                passwords: []
            });
        }
    }

    /**
     * Crea una nueva contraseña
     */
    async createPassword(entry, sendResponse) {
        if (!this.isConnected) {
            sendResponse({ 
                success: false, 
                error: 'No conectado a la aplicación Tauri'
            });
            return;
        }

        try {
            const message = {
                type: 'create_password',
                entry: entry
            };

            if (this.connectionPort) {
                this.connectionPort.postMessage(message);
                
                // Simular respuesta exitosa
                setTimeout(() => {
                    sendResponse({ success: true });
                }, 100);
            } else {
                sendResponse({
                    success: false,
                    error: 'Puerto de conexión no disponible'
                });
            }
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al crear contraseña:', error);
            sendResponse({
                success: false,
                error: error.message
            });
        }
    }

    /**
     * Busca contraseñas por query
     */
    async searchPasswords(query, sendResponse) {
        if (!this.isConnected) {
            sendResponse({ 
                success: false, 
                error: 'No conectado a la aplicación Tauri',
                passwords: []
            });
            return;
        }

        try {
            const message = {
                type: 'search_passwords',
                query: query
            };

            if (this.connectionPort) {
                this.connectionPort.postMessage(message);
                
                // Simular respuesta
                setTimeout(() => {
                    sendResponse({
                        success: true,
                        passwords: this.getMockPasswords('*')
                    });
                }, 100);
            } else {
                sendResponse({
                    success: false,
                    error: 'Puerto de conexión no disponible',
                    passwords: []
                });
            }
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al buscar contraseñas:', error);
            sendResponse({
                success: false,
                error: error.message,
                passwords: []
            });
        }
    }

    /**
     * Notifica a las pestañas activas sobre actualizaciones
     */
    notifyTabsAboutUpdate() {
        chrome.tabs.query({ active: true }, (tabs) => {
            tabs.forEach(tab => {
                chrome.tabs.sendMessage(tab.id, {
                    action: 'passwordsUpdated'
                }).catch(() => {
                    // Ignorar errores si la pestaña no tiene el content script
                });
            });
        });
    }

    /**
     * Obtiene contraseñas de ejemplo para testing
     */
    getMockPasswords(domain) {
        return [
            {
                id: '1',
                title: 'Cuenta principal',
                username: 'usuario@ejemplo.com',
                password: '********',
                email: 'usuario@ejemplo.com',
                url: `https://${domain}`,
                domain: domain,
                category: 'Personal',
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            },
            {
                id: '2',
                title: 'Cuenta de trabajo',
                username: 'trabajo@empresa.com',
                password: '********',
                email: 'trabajo@empresa.com',
                url: `https://${domain}`,
                domain: domain,
                category: 'Trabajo',
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            }
        ];
    }
}

// Inicializar el background script
new AlohoPassBackground();
