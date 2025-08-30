//! Sincronización inteligente de datos
//! 
//! Este módulo implementa la lógica de sincronización inteligente:
//! - Detección de cambios
//! - Resolución de conflictos
//! - Sincronización incremental
//! - Compresión y optimización de datos

use crate::sync::{DeviceInfo, SyncEvent, SyncEventHandler, SyncResult};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, RwLock};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Tipo de cambio en los datos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    /// Nuevo elemento creado
    Created,
    /// Elemento modificado
    Modified,
    /// Elemento eliminado
    Deleted,
    /// Elemento movido
    Moved,
    /// Metadatos cambiados
    MetadataChanged,
}

impl ChangeType {
    /// Obtener el ícono emoji para el tipo de cambio
    pub fn emoji(&self) -> &'static str {
        match self {
            ChangeType::Created => "➕",
            ChangeType::Modified => "✏️",
            ChangeType::Deleted => "🗑️",
            ChangeType::Moved => "📁",
            ChangeType::MetadataChanged => "ℹ️",
        }
    }

    /// Obtener el nombre legible del tipo de cambio
    pub fn display_name(&self) -> &'static str {
        match self {
            ChangeType::Created => "Creado",
            ChangeType::Modified => "Modificado",
            ChangeType::Deleted => "Eliminado",
            ChangeType::Moved => "Movido",
            ChangeType::MetadataChanged => "Metadatos",
        }
    }
}

/// Cambio en un elemento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChange {
    /// ID único del cambio
    pub id: String,
    /// ID del elemento
    pub element_id: String,
    /// Tipo de cambio
    pub change_type: ChangeType,
    /// Timestamp del cambio
    pub timestamp: DateTime<Utc>,
    /// Dispositivo que originó el cambio
    pub source_device: String,
    /// Datos del elemento (serializados)
    pub element_data: Option<Vec<u8>>,
    /// Metadatos del cambio
    pub metadata: HashMap<String, String>,
    /// Versión del elemento
    pub version: u64,
    /// Hash del elemento anterior
    pub previous_hash: Option<String>,
    /// Hash del elemento actual
    pub current_hash: String,
}

impl DataChange {
    /// Crear un nuevo cambio
    pub fn new(
        element_id: String,
        change_type: ChangeType,
        source_device: String,
        element_data: Option<Vec<u8>>,
        version: u64,
        previous_hash: Option<String>,
    ) -> Self {
        let current_hash = if let Some(ref data) = element_data {
            Self::calculate_hash(data)
        } else {
            String::new()
        };

        Self {
            id: Uuid::new_v4().to_string(),
            element_id,
            change_type,
            timestamp: Utc::now(),
            source_device,
            element_data,
            metadata: HashMap::new(),
            version,
            previous_hash,
            current_hash,
        }
    }

    /// Calcular hash de los datos
    fn calculate_hash(data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Agregar metadato
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Obtener metadato
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Verificar si el cambio es válido
    pub fn is_valid(&self) -> bool {
        !self.element_id.is_empty() && !self.source_device.is_empty()
    }

    /// Obtener tamaño de los datos
    pub fn data_size(&self) -> usize {
        self.element_data.as_ref().map(|d| d.len()).unwrap_or(0)
    }
}

/// Conflicto de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    /// ID del conflicto
    pub id: String,
    /// ID del elemento en conflicto
    pub element_id: String,
    /// Cambios en conflicto
    pub conflicting_changes: Vec<DataChange>,
    /// Timestamp del conflicto
    pub timestamp: DateTime<Utc>,
    /// Estado del conflicto
    pub status: ConflictStatus,
    /// Resolución del conflicto
    pub resolution: Option<ConflictResolution>,
}

/// Estado del conflicto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictStatus {
    /// Conflicto pendiente de resolución
    Pending,
    /// Conflicto resuelto
    Resolved,
    /// Conflicto ignorado
    Ignored,
    /// Conflicto en resolución automática
    AutoResolving,
}

/// Resolución del conflicto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Usar cambio local
    UseLocal,
    /// Usar cambio remoto
    UseRemote,
    /// Combinar cambios
    Merge,
    /// Crear nuevo elemento
    CreateNew,
    /// Eliminar elemento
    Delete,
}

/// Sincronización inteligente
pub struct SmartSync {
    /// Cambios pendientes de sincronización
    pending_changes: Arc<RwLock<Vec<DataChange>>>,
    /// Cambios sincronizados
    synced_changes: Arc<RwLock<Vec<DataChange>>>,
    /// Conflictos de sincronización
    conflicts: Arc<RwLock<Vec<SyncConflict>>>,
    /// Estado de sincronización
    sync_state: Arc<RwLock<SyncState>>,
    /// Canal para eventos
    event_sender: mpsc::Sender<SyncEvent>,
    /// Manejador de eventos
    event_handler: Arc<dyn SyncEventHandler + Send + Sync>,
    /// Configuración de sincronización
    config: SyncConfig,
}

