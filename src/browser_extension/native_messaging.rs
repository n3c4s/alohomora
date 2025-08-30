use crate::browser_extension::protocol::*;
use crate::sync::SyncManager;
use log::{info, error, warn};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

/// Gestor de la extensión del navegador
pub struct BrowserExtensionManager {
    is_running: Arc<Mutex<bool>>,
    sync_manager: Arc<Mutex<Option<SyncManager>>>,
    config: PluginConfig,
    connections: Arc<Mutex<HashMap<String, std::net::TcpStream>>>,
}

impl BrowserExtensionManager {
    /// Crear una nueva instancia del gestor
    pub fn new(sync_manager: Arc<Mutex<Option<SyncManager>>>) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            sync_manager,
            config: PluginConfig::default(),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Iniciar el gestor de extensiones
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔌 AlohoPass: Iniciando gestor de extensiones del navegador");
        
        let is_running = self.is_running.clone();
        let connections = self.connections.clone();
        let sync_manager = self.sync_manager.clone();
        let config = self.config.clone();

        // Iniciar en un hilo separado para no bloquear
        thread::spawn(move || {
            if let Err(e) = Self::run_native_host(is_running, connections, sync_manager, config) {
                error!("🔌 AlohoPass: Error en el host nativo: {}", e);
            }
        });

        *self.is_running.lock().unwrap() = true;
        info!("🔌 AlohoPass: Gestor de extensiones iniciado");
        
