use crate::browser_extension::protocol::*;
use crate::sync::SyncManager;
use log::{info, error, warn};
use serde_json;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

/// Gestor de la extensión del navegador
#[derive(Clone)]
pub struct BrowserExtensionManager {
    is_running: Arc<Mutex<bool>>,
    sync_manager: Arc<Mutex<Option<SyncManager>>>,
    config: PluginConfig,
    connections: Arc<Mutex<HashMap<String, TcpStream>>>,
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

    /// Ejecutar el host nativo real
    fn run_native_host(
        is_running: Arc<Mutex<bool>>,
        connections: Arc<Mutex<HashMap<String, TcpStream>>>,
        sync_manager: Arc<Mutex<Option<SyncManager>>>,
        config: PluginConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔌 AlohoPass: Iniciando servidor TCP para Native Messaging");

        // Intentar diferentes puertos si el 12345 está ocupado
        let ports = vec![12345, 12346, 12347, 12348, 12349];
        let mut listener = None;
        let mut selected_port = None;

        for port in ports {
            match TcpListener::bind(format!("127.0.0.1:{}", port)) {
                Ok(l) => {
                    listener = Some(l);
                    selected_port = Some(port);
                    info!("🔌 AlohoPass: Servidor TCP iniciado en 127.0.0.1:{}", port);
                    break;
                }
                Err(e) => {
                    warn!("🔌 AlohoPass: No se pudo usar puerto {}: {}", port, e);
                    continue;
                }
            }
        }

        let listener = listener.ok_or("No se pudo iniciar servidor en ningún puerto")?;
        let selected_port = selected_port.unwrap();

        // Guardar el puerto en un archivo para que el script de conexión lo use
        if let Err(e) = std::fs::write(
            format!("{}/.alohopass_port", std::env::current_dir()?.display()),
            selected_port.to_string()
        ) {
            warn!("🔌 AlohoPass: No se pudo guardar el puerto: {}", e);
        }

        info!("🔌 AlohoPass: Servidor TCP activo en puerto {}", selected_port);

        // Escuchar conexiones entrantes
        for stream in listener.incoming() {
            if !*is_running.lock().unwrap() {
                info!("🔌 AlohoPass: Señal de parada recibida, cerrando servidor");
                break;
            }

            match stream {
                Ok(stream) => {
                    info!("🔌 AlohoPass: Nueva conexión entrante desde {:?}", stream.peer_addr());
                    let stream_id = format!("conn_{}", std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis());
                    
                    // Agregar conexión a la lista
                    if let Ok(mut conns) = connections.lock() {
                        conns.insert(stream_id.clone(), stream.try_clone()?);
                        info!("🔌 AlohoPass: Conexión {} agregada, total: {}", stream_id, conns.len());
                    }

                    // Manejar la conexión en un hilo separado
                    let stream_id_clone = stream_id.clone();
                    let connections_clone = connections.clone();
                    let sync_manager_clone = sync_manager.clone();
                    let stream_id_for_error = stream_id.clone(); // Clonar para el error
                    
                    thread::spawn(move || {
                        info!("🔌 AlohoPass: Iniciando manejo de conexión {}", stream_id_clone);
                        if let Err(e) = Self::handle_connection(
                            stream,
                            stream_id_clone,
                            connections_clone,
                            sync_manager_clone,
                        ) {
                            error!("🔌 AlohoPass: Error manejando conexión {}: {}", stream_id_for_error, e);
                        }
                    });
                }
                Err(e) => {
                    error!("🔌 AlohoPass: Error aceptando conexión: {}", e);
                }
            }
        }

        info!("🔌 AlohoPass: Servidor TCP detenido");
        Ok(())
    }