/// Estado de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Si la sincronización está activa
    pub is_active: bool,
    /// Última sincronización
    pub last_sync: Option<DateTime<Utc>>,
    /// Próxima sincronización programada
    pub next_sync: Option<DateTime<Utc>>,
    /// Dispositivos sincronizando
    pub syncing_devices: Vec<String>,
    /// Cambios pendientes
    pub pending_changes_count: usize,
    /// Conflictos pendientes
    pub pending_conflicts_count: usize,
}

/// Configuración de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Sincronización automática
    pub auto_sync: bool,
    /// Intervalo de sincronización (segundos)
    pub sync_interval: u64,
    /// Resolución automática de conflictos
    pub auto_resolve_conflicts: bool,
    /// Estrategia de resolución de conflictos
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
    /// Compresión de datos
    pub enable_compression: bool,
    /// Encriptación de datos
    pub enable_encryption: bool,
    /// Tamaño máximo del batch
    pub max_batch_size: usize,
    /// Tiempo de espera para sincronización (segundos)
    pub sync_timeout: u64,
}

/// Estrategia de resolución de conflictos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    /// Usar siempre el cambio más reciente
    LatestWins,
    /// Usar siempre el cambio local
    LocalWins,
    /// Usar siempre el cambio remoto
    RemoteWins,
    /// Combinar cambios automáticamente
    AutoMerge,
    /// Preguntar al usuario
    AskUser,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_interval: 300, // 5 minutos
            auto_resolve_conflicts: true,
            conflict_resolution_strategy: ConflictResolutionStrategy::LatestWins,
            enable_compression: true,
            enable_encryption: true,
            max_batch_size: 100,
            sync_timeout: 60,
        }
    }
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            is_active: false,
            last_sync: None,
            next_sync: None,
            syncing_devices: Vec::new(),
            pending_changes_count: 0,
            pending_conflicts_count: 0,
        }
    }
}

impl SmartSync {
    /// Crear una nueva instancia de sincronización inteligente
    pub fn new(config: SyncConfig, event_sender: mpsc::Sender<SyncEvent>) -> Self {
        Self {
            pending_changes: Arc::new(RwLock::new(Vec::new())),
            synced_changes: Arc::new(RwLock::new(Vec::new())),
            conflicts: Arc::new(RwLock::new(Vec::new())),
            sync_state: Arc::new(RwLock::new(SyncState::default())),
            event_sender,
            event_handler: Arc::new(crate::sync::DefaultSyncEventHandler),
            config,
        }
    }

    /// Crear con configuración por defecto
    pub fn new_default(event_sender: mpsc::Sender<SyncEvent>) -> Self {
        Self::new(SyncConfig::default(), event_sender)
    }

    /// Agregar un cambio para sincronización
    pub async fn add_change(&self, change: DataChange) -> Result<()> {
        if !change.is_valid() {
            return Err(anyhow!("Cambio inválido"));
        }

        log::debug!("Agregando cambio: {} {} ({} bytes)", 
            change.change_type.emoji(), 
            change.element_id, 
            change.data_size()
        );

        // Agregar a cambios pendientes
        {
            let mut pending = self.pending_changes.write().await;
            pending.push(change.clone());
        }

        // Actualizar estado
        {
            let mut state = self.sync_state.write().await;
            state.pending_changes_count = self.pending_changes.read().await.len();
        }

        // Enviar evento
        let _ = self.event_sender.send(SyncEvent::ChangesDetected(1)).await;

        Ok(())
    }

    /// Obtener cambios pendientes
    pub async fn get_pending_changes(&self) -> Vec<DataChange> {
        self.pending_changes.read().await.clone()
    }

    /// Obtener cambios sincronizados
    pub async fn get_synced_changes(&self) -> Vec<DataChange> {
        self.synced_changes.read().await.clone()
    }

    /// Obtener conflictos
    pub async fn get_conflicts(&self) -> Vec<SyncConflict> {
        self.conflicts.read().await.clone()
    }

