/**
 * JavaScript del popup de AlohoPass
 * Maneja la interfaz del usuario y la comunicación con el background script
 */

class AlohoPassPopup {
    constructor() {
        this.isConnected = false;
        this.stats = {};
        this.init();
    }

    init() {
        console.log('🔐 AlohoPass: Popup iniciado');
        this.setupEventListeners();
        this.checkConnection();
        this.loadStats();
    }

    /**
     * Configura los event listeners de los botones
     */
    setupEventListeners() {
        // Botón para abrir la aplicación
        document.getElementById('openApp').addEventListener('click', () => {
            this.openAlohoPassApp();
        });

        // Botón para sincronizar
        document.getElementById('syncNow').addEventListener('click', () => {
            this.syncNow();
        });

        // Botón para buscar contraseñas
        document.getElementById('searchPasswords').addEventListener('click', () => {
            this.searchPasswords();
        });

        // Botón de configuración
        document.getElementById('settings').addEventListener('click', () => {
            this.openSettings();
        });
    }

    /**
     * Verifica la conexión con la aplicación Tauri
     */
    async checkConnection() {
        try {
            const response = await this.sendMessage({ action: 'checkConnection' });
            this.isConnected = response.connected;
            this.updateConnectionStatus();
        } catch (error) {
            console.error('🔐 AlohoPass: Error al verificar conexión:', error);
            this.isConnected = false;
            this.updateConnectionStatus();
        }
    }

    /**
     * Actualiza el estado de conexión en la UI
     */
    updateConnectionStatus() {
        const status = document.getElementById('status');
        const statusIcon = document.getElementById('statusIcon');
        const statusText = document.getElementById('statusText');

        if (this.isConnected) {
            status.className = 'status connected';
            statusIcon.textContent = '✅';
            statusText.textContent = 'Conectado a AlohoPass';
        } else {
            status.className = 'status disconnected';
            statusIcon.textContent = '❌';
            statusText.textContent = 'No conectado a AlohoPass';
        }

        // Mostrar/ocultar estadísticas según el estado de conexión
        const stats = document.getElementById('stats');
        if (this.isConnected) {
            stats.style.display = 'block';
        } else {
            stats.style.display = 'none';
        }
    }

    /**
     * Carga las estadísticas de la aplicación
     */
    async loadStats() {
        if (!this.isConnected) return;

        try {
            // Aquí podrías obtener estadísticas reales de Tauri
            this.stats = {
                totalPasswords: 42,
                lastSync: 'Hace 5 minutos',
                connectedDevices: 2
            };

            this.updateStatsDisplay();
        } catch (error) {
            console.error('🔐 AlohoPass: Error al cargar estadísticas:', error);
        }
    }

    /**
     * Actualiza la visualización de estadísticas
     */
    updateStatsDisplay() {
        document.getElementById('totalPasswords').textContent = this.stats.totalPasswords || '-';
        document.getElementById('lastSync').textContent = this.stats.lastSync || '-';
        document.getElementById('connectedDevices').textContent = this.stats.connectedDevices || '-';
    }

    /**
     * Abre la aplicación AlohoPass
     */
    openAlohoPassApp() {
        try {
            // Intentar abrir usando protocolo personalizado
            window.open('alohopass://open', '_blank');
            
            // Como fallback, abrir la aplicación local
            chrome.tabs.create({
                url: 'http://localhost:5175'
            });
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al abrir la aplicación:', error);
            
            // Fallback: abrir en nueva pestaña
            chrome.tabs.create({
                url: 'http://localhost:5175'
            });
        }
    }

    /**
     * Inicia la sincronización
     */
    async syncNow() {
        if (!this.isConnected) {
            this.showNotification('No conectado a AlohoPass', 'error');
            return;
        }

        try {
            const button = document.getElementById('syncNow');
            const originalText = button.innerHTML;
            
            // Mostrar estado de carga
            button.innerHTML = '<span class="loading"></span> Sincronizando...';
            button.disabled = true;

            // Enviar mensaje para sincronizar
            const response = await this.sendMessage({ action: 'syncNow' });
            
            if (response.success) {
                this.showNotification('Sincronización completada', 'success');
                this.loadStats(); // Recargar estadísticas
            } else {
                this.showNotification('Error en la sincronización', 'error');
            }
            
        } catch (error) {
            console.error('🔐 AlohoPass: Error al sincronizar:', error);
            this.showNotification('Error al sincronizar', 'error');
        } finally {
            // Restaurar botón
            const button = document.getElementById('syncNow');
            button.innerHTML = originalText;
            button.disabled = false;
        }
    }