        Ok(())
    }

    /// Detener el gestor
    pub fn stop(&mut self) {
        info!("🔌 AlohoPass: Deteniendo gestor de extensiones");
        *self.is_running.lock().unwrap() = false;
    }

    /// Ejecutar el host nativo
    fn run_native_host(
        is_running: Arc<Mutex<bool>>,
        connections: Arc<Mutex<HashMap<String, std::net::TcpStream>>>,
        sync_manager: Arc<Mutex<Option<SyncManager>>>,
        config: PluginConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Por ahora, simulamos el host nativo
        // En una implementación real, esto sería un servidor TCP o pipe nombrado
        
        info!("🔌 AlohoPass: Host nativo iniciado (modo simulado)");
        
        // Simular conexiones entrantes
        while *is_running.lock().unwrap() {
            thread::sleep(Duration::from_secs(5));
            
            // Simular mensajes de prueba
            if let Ok(mut conns) = connections.lock() {
                if conns.is_empty() {
                    info!("🔌 AlohoPass: Esperando conexiones del plugin...");
                }
            }
        }
        
        info!("🔌 AlohoPass: Host nativo detenido");
        Ok(())
    }

    /// Manejar mensaje del plugin
    pub async fn handle_message(&self, message: BrowserMessage) -> BrowserResponse {
        info!("🔌 AlohoPass: Mensaje recibido del plugin: {:?}", message);
        
        match message {
            BrowserMessage::ConnectionStatus => {
                self.handle_connection_status().await
            }
            
            BrowserMessage::GetPasswords { domain, form_type } => {
                self.handle_get_passwords(domain, form_type).await
            }
            
            BrowserMessage::CreatePassword { entry } => {
                self.handle_create_password(entry).await
            }
            
            BrowserMessage::SearchPasswords { query } => {
                self.handle_search_passwords(query).await
            }
            
            BrowserMessage::SyncNow => {
                self.handle_sync_now().await
            }
            
            BrowserMessage::GetStats => {
                self.handle_get_stats().await
            }
        }
    }

    /// Manejar verificación de estado de conexión
    async fn handle_connection_status(&self) -> BrowserResponse {
        let is_connected = *self.is_running.lock().unwrap();
        
        let status = serde_json::json!({
            "connected": is_connected,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        BrowserResponse::success(status)
    }

    /// Manejar solicitud de contraseñas
    async fn handle_get_passwords(&self, domain: String, _form_type: FormType) -> BrowserResponse {
        info!("🔌 AlohoPass: Solicitando contraseñas para dominio: {}", domain);
        
        // Por ahora, retornar contraseñas de ejemplo
        // En una implementación real, esto consultaría la base de datos
        let passwords = vec![
            BrowserPassword {
                id: "1".to_string(),
                title: "Cuenta principal".to_string(),
                username: "usuario@ejemplo.com".to_string(),
                email: Some("usuario@ejemplo.com".to_string()),
                url: format!("https://{}", domain),
                domain: domain.clone(),
                category: Some("Personal".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            BrowserPassword {
                id: "2".to_string(),
                title: "Cuenta de trabajo".to_string(),
                username: "trabajo@empresa.com".to_string(),
                email: Some("trabajo@empresa.com".to_string()),
                url: format!("https://{}", domain),
                domain,
                category: Some("Trabajo".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        ];
        
        let data = serde_json::json!({
            "passwords": passwords,
            "domain": domain,
            "count": passwords.len()
        });
        
        BrowserResponse::success(data)
    }

    /// Manejar creación de nueva contraseña
    async fn handle_create_password(&self, entry: PasswordEntry) -> BrowserResponse {
        info!("🔌 AlohoPass: Creando nueva contraseña para: {}", entry.title);
        
        // Aquí se integraría con el sistema de contraseñas de Tauri
        // Por ahora, simulamos éxito
        
        let data = serde_json::json!({
            "message": "Contraseña creada exitosamente",
            "entry_id": "new_id_123",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        BrowserResponse::success(data)
    }

    /// Manejar búsqueda de contraseñas
    async fn handle_search_passwords(&self, query: String) -> BrowserResponse {
        info!("🔌 AlohoPass: Buscando contraseñas con query: {}", query);
        
        // Simular búsqueda
        let passwords = vec![
            BrowserPassword {
                id: "1".to_string(),
                title: "Cuenta principal".to_string(),
                username: "usuario@ejemplo.com".to_string(),
                email: Some("usuario@ejemplo.com".to_string()),
                url: "https://ejemplo.com".to_string(),
                domain: "ejemplo.com".to_string(),
                category: Some("Personal".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }
        ];
        
        let data = serde_json::json!({
            "passwords": passwords,
            "query": query,
            "count": passwords.len()
        });
        
        BrowserResponse::success(data)
    }

    /// Manejar sincronización manual
    async fn handle_sync_now(&self) -> BrowserResponse {
        info!("🔌 AlohoPass: Sincronización manual solicitada por el plugin");
        
        // Aquí se integraría con el SyncManager
        // Por ahora, simulamos éxito
        
        let data = serde_json::json!({
            "message": "Sincronización iniciada",
            "status": "running",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        BrowserResponse::success(data)
    }

    /// Manejar solicitud de estadísticas
    async fn handle_get_stats(&self) -> BrowserResponse {
        info!("🔌 AlohoPass: Estadísticas solicitadas por el plugin");
        
        let stats = BrowserStats {
            total_passwords: 42,
            last_sync: Some("Hace 5 minutos".to_string()),
            connected_devices: 2,
            sync_status: "Conectado".to_string(),
        };
        
        let data = serde_json::to_value(stats)
            .unwrap_or_else(|_| serde_json::json!({"error": "Error serializando estadísticas"}));
        
        BrowserResponse::success(data)
    }

    /// Enviar evento al plugin
    pub async fn send_event(&self, event: TauriEvent) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔌 AlohoPass: Enviando evento al plugin: {:?}", event);
        
        // En una implementación real, esto enviaría el evento a todas las conexiones activas
        // Por ahora, solo logueamos
        
        Ok(())
    }

    /// Obtener configuración del plugin
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    /// Actualizar configuración del plugin
    pub fn update_config(&mut self, new_config: PluginConfig) {
        info!("🔌 AlohoPass: Actualizando configuración del plugin");
        self.config = new_config;
    }
}

impl Drop for BrowserExtensionManager {
    fn drop(&mut self) {
        self.stop();
    }
}