    /// Sincronizar cambios con un dispositivo
    pub async fn sync_with_device(&self, device: &DeviceInfo) -> Result<SyncResult> {
        let start_time = Instant::now();
        
        log::info!("Iniciando sincronización con: {} ({})", 
            device.name, device.device_type.display_name()
        );

        // Obtener cambios pendientes
        let pending_changes = self.get_pending_changes().await;
        if pending_changes.is_empty() {
            log::info!("No hay cambios pendientes para sincronizar");
            return Ok(SyncResult::success(
                device.id.clone(),
                0,
                0,
                start_time.elapsed().as_millis() as u64,
            ));
        }

        // Agregar dispositivo a la lista de sincronización
        {
            let mut state = self.sync_state.write().await;
            if !state.syncing_devices.contains(&device.id) {
                state.syncing_devices.push(device.id.clone());
            }
        }

        // Procesar cambios en batches
        let mut total_synced = 0;
        let mut total_data_size = 0;

        for batch in pending_changes.chunks(self.config.max_batch_size) {
            match self.process_batch(batch, device).await {
                Ok((synced_count, data_size)) => {
                    total_synced += synced_count;
                    total_data_size += data_size;
                }
                Err(e) => {
                    log::error!("Error al procesar batch: {}", e);
                    return Ok(SyncResult::failure(
                        device.id.clone(),
                        e.to_string(),
                    ));
                }
            }
        }

        // Marcar cambios como sincronizados
        self.mark_changes_as_synced(&pending_changes).await?;

        // Actualizar estado
        {
            let mut state = self.sync_state.write().await;
            state.last_sync = Some(Utc::now());
            state.syncing_devices.retain(|id| id != &device.id);
            state.pending_changes_count = self.pending_changes.read().await.len();
        }

        let duration = start_time.elapsed().as_millis() as u64;
        
        log::info!("Sincronización completada: {} elementos, {} bytes, {}ms", 
            total_synced, total_data_size, duration
        );

        Ok(SyncResult::success(
            device.id.clone(),
            total_synced as u64,
            total_data_size as u64,
            duration,
        ))
    }

    /// Procesar un batch de cambios
    async fn process_batch(
        &self,
        changes: &[DataChange],
        device: &DeviceInfo,
    ) -> Result<(usize, usize)> {
        let mut synced_count = 0;
        let mut total_data_size = 0;

        for change in changes {
            match self.process_change(change, device).await {
                Ok(data_size) => {
                    synced_count += 1;
                    total_data_size += data_size;
                }
                Err(e) => {
                    log::warn!("Error al procesar cambio {}: {}", change.element_id, e);
                    // Continuar con el siguiente cambio
                }
            }
        }

        Ok((synced_count, total_data_size))
    }

    /// Procesar un cambio individual
    async fn process_change(
        &self,
        change: &DataChange,
        device: &DeviceInfo,
    ) -> Result<usize> {
        // TODO: Implementar lógica de sincronización real
        // Por ahora solo simulamos el procesamiento
        
        log::debug!("Procesando cambio: {} {} -> {}", 
            change.change_type.emoji(), 
            change.element_id, 
            device.name
        );

        // Simular envío de datos
        let data_size = change.data_size();
        
        // Simular latencia de red
        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(data_size)
    }

    /// Marcar cambios como sincronizados
    async fn mark_changes_as_synced(&self, changes: &[DataChange]) -> Result<()> {
        let mut pending = self.pending_changes.write().await;
        let mut synced = self.synced_changes.write().await;

        for change in changes {
            // Remover de pendientes
            pending.retain(|c| c.id != change.id);
            
            // Agregar a sincronizados
            synced.push(change.clone());
        }

        Ok(())
    }

    /// Detectar conflictos
    pub async fn detect_conflicts(&self, remote_changes: Vec<DataChange>) -> Result<Vec<SyncConflict>> {
        let local_changes = self.get_pending_changes().await;
        let mut conflicts = Vec::new();

        for remote_change in remote_changes {
            for local_change in &local_changes {
                if remote_change.element_id == local_change.element_id {
                    // Verificar si hay conflicto
                    if self.is_conflict(&remote_change, local_change).await {
                        let conflict = SyncConflict {
                            id: Uuid::new_v4().to_string(),
                            element_id: remote_change.element_id.clone(),
                            conflicting_changes: vec![remote_change.clone(), local_change.clone()],
                            timestamp: Utc::now(),
                            status: ConflictStatus::Pending,
                            resolution: None,
                        };

                        conflicts.push(conflict);
                    }
                }
            }
        }

        // Agregar conflictos detectados
        if !conflicts.is_empty() {
            let mut all_conflicts = self.conflicts.write().await;
            all_conflicts.extend(conflicts.clone());
        }

        // Actualizar estado
        {
            let mut state = self.sync_state.write().await;
            state.pending_conflicts_count = self.conflicts.read().await.len();
        }

        Ok(conflicts)
    }

    /// Verificar si hay conflicto entre dos cambios
    async fn is_conflict(&self, change1: &DataChange, change2: &DataChange) -> bool {
        // Cambios del mismo tipo no generan conflicto
        if change1.change_type == change2.change_type {
            return false;
        }

        // Cambios de dispositivos diferentes pueden generar conflicto
        if change1.source_device != change2.source_device {
            return true;
        }

        // Cambios con versiones diferentes pueden generar conflicto
        if change1.version != change2.version {
            return true;
        }

        false
    }