    /**
     * Abre la búsqueda de contraseñas
     */
    searchPasswords() {
        if (!this.isConnected) {
            this.showNotification('No conectado a AlohoPass', 'error');
            return;
        }

        // Crear modal de búsqueda
        this.createSearchModal();
    }

    /**
     * Crea el modal de búsqueda de contraseñas
     */
    createSearchModal() {
        const modal = document.createElement('div');
        modal.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.8);
            z-index: 10000;
            display: flex;
            align-items: center;
            justify-content: center;
        `;

        modal.innerHTML = `
            <div style="
                background: white;
                color: #333;
                padding: 20px;
                border-radius: 8px;
                width: 90%;
                max-width: 400px;
            ">
                <h3 style="margin-bottom: 15px;">🔍 Buscar contraseñas</h3>
                
                <input type="text" id="searchInput" placeholder="Buscar por sitio, usuario, email..." 
                       style="
                           width: 100%;
                           padding: 10px;
                           border: 1px solid #ddd;
                           border-radius: 4px;
                           margin-bottom: 15px;
                       "
                >
                
                <div id="searchResults" style="max-height: 200px; overflow-y: auto;"></div>
                
                <div style="margin-top: 15px; text-align: right;">
                    <button id="closeSearch" style="
                        padding: 8px 16px;
                        background: #6b7280;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    ">Cerrar</button>
                </div>
            </div>
        `;

        document.body.appendChild(modal);

        // Configurar funcionalidad
        const searchInput = modal.querySelector('#searchInput');
        const closeBtn = modal.querySelector('#closeSearch');
        const resultsDiv = modal.querySelector('#searchResults');

        // Event listener para búsqueda en tiempo real
        searchInput.addEventListener('input', async (e) => {
            const query = e.target.value.trim();
            if (query.length > 2) {
                await this.performSearch(query, resultsDiv);
            } else {
                resultsDiv.innerHTML = '';
            }
        });

        // Event listener para cerrar
        closeBtn.addEventListener('click', () => {
            document.body.removeChild(modal);
        });

        // Cerrar al hacer click fuera
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });

        // Focus en el input
        searchInput.focus();
    }

    /**
     * Realiza la búsqueda de contraseñas
     */
    async performSearch(query, resultsDiv) {
        try {
            const response = await this.sendMessage({
                action: 'searchPasswords',
                query: query
            });

            if (response.success && response.passwords) {
                this.displaySearchResults(response.passwords, resultsDiv);
            } else {
                resultsDiv.innerHTML = '<p style="color: #666; text-align: center;">No se encontraron resultados</p>';
            }
        } catch (error) {
            console.error('🔐 AlohoPass: Error en la búsqueda:', error);
            resultsDiv.innerHTML = '<p style="color: #ef4444; text-align: center;">Error en la búsqueda</p>';
        }
    }

    /**
     * Muestra los resultados de búsqueda
     */
    displaySearchResults(passwords, resultsDiv) {
        if (passwords.length === 0) {
            resultsDiv.innerHTML = '<p style="color: #666; text-align: center;">No se encontraron resultados</p>';
            return;
        }

        const resultsHTML = passwords.map(password => `
            <div style="
                padding: 10px;
                border: 1px solid #eee;
                border-radius: 4px;
                margin-bottom: 8px;
                cursor: pointer;
                transition: background 0.2s;
            " onmouseover="this.style.background='#f9fafb'" onmouseout="this.style.background='white'">
                <div style="font-weight: bold; margin-bottom: 4px;">${password.title}</div>
                <div style="font-size: 12px; color: #666;">${password.username}</div>
                <div style="font-size: 12px; color: #666;">${password.domain}</div>
            </div>
        `).join('');

        resultsDiv.innerHTML = resultsHTML;
    }

    /**
     * Abre la configuración
     */
    openSettings() {
        // Crear modal de configuración
        this.createSettingsModal();
    }

    /**
     * Crea el modal de configuración
     */
    createSettingsModal() {
        const modal = document.createElement('div');
        modal.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.8);
            z-index: 10000;
            display: flex;
            align-items: center;
            justify-content: center;
        `;

