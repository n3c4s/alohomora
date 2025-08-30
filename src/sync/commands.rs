use crate::sync::{SyncManager, SyncConfig, SyncStatus, SyncStats, DeviceInfo};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncConfigUpdate {
    pub auto_sync: bool,
    pub sync_interval: u64,
    pub discovery_enabled: bool,
    pub allow_incoming_connections: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceTrustRequest {
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRemoveRequest {
    pub device_id: String,
}

/// Obtener la configuración actual de sincronización
#[tauri::command]
pub async fn get_sync_config(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<SyncConfig, String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora retornamos configuración por defecto
        Ok(SyncConfig::default())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Obtener el estado actual de sincronización
#[tauri::command]
pub async fn get_sync_status(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<SyncStatus, String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora retornamos estado por defecto
        Ok(SyncStatus::default())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Obtener dispositivos sincronizados
#[tauri::command]
pub async fn get_sync_devices(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<Vec<DeviceInfo>, String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora retornamos lista vacía
        Ok(Vec::new())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Obtener estadísticas de sincronización
#[tauri::command]
pub async fn get_sync_stats(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<SyncStats, String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora retornamos estadísticas por defecto
        Ok(SyncStats::default())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Iniciar sincronización
#[tauri::command]
pub async fn start_sync(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<(), String> {
    let mut manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_mut() {
        // Por ahora solo simulamos éxito
        log::info!("Sincronización iniciada");
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Detener sincronización
#[tauri::command]
pub async fn stop_sync(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<(), String> {
    let mut manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_mut() {
        // Por ahora solo simulamos éxito
        log::info!("Sincronización detenida");
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Iniciar descubrimiento de dispositivos
#[tauri::command]
pub async fn start_device_discovery(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<(), String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora solo simulamos éxito
        log::info!("Descubrimiento de dispositivos iniciado");
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Sincronizar ahora
#[tauri::command]
pub async fn sync_now(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>
) -> Result<(), String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora solo simulamos éxito
        log::info!("Sincronización manual iniciada");
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Actualizar configuración de sincronización
#[tauri::command]
pub async fn update_sync_config(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>,
    config: SyncConfigUpdate
) -> Result<(), String> {
    let mut manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_mut() {
        // Por ahora solo simulamos éxito
        log::info!("Configuración actualizada: {:?}", config);
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Confiar en un dispositivo
#[tauri::command]
pub async fn trust_device(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>,
    request: DeviceTrustRequest
) -> Result<(), String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora solo simulamos éxito
        log::info!("Dispositivo marcado como confiable: {}", request.device_id);
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}

/// Remover un dispositivo
#[tauri::command]
pub async fn remove_device(
    sync_manager: State<'_, Mutex<Option<SyncManager>>>,
    request: DeviceRemoveRequest
) -> Result<(), String> {
    let manager = sync_manager.lock().map_err(|e| e.to_string())?;
    
    if let Some(manager) = manager.as_ref() {
        // Por ahora solo simulamos éxito
        log::info!("Dispositivo removido: {}", request.device_id);
        Ok(())
    } else {
        Err("Sync manager not initialized".to_string())
    }
}