    /// Resolver conflicto
    pub async fn resolve_conflict(
        &self,
        conflict_id: &str,
        resolution: ConflictResolution,
    ) -> Result<()> {
        let mut conflicts = self.conflicts.write().await;
        
        if let Some(conflict) = conflicts.iter_mut().find(|c| c.id == conflict_id) {
            conflict.status = ConflictStatus::Resolved;
            conflict.resolution = Some(resolution.clone());
            
            log::info!("Conflicto resuelto: {} -> {:?}", conflict_id, resolution);
        }

        // Actualizar estado
        {
            let mut state = self.sync_state.write().await;
            state.pending_conflicts_count = conflicts.iter()
                .filter(|c| c.status == ConflictStatus::Pending)
                .count();
        }

        Ok(())
    }

    /// Obtener estadísticas de sincronización
    pub async fn get_sync_stats(&self) -> SyncStats {
        let _pending_count = self.pending_changes.read().await.len();
        let synced_count = self.synced_changes.read().await.len();
        let _conflicts_count = self.conflicts.read().await.len();
        let state = self.sync_state.read().await;

        SyncStats {
            total_syncs: synced_count as u64,
            successful_syncs: synced_count as u64,
            failed_syncs: 0, // TODO: Implementar conteo de fallos
            total_data_synced: 0, // TODO: Implementar conteo de bytes
            last_sync_duration: None,
            devices_synced_with: state.syncing_devices.clone(),
        }
    }

    /// Obtener el estado de sincronización
    pub async fn get_sync_state(&self) -> SyncState {
        self.sync_state.read().await.clone()
    }

    /// Establecer manejador de eventos personalizado
    pub fn set_event_handler(&mut self, handler: Arc<dyn SyncEventHandler + Send + Sync>) {
        self.event_handler = handler;
    }

    /// Limpiar cambios antiguos
    pub async fn cleanup_old_changes(&self, max_age: Duration) -> Result<()> {
        let now = Utc::now();
        let max_age_chrono = chrono::Duration::from_std(max_age)?;

        // Limpiar cambios sincronizados antiguos
        {
            let mut synced = self.synced_changes.write().await;
            synced.retain(|change| {
                let age = now - change.timestamp;
                age < max_age_chrono
            });
        }

        // Limpiar conflictos resueltos antiguos
        {
            let mut conflicts = self.conflicts.write().await;
            conflicts.retain(|conflict| {
                let age = now - conflict.timestamp;
                age < max_age_chrono || conflict.status == ConflictStatus::Pending
            });
        }

        Ok(())
    }
}

/// Estadísticas de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    /// Total de sincronizaciones
    pub total_syncs: u64,
    /// Sincronizaciones exitosas
    pub successful_syncs: u64,
    /// Sincronizaciones fallidas
    pub failed_syncs: u64,
    /// Total de datos sincronizados (bytes)
    pub total_data_synced: u64,
    /// Duración de la última sincronización (ms)
    pub last_sync_duration: Option<u64>,
    /// Dispositivos sincronizados
    pub devices_synced_with: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_smart_sync_creation() {
        let (sender, _) = mpsc::channel(10);
        let sync = SmartSync::new_default(sender);
        
        assert_eq!(sync.get_pending_changes().await.len(), 0);
        assert_eq!(sync.get_synced_changes().await.len(), 0);
        assert_eq!(sync.get_conflicts().await.len(), 0);
    }

    #[tokio::test]
    async fn test_data_change_creation() {
        let change = DataChange::new(
            "test-element".to_string(),
            ChangeType::Created,
            "test-device".to_string(),
            Some(b"test data".to_vec()),
            1,
            None,
        );

        assert!(change.is_valid());
        assert_eq!(change.change_type, ChangeType::Created);
        assert_eq!(change.data_size(), 9);
    }

    #[test]
    fn test_change_type_display() {
        let change_type = ChangeType::Modified;
        
        assert_eq!(change_type.emoji(), "✏️");
        assert_eq!(change_type.display_name(), "Modificado");
    }

    #[tokio::test]
    async fn test_add_change() {
        let (sender, _) = mpsc::channel(10);
        let sync = SmartSync::new_default(sender);
        
        let change = DataChange::new(
            "test-element".to_string(),
            ChangeType::Created,
            "test-device".to_string(),
            Some(b"test data".to_vec()),
            1,
            None,
        );

        sync.add_change(change).await.unwrap();
        
        assert_eq!(sync.get_pending_changes().await.len(), 1);
    }
}
