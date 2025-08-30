//! Gestor principal del sistema de sincronización
//! 
//! Este módulo coordina todos los componentes del sistema de sincronización:
//! - Descubrimiento automático de dispositivos
//! - Conexiones P2P
//! - Sincronización inteligente
//! - Gestión de eventos y estado

use crate::sync::{
    DeviceDiscovery, DeviceInfo, SyncEvent, SyncEventHandler, SyncStatus, SyncConfig,
    SyncMethod, SyncStats, SyncResult, DefaultSyncEventHandler,
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc, RwLock, Mutex},
    time::{interval, timeout},
};
use serde::{Deserialize, Serialize};

/// Gestor principal de sincronización
pub struct SyncManager {
    /// Estado del sistema de sincronización
    status: Arc<RwLock<SyncStatus>>,
    /// Configuración del sistema
    config: Arc<RwLock<SyncConfig>>,
    /// Sistema de descubrimiento
    discovery: Arc<Mutex<Option<DeviceDiscovery>>>,
    /// Dispositivos conectados
    connected_devices: Arc<RwLock<HashMap<String, DeviceInfo>>>,
    /// Estadísticas de sincronización
    stats: Arc<RwLock<SyncStats>>,
    /// Canal para eventos de sincronización
    event_sender: mpsc::Sender<SyncEvent>,
    /// Receptor de eventos
    event_receiver: Option<mpsc::Receiver<SyncEvent>>,
    /// Manejador de eventos
    event_handler: Arc<dyn SyncEventHandler + Send + Sync>,
    /// Estado del gestor
    is_running: Arc<RwLock<bool>>,
    /// Tarea principal del gestor
    manager_task: Option<tokio::task::JoinHandle<()>>,
    /// Tarea de limpieza
    cleanup_task: Option<tokio::task::JoinHandle<()>>,
}

