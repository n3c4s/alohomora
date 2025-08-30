//! Sistema de sincronización P2P automática para Alohopass
//! 
//! Este módulo implementa:
//! - Descubrimiento automático de dispositivos
//! - Conexión P2P con WebRTC
//! - Sincronización inteligente de contraseñas
//! - Fallback en la nube encriptado

pub mod device_info;
pub mod discovery;
pub mod p2p_connection;
pub mod smart_sync;
pub mod sync_manager;
pub mod commands;

pub use device_info::{DeviceInfo, DeviceType, DeviceStatus};
pub use discovery::DeviceDiscovery;
pub use p2p_connection::P2PConnection;
pub use smart_sync::SmartSync;
pub use sync_manager::SyncManager;
pub use commands::*;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    DeviceDiscovered(DeviceInfo),
    DeviceConnected(DeviceInfo),
    DeviceDisconnected(DeviceInfo),
    SyncStarted(DeviceInfo),
    SyncCompleted(DeviceInfo, u64),
    SyncFailed(DeviceInfo, String),
    ChangesDetected(u64),
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub auto_sync: bool,
    pub sync_interval: u64, // en minutos
    pub discovery_enabled: bool,
    pub allow_incoming_connections: bool,
    pub auto_discovery: bool, // para compatibilidad
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_interval: 15,
            discovery_enabled: true,
            allow_incoming_connections: true,
            auto_discovery: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub is_enabled: bool,
    pub is_syncing: bool,
    pub last_sync_time: Option<String>,
    pub error: Option<String>,
    pub connected_devices: Vec<DeviceInfo>, // Cambiado de u32 a Vec<DeviceInfo>
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>, // para compatibilidad
    pub sync_method: SyncMethod, // para compatibilidad
    pub auto_sync: bool, // para compatibilidad
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            is_enabled: false,
            is_syncing: false,
            last_sync_time: None,
            error: None,
            connected_devices: Vec::new(),
            last_sync: None,
            sync_method: SyncMethod::Hybrid,
            auto_sync: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub total_passwords: u64,
    pub synced_passwords: u64,
    pub last_sync_duration: u64, // en segundos
    pub devices_count: u32,
    // Campos para compatibilidad
    pub total_syncs: u64,
    pub successful_syncs: u64,
    pub failed_syncs: u64,
    pub total_data_synced: u64,
    pub devices_synced_with: Vec<String>,
}

impl Default for SyncStats {
    fn default() -> Self {
        Self {
            total_passwords: 0,
            synced_passwords: 0,
            last_sync_duration: 0,
            devices_count: 0,
            total_syncs: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            total_data_synced: 0,
            devices_synced_with: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncMethod {
    P2P,
    CloudEncrypted,
    Hybrid,
    LocalOnly,
}

impl Default for SyncMethod {
    fn default() -> Self {
        SyncMethod::Hybrid
    }
}

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

pub trait SyncEventHandler: Send + Sync {
    fn handle_event(&self, event: &SyncEvent);
}

pub struct DefaultSyncEventHandler;

impl SyncEventHandler for DefaultSyncEventHandler {
    fn handle_event(&self, event: &SyncEvent) {
        match event {
            SyncEvent::DeviceDiscovered(device) => {
                log::info!("Dispositivo descubierto: {}", device.name);
            }
            SyncEvent::DeviceConnected(device) => {
                log::info!("Dispositivo conectado: {}", device.name);
            }
            SyncEvent::DeviceDisconnected(device) => {
                log::info!("Dispositivo desconectado: {}", device.name);
            }
            SyncEvent::SyncStarted(device) => {
                log::info!("Sincronización iniciada con: {}", device.name);
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
                log::debug!("Heartbeat de sincronización");
            }
        }
    }
}