    /// Manejar una conexión individual
    fn handle_connection(
        mut stream: TcpStream,
        stream_id: String,
        connections: Arc<Mutex<HashMap<String, TcpStream>>>,
        sync_manager: Arc<Mutex<Option<SyncManager>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔌 AlohoPass: Manejando conexión: {}", stream_id);

        // Configurar timeout para la conexión
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(30)))?;

        // Buffer para leer mensajes
        let mut buffer = [0; 4096];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let message_data = &buffer[..n];
                    
                    // Intentar parsear el mensaje JSON
                    match serde_json::from_slice::<NativeMessage>(message_data) {
                        Ok(native_message) => {
                            info!("🔌 AlohoPass: Mensaje recibido: {:?}", native_message.message);
                            
                            // Procesar el mensaje
                            let response = Self::process_message(native_message.message, &sync_manager);
                            
                            // Enviar respuesta
                            let native_response = NativeResponse {
                                id: native_message.id,
                                response,
                            };
                            
                            let response_json = serde_json::to_vec(&native_response)?;
                            stream.write_all(&response_json)?;
                            stream.flush()?;
                        }
                        Err(e) => {
                            error!("🔌 AlohoPass: Error parseando mensaje: {}", e);
                            break;
                        }
                    }
                }
                Ok(0) => {
                    info!("🔌 AlohoPass: Conexión cerrada por el cliente");
                    break;
                }
                Ok(_) => {
                    // Caso donde n = 0, ya cubierto arriba
                    continue;
                }
                Err(e) => {
                    error!("🔌 AlohoPass: Error leyendo de la conexión: {}", e);
                    break;
                }
            }
        }

        // Remover conexión de la lista
        if let Ok(mut conns) = connections.lock() {
            conns.remove(&stream_id);
        }

        info!("🔌 AlohoPass: Conexión cerrada: {}", stream_id);
        Ok(())
    }

    /// Procesar un mensaje del plugin
    fn process_message(
        message: BrowserMessage,
        sync_manager: &Arc<Mutex<Option<SyncManager>>>,
    ) -> BrowserResponse {
        info!("🔌 AlohoPass: Procesando mensaje: {:?}", message);

        match message {
            BrowserMessage::ConnectionStatus => {
                BrowserResponse::success(serde_json::json!({
                    "connected": true,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            }

            BrowserMessage::GetPasswords { domain, form_type } => {
                info!("🔌 AlohoPass: Solicitando contraseñas para dominio: {}", domain);

                // Por ahora, retornar contraseñas de ejemplo
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
                    }
                ];

                let data = serde_json::json!({
                    "passwords": passwords,
                    "domain": domain,
                    "count": passwords.len()
                });

                BrowserResponse::success(data)
            }

            BrowserMessage::CreatePassword { entry } => {
                info!("🔌 AlohoPass: Creando nueva contraseña para: {}", entry.title);
                BrowserResponse::simple_success()
            }

            BrowserMessage::SearchPasswords { query } => {
                info!("🔌 AlohoPass: Buscando contraseñas con query: {}", query);
                
                let passwords = vec![
                    BrowserPassword {
                        id: "1".to_string(),
                        title: "Resultado de búsqueda".to_string(),
                        username: "usuario@ejemplo.com".to_string(),
                        email: Some("usuario@ejemplo.com".to_string()),
                        url: "https://ejemplo.com".to_string(),
                        domain: "ejemplo.com".to_string(),
                        category: Some("Personal".to_string()),
                        created_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                    }
                ];

                BrowserResponse::success(serde_json::json!({
                    "passwords": passwords,
                    "query": query
                }))
            }

            BrowserMessage::SyncNow => {
                info!("🔌 AlohoPass: Sincronización solicitada");
                BrowserResponse::simple_success()
            }

            BrowserMessage::GetStats => {
                let stats = BrowserStats {
                    total_passwords: 42,
                    last_sync: Some("Hace 5 minutos".to_string()),
                    connected_devices: 1,
                    sync_status: "Conectado".to_string(),
                };

                BrowserResponse::success(serde_json::to_value(stats).unwrap())
            }
        }
    }

    /// Manejar mensaje del plugin (método público para compatibilidad)
    pub async fn handle_message(&self, message: BrowserMessage) -> BrowserResponse {
        Self::process_message(message, &self.sync_manager)
    }

    /// Obtener configuración
    pub fn get_config(&self) -> &PluginConfig {
        &self.config
    }

    /// Actualizar configuración
    pub fn update_config(&mut self, new_config: PluginConfig) {
        self.config = new_config;
    }

    /// Enviar evento al plugin
    pub fn send_event(&self, event: TauriEvent) {
        info!("🔌 AlohoPass: Enviando evento al plugin: {:?}", event);
        // En una implementación real, esto enviaría el evento a todas las conexiones activas
    }
}

impl Drop for BrowserExtensionManager {
    fn drop(&mut self) {
        self.stop();
    }
}
