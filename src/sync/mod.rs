//! Sistema de sincronización P2P automática para Alohopass
//! 
//! Este módulo implementa:
//! - Descubrimiento automático de dispositivos
//! - Conexión P2P con WebRTC
//! - Sincronización inteligente de contraseñas
//! - Fallback en la nube encriptado

pub mod discovery;
pub mod p2p_connection;
pub mod smart_sync;
pub mod device_info;
pub mod sync_manager;

pub use discovery::DeviceDiscovery;
pub use p2p_connection::P2PConnection;
pub use smart_sync::SmartSync;
pub use device_info::{DeviceInfo, DeviceStatus, DeviceType};
pub use sync_manager::SyncManager;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use anyhow::Result;

/// Estado general del sistema de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub is_enabled: bool,
    pub connected_devices: Vec<DeviceInfo>,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub sync_method: SyncMethod,
    pub auto_sync: bool,
}

/// Métodos de sincronización disponibles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncMethod {
    /// Sincronización P2P directa (más segura)
    P2P,
    /// Sincronización a través de la nube encriptada (fallback)
    CloudEncrypted,
    /// Sincronización híbrida (P2P + fallback en la nube)
    Hybrid,
    /// Solo sincronización local
    LocalOnly,
}

impl Default for SyncMethod {
    fn default() -> Self {
        SyncMethod::Hybrid
    }
}

/// Configuración del sistema de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub auto_discovery: bool,
    pub auto_sync: bool,
    pub sync_interval: u64, // en segundos
    pub max_devices: usize,
    pub encryption_level: EncryptionLevel,
    pub allowed_networks: Vec<String>, // redes WiFi permitidas
}

/// Niveles de encriptación para sincronización
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncryptionLevel {
    /// Encriptación básica (AES-128)
    Basic,
    /// Encriptación estándar (AES-256)
    Standard,
    /// Encriptación militar (AES-256 + ChaCha20-Poly1305)
    Military,
}

impl Default for EncryptionLevel {
    fn default() -> Self {
        EncryptionLevel::Military
    }
}

/// Eventos de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    /// Dispositivo descubierto
    DeviceDiscovered(DeviceInfo),
    /// Dispositivo conectado
    DeviceConnected(DeviceInfo),
    /// Dispositivo desconectado
    DeviceDisconnected(DeviceInfo),
    /// Sincronización iniciada
    SyncStarted(DeviceInfo),
    /// Sincronización completada
    SyncCompleted(DeviceInfo, u64), // número de elementos sincronizados
    /// Sincronización falló
    SyncFailed(DeviceInfo, String), // mensaje de error
    /// Cambios detectados
    ChangesDetected(u64), // número de cambios
    /// Heartbeat del dispositivo
    Heartbeat,
}

/// Estadísticas de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub total_syncs: u64,
    pub successful_syncs: u64,
    pub failed_syncs: u64,
    pub total_data_synced: u64, // en bytes
    pub last_sync_duration: Option<u64>, // en milisegundos
    pub devices_synced_with: Vec<String>, // IDs de dispositivos
}

impl Default for SyncStats {
    fn default() -> Self {
        Self {
            total_syncs: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            total_data_synced: 0,
            last_sync_duration: None,
            devices_synced_with: Vec::new(),
        }
    }
}

/// Trait para manejar eventos de sincronización
#[async_trait::async_trait]
pub trait SyncEventHandler: Send + Sync {
    async fn handle_sync_event(&self, event: SyncEvent) -> Result<()>;
}

/// Implementación por defecto del manejador de eventos
pub struct DefaultSyncEventHandler;

#[async_trait::async_trait]
impl SyncEventHandler for DefaultSyncEventHandler {
    async fn handle_sync_event(&self, event: SyncEvent) -> Result<()> {
        match event {
            SyncEvent::DeviceDiscovered(device) => {
                log::info!("Dispositivo descubierto: {} ({:?})", device.name, device.device_type);
            }
            SyncEvent::DeviceConnected(device) => {
                log::info!("Dispositivo conectado: {} ({:?})", device.name, device.device_type);
            }
            SyncEvent::DeviceDisconnected(device) => {
                log::info!("Dispositivo desconectado: {} ({:?})", device.name, device.device_type);
            }
            SyncEvent::SyncStarted(device) => {
                log::info!("Sincronización iniciada con: {} ({:?})", device.name, device.device_type);
            }
            SyncEvent::SyncCompleted(device, count) => {
                log::info!("Sincronización completada con: {} ({} elementos)", device.name, count);
            }
            SyncEvent::SyncFailed(device, error) => {
                log::error!("Sincronización falló con: {} - Error: {}", device.name, error);
            }
            SyncEvent::ChangesDetected(count) => {
                log::info!("Cambios detectados: {} elementos", count);
            }
            SyncEvent::Heartbeat => {
                log::debug!("Heartbeat recibido");
            }
        }
        Ok(())
    }
}

/// Resultado de una operación de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub device_id: String,
    pub elements_synced: u64,
    pub data_size: u64, // en bytes
    pub duration: u64, // en milisegundos
    pub error_message: Option<String>,
}

impl SyncResult {
    pub fn success(device_id: String, elements_synced: u64, data_size: u64, duration: u64) -> Self {
        Self {
            success: true,
            device_id,
            elements_synced,
            data_size,
            duration,
            error_message: None,
        }
    }

    pub fn failure(device_id: String, error_message: String) -> Self {
        Self {
            success: false,
            device_id,
            elements_synced: 0,
            data_size: 0,
            duration: 0,
            error_message: Some(error_message),
        }
    }
}

/// Configuración por defecto del sistema de sincronización
impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            auto_sync: true,
            sync_interval: 300, // 5 minutos
            max_devices: 10,
            encryption_level: EncryptionLevel::Military,
            allowed_networks: Vec::new(), // todas las redes permitidas por defecto
        }
    }
}

/// Configuración por defecto del estado de sincronización
impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            is_enabled: true,
            connected_devices: Vec::new(),
            last_sync: None,
            sync_method: SyncMethod::Hybrid,
            auto_sync: true,
        }
    }
}