        modal.innerHTML = `
            <div style="
                background: white;
                color: #333;
                padding: 20px;
                border-radius: 8px;
                width: 90%;
                max-width: 400px;
            ">
                <h3 style="margin-bottom: 15px;">⚙️ Configuración</h3>
                
                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px;">Auto-completar formularios:</label>
                    <input type="checkbox" id="autoFillEnabled" checked>
                </div>
                
                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px;">Mostrar indicadores:</label>
                    <input type="checkbox" id="showIndicators" checked>
                </div>
                
                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px;">Sincronización automática:</label>
                    <input type="checkbox" id="autoSync" checked>
                </div>
                
                <div style="margin-top: 20px; text-align: right;">
                    <button id="saveSettings" style="
                        padding: 8px 16px;
                        background: #10b981;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                        margin-right: 10px;
                    ">Guardar</button>
                    <button id="closeSettings" style="
                        padding: 8px 16px;
                        background: #6b7280;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    ">Cerrar</button>
                </div>
            </div>
        `;

        document.body.appendChild(modal);

        // Configurar funcionalidad
        const saveBtn = modal.querySelector('#saveSettings');
        const closeBtn = modal.querySelector('#closeSettings');

        // Event listeners
        saveBtn.addEventListener('click', () => {
            this.saveSettings(modal);
        });

        closeBtn.addEventListener('click', () => {
            document.body.removeChild(modal);
        });

        // Cerrar al hacer click fuera
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });
    }

    /**
     * Guarda la configuración
     */
    saveSettings(modal) {
        const autoFillEnabled = modal.querySelector('#autoFillEnabled').checked;
        const showIndicators = modal.querySelector('#showIndicators').checked;
        const autoSync = modal.querySelector('#autoSync').checked;

        // Guardar en chrome.storage
        chrome.storage.sync.set({
            autoFillEnabled,
            showIndicators,
            autoSync
        }, () => {
            this.showNotification('Configuración guardada', 'success');
            document.body.removeChild(modal);
        });
    }

    /**
     * Envía un mensaje al background script
     */
    sendMessage(message) {
        return new Promise((resolve, reject) => {
            chrome.runtime.sendMessage(message, (response) => {
                if (chrome.runtime.lastError) {
                    reject(new Error(chrome.runtime.lastError.message));
                } else {
                    resolve(response);
                }
            });
        });
    }

    /**
     * Muestra una notificación
     */
    showNotification(message, type = 'info') {
        const notification = document.createElement('div');
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 12px 20px;
            border-radius: 4px;
            color: white;
            font-weight: 500;
            z-index: 10001;
            animation: slideIn 0.3s ease;
        `;

        // Estilo según el tipo
        switch (type) {
            case 'success':
                notification.style.background = '#10b981';
                break;
            case 'error':
                notification.style.background = '#ef4444';
                break;
            default:
                notification.style.background = '#3b82f6';
        }

        notification.textContent = message;
        document.body.appendChild(notification);

        // Remover después de 3 segundos
        setTimeout(() => {
            if (notification.parentNode) {
                notification.style.animation = 'slideOut 0.3s ease';
                setTimeout(() => {
                    if (notification.parentNode) {
                        document.body.removeChild(notification);
                    }
                }, 300);
            }
        }, 3000);
    }
}

// Inicializar el popup cuando se carga
document.addEventListener('DOMContentLoaded', () => {
    new AlohoPassPopup();
});

// Agregar estilos de animación
const style = document.createElement('style');
style.textContent = `
    @keyframes slideIn {
        from { transform: translateX(100%); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
    }
    
    @keyframes slideOut {
        from { transform: translateX(0); opacity: 1; }
        to { transform: translateX(100%); opacity: 0; }
    }
`;
document.head.appendChild(style);