impl SyncManager {
    /// Crear una nueva instancia del gestor
    pub fn new(config: SyncConfig) -> Self {
        let (event_sender, event_receiver) = mpsc::channel(100);
        
        Self {
            status: Arc::new(RwLock::new(SyncStatus::default())),
            config: Arc::new(RwLock::new(config)),
            discovery: Arc::new(Mutex::new(None)),
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(SyncStats::default())),
            event_sender,
            event_receiver: Some(event_receiver),
            event_handler: Arc::new(DefaultSyncEventHandler),
            is_running: Arc::new(RwLock::new(false)),
            manager_task: None,
            cleanup_task: None,
        }
    }

    /// Crear con configuración por defecto
    pub fn new_default() -> Self {
        Self::new(SyncConfig::default())
    }

    /// Iniciar el sistema de sincronización
    pub async fn start(&mut self) -> Result<()> {
        if *self.is_running.read().await {
            return Ok(());
        }

        log::info!("Iniciando sistema de sincronización Alohopass...");

        // Verificar configuración
        {
            let config = self.config.read().await;
            if !config.auto_discovery {
                log::warn!("Descubrimiento automático deshabilitado en la configuración");
            }
        }

        // Inicializar sistema de descubrimiento
        {
            let should_init = {
                let config = self.config.read().await;
                config.auto_discovery
            };
            
            if should_init {
                self.init_discovery().await?;
            }
        }

        // Iniciar tareas principales
        self.start_manager_task().await?;
        self.start_cleanup_task().await?;

        // Marcar como ejecutándose
        *self.is_running.write().await = true;

        // Actualizar estado
        {
            let mut status = self.status.write().await;
            status.is_enabled = true;
        }

        log::info!("Sistema de sincronización iniciado correctamente");
        Ok(())
    }

    /// Detener el sistema de sincronización
    pub async fn stop(&mut self) -> Result<()> {
        if !*self.is_running.read().await {
            return Ok(());
        }

        log::info!("Deteniendo sistema de sincronización...");

        // Detener tareas
        if let Some(task) = self.manager_task.take() {
            task.abort();
        }
        if let Some(task) = self.cleanup_task.take() {
            task.abort();
        }

        // Detener descubrimiento
        if let Some(mut discovery) = self.discovery.lock().await.take() {
            discovery.stop().await?;
        }

        // Marcar como detenido
        *self.is_running.write().await = false;

        // Actualizar estado
        {
            let mut status = self.status.write().await;
            status.is_enabled = false;
        }

        log::info!("Sistema de sincronización detenido correctamente");
        Ok(())
    }

    /// Inicializar el sistema de descubrimiento
    async fn init_discovery(&mut self) -> Result<()> {
        log::info!("Inicializando sistema de descubrimiento...");

        let config = self.config.read().await;
        let discovery_config = crate::sync::discovery::DiscoveryConfig::default();
        
        let mut discovery = DeviceDiscovery::new(discovery_config, self.event_sender.clone());
        discovery.start().await?;

        // No necesitamos establecer manejador de eventos personalizado
        // discovery.set_event_handler(Box::new(DiscoveryEventHandler { event_sender }));

        *self.discovery.lock().await = Some(discovery);

        log::info!("Sistema de descubrimiento inicializado correctamente");
        Ok(())
    }

    /// Iniciar la tarea principal del gestor
    async fn start_manager_task(&mut self) -> Result<()> {
        let event_receiver = self.event_receiver.take().unwrap();
        let event_handler = self.event_handler.clone();
        let connected_devices = self.connected_devices.clone();
        let stats = self.stats.clone();
        let status = self.status.clone();

        let task = tokio::spawn(async move {
            let mut receiver = event_receiver;
            
            while let Some(event) = receiver.recv().await {
                // Manejar evento
                if let Err(e) = event_handler.handle_sync_event(event.clone()).await {
                    log::error!("Error al manejar evento: {}", e);
                }

                // Procesar evento localmente
                if let Err(e) = Self::process_event_locally(
                    event,
                    &connected_devices,
                    &stats,
                    &status
                ).await {
                    log::error!("Error al procesar evento localmente: {}", e);
                }
            }
        });

        self.manager_task = Some(task);
        Ok(())
    }

    /// Iniciar la tarea de limpieza
    async fn start_cleanup_task(&mut self) -> Result<()> {
        let _config = self.config.clone();
        let discovery = self.discovery.clone();
        let connected_devices = self.connected_devices.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Cada minuto
            
            loop {
                interval.tick().await;
                
                // Limpiar dispositivos antiguos
                if let Some(discovery) = discovery.lock().await.as_mut() {
                    if let Err(e) = discovery.cleanup_old_devices(Duration::from_secs(300)).await {
                        log::error!("Error al limpiar dispositivos antiguos: {}", e);
                    }
                }

                // Limpiar dispositivos desconectados
                {
                    let mut devices = connected_devices.write().await;
                    devices.retain(|_, device| {
                        if let Some(last_seen) = device.last_seen {
                            let age = chrono::Utc::now() - last_seen;
                            age < chrono::Duration::from_std(Duration::from_secs(600)).unwrap_or_default()
                        } else {
                            false // Si no hay last_seen, eliminar el dispositivo
                        }
                    });
                }
            }
        });

        self.cleanup_task = Some(task);
        Ok(())
    }

    /// Procesar evento localmente
    async fn process_event_locally(
        event: SyncEvent,
        connected_devices: &Arc<RwLock<HashMap<String, DeviceInfo>>>,
        stats: &Arc<RwLock<SyncStats>>,
        status: &Arc<RwLock<SyncStatus>>,
    ) -> Result<()> {
        match event {
            SyncEvent::DeviceDiscovered(device) => {
                log::info!("Dispositivo descubierto: {} ({})", device.name, device.device_type.display_name());
                
                // Actualizar estadísticas
                let mut stats = stats.write().await;
                stats.total_syncs += 1;
            }
            SyncEvent::DeviceConnected(device) => {
                log::info!("Dispositivo conectado: {} ({})", device.name, device.device_type.display_name());
                
                // Agregar a dispositivos conectados
                let device_id = device.id.clone();
                let mut devices = connected_devices.write().await;
                devices.insert(device_id.clone(), device.clone());
                
                // Actualizar estado
                let mut status = status.write().await;
                status.connected_devices.push(device);
                
                // Actualizar estadísticas
                let mut stats = stats.write().await;
                stats.devices_synced_with.push(device_id);
            }
            SyncEvent::DeviceDisconnected(device) => {
                log::info!("Dispositivo desconectado: {} ({})", device.name, device.device_type.display_name());
                
                // Remover de dispositivos conectados
                let mut devices = connected_devices.write().await;
                devices.remove(&device.id);
                
                // Actualizar estado
                let mut status = status.write().await;
                status.connected_devices.retain(|d| d.id != device.id);
            }
            SyncEvent::SyncStarted(device) => {
                log::info!("Sincronización iniciada con: {} ({})", device.name, device.device_type.display_name());
                
                // Actualizar estado del dispositivo
                if let Some(device) = connected_devices.write().await.get_mut(&device.id) {
                    device.update_status(crate::sync::DeviceStatus::Syncing);
                }
            }
            SyncEvent::SyncCompleted(device, count) => {
                log::info!("Sincronización completada con: {} ({} elementos)", device.name, count);
                
                // Actualizar estado del dispositivo
                if let Some(device) = connected_devices.write().await.get_mut(&device.id) {
                    device.mark_synced();
                }
                
                // Actualizar estado general
                let mut status = status.write().await;
                status.last_sync = Some(chrono::Utc::now());
                
                // Actualizar estadísticas
                let mut stats = stats.write().await;
                stats.successful_syncs += 1;
                stats.total_syncs += 1;
            }
            SyncEvent::SyncFailed(device, error) => {
                log::error!("Sincronización falló con: {} - Error: {}", device.name, error);
                
                // Actualizar estado del dispositivo
                if let Some(device) = connected_devices.write().await.get_mut(&device.id) {
                    device.update_status(crate::sync::DeviceStatus::Error(error.clone()));
                }
                
                // Actualizar estadísticas
                let mut stats = stats.write().await;
                stats.failed_syncs += 1;
                stats.total_syncs += 1;
            }
            SyncEvent::ChangesDetected(count) => {
                log::info!("Cambios detectados: {} elementos", count);
                
                // Actualizar estadísticas
                let mut stats = stats.write().await;
                stats.total_data_synced += count;
            }
            SyncEvent::Heartbeat => {
                log::debug!("Heartbeat recibido");
                // No necesitamos hacer nada especial para el heartbeat
            }
        }
        Ok(())
    }

    /// Obtener el estado del sistema
    pub async fn get_status(&self) -> SyncStatus {
        self.status.read().await.clone()
    }

    /// Obtener la configuración del sistema
    pub async fn get_config(&self) -> SyncConfig {
        self.config.read().await.clone()
    }

    /// Actualizar la configuración
    pub async fn update_config(&self, new_config: SyncConfig) -> Result<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }

    /// Obtener dispositivos conectados
    pub async fn get_connected_devices(&self) -> Vec<DeviceInfo> {
        let devices = self.connected_devices.read().await;
        devices.values().cloned().collect()
    }

    /// Obtener dispositivos descubiertos
    pub async fn get_discovered_devices(&self) -> Vec<DeviceInfo> {
        if let Some(discovery) = self.discovery.lock().await.as_ref() {
            discovery.get_discovered_devices().await
        } else {
            Vec::new()
        }
    }

    /// Buscar dispositivos
    pub async fn search_devices(&self, query: &str) -> Vec<DeviceInfo> {
        if let Some(discovery) = self.discovery.lock().await.as_ref() {
            let devices = discovery.get_discovered_devices().await;
            let query_lower = query.to_lowercase();
            
            devices.into_iter()
                .filter(|device| {
                    device.name.to_lowercase().contains(&query_lower) ||
                    device.os.to_lowercase().contains(&query_lower) ||
                    device.device_type.to_string().to_lowercase().contains(&query_lower)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Obtener estadísticas
    pub async fn get_stats(&self) -> SyncStats {
        self.stats.read().await.clone()
    }

    /// Conectar a un dispositivo
    pub async fn connect_to_device(&self, device_id: &str) -> Result<()> {
        // TODO: Implementar conexión P2P
        log::info!("Conectando a dispositivo: {}", device_id);
        Ok(())
    }

    /// Desconectar de un dispositivo
    pub async fn disconnect_from_device(&self, device_id: &str) -> Result<()> {
        // TODO: Implementar desconexión
        log::info!("Desconectando de dispositivo: {}", device_id);
        Ok(())
    }

    /// Sincronizar con un dispositivo
    pub async fn sync_with_device(&self, device_id: &str) -> Result<SyncResult> {
        // TODO: Implementar sincronización
        log::info!("Sincronizando con dispositivo: {}", device_id);
        
        Ok(SyncResult::success(
            device_id.to_string(),
            0,
            0,
            0,
        ))
    }

    /// Sincronizar con todos los dispositivos
    pub async fn sync_all_devices(&self) -> Result<Vec<SyncResult>> {
        let devices = self.get_connected_devices().await;
        let mut results = Vec::new();

        for device in devices {
            if device.is_available_for_sync() {
                match self.sync_with_device(&device.id).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        results.push(SyncResult::failure(
                            device.id.clone(),
                            e.to_string(),
                        ));
                    }
                }
            }
        }

        Ok(results)
    }

    /// Establecer manejador de eventos personalizado
    pub fn set_event_handler(&mut self, handler: Box<dyn SyncEventHandler + Send + Sync>) {
        self.event_handler = Arc::from(handler);
    }

    /// Verificar si está ejecutándose
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Obtener información del sistema
    pub async fn get_system_info(&self) -> SystemInfo {
        let status = self.status.read().await;
        let config = self.config.read().await;
        let stats = self.stats.read().await;
        let connected_count = self.connected_devices.read().await.len();
        let discovered_count = if let Some(discovery) = self.discovery.lock().await.as_ref() {
            discovery.get_discovered_devices().await.len()
        } else {
            0
        };

        SystemInfo {
            is_running: *self.is_running.read().await,
            is_enabled: status.is_enabled,
            sync_method: status.sync_method.clone(),
            auto_sync: status.auto_sync,
            auto_discovery: config.auto_discovery,
            connected_devices: connected_count,
            discovered_devices: discovered_count,
            total_syncs: stats.total_syncs,
            successful_syncs: stats.successful_syncs,
            failed_syncs: stats.failed_syncs,
            last_sync: status.last_sync,
        }
    }
}

/// Manejador de eventos para el descubrimiento
struct DiscoveryEventHandler {
    event_sender: mpsc::Sender<SyncEvent>,
}

#[async_trait]
impl SyncEventHandler for DiscoveryEventHandler {
    async fn handle_sync_event(&self, event: SyncEvent) -> Result<()> {
        // Reenviar evento al gestor principal
        if let Err(e) = self.event_sender.send(event).await {
            log::error!("Error al reenviar evento: {}", e);
        }
        Ok(())
    }
}

/// Información del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Si el sistema está ejecutándose
    pub is_running: bool,
    /// Si el sistema está habilitado
    pub is_enabled: bool,
    /// Método de sincronización actual
    pub sync_method: SyncMethod,
    /// Sincronización automática habilitada
    pub auto_sync: bool,
    /// Descubrimiento automático habilitado
    pub auto_discovery: bool,
    /// Número de dispositivos conectados
    pub connected_devices: usize,
    /// Número de dispositivos descubiertos
    pub discovered_devices: usize,
    /// Total de sincronizaciones
    pub total_syncs: u64,
    /// Sincronizaciones exitosas
    pub successful_syncs: u64,
    /// Sincronizaciones fallidas
    pub failed_syncs: u64,
    /// Última sincronización
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            is_running: false,
            is_enabled: false,
            sync_method: SyncMethod::Hybrid,
            auto_sync: true,
            auto_discovery: true,
            connected_devices: 0,
            discovered_devices: 0,
            total_syncs: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            last_sync: None,
        }
    }
}

/// Implementar Drop para limpiar recursos
impl Drop for SyncManager {
    fn drop(&mut self) {
        // Intentar detener el gestor si aún está ejecutándose
        let should_stop = {
            if let Ok(running) = self.is_running.try_read() {
                *running
            } else {
                false
            }
        };
        
        if should_stop {
            let _ = tokio::runtime::Handle::current().block_on(self.stop());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_manager_creation() {
        let config = SyncConfig::default();
        let manager = SyncManager::new(config);
        
        assert!(!manager.is_running().await);
        assert_eq!(manager.get_connected_devices().await.len(), 0);
    }

    #[tokio::test]
    async fn test_sync_manager_default() {
        let manager = SyncManager::new_default();
        
        assert!(!manager.is_running().await);
        assert_eq!(manager.get_connected_devices().await.len(), 0);
    }

    #[tokio::test]
    async fn test_system_info_default() {
        let info = SystemInfo::default();
        
        assert!(!info.is_running);
        assert!(!info.is_enabled);
        assert_eq!(info.connected_devices, 0);
        assert_eq!(info.discovered_devices, 0);
    }
}
