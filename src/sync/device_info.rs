//! Información de dispositivos para sincronización P2P
//! 
//! Este módulo define las estructuras y tipos para identificar
//! y gestionar dispositivos en el sistema de sincronización

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;

/// Tipos de dispositivos soportados
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// Dispositivo móvil (teléfono, smartphone)
    Mobile,
    /// Computadora de escritorio
    Desktop,
    /// Computadora portátil
    Laptop,
    /// Tablet
    Tablet,
    /// Servidor
    Server,
    /// Tipo desconocido
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Mobile => write!(f, "Mobile"),
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Laptop => write!(f, "Laptop"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::Server => write!(f, "Server"),
            DeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::str::FromStr for DeviceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mobile" => Ok(DeviceType::Mobile),
            "Desktop" => Ok(DeviceType::Desktop),
            "Laptop" => Ok(DeviceType::Laptop),
            "Tablet" => Ok(DeviceType::Tablet),
            "Server" => Ok(DeviceType::Server),
            "Unknown" => Ok(DeviceType::Unknown),
            _ => Err(anyhow::anyhow!("Tipo de dispositivo desconocido: {}", s)),
        }
    }
}

impl DeviceType {
    /// Obtener el ícono emoji para el tipo de dispositivo
    pub fn emoji(&self) -> &'static str {
        match self {
            DeviceType::Mobile => "📱",
            DeviceType::Desktop => "🖥️",
            DeviceType::Laptop => "💻",
            DeviceType::Tablet => "📱",
            DeviceType::Server => "🖥️",
            DeviceType::Unknown => "❓",
        }
    }

    /// Obtener el nombre legible del tipo de dispositivo
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceType::Mobile => "Móvil",
            DeviceType::Desktop => "Escritorio",
            DeviceType::Laptop => "Portátil",
            DeviceType::Tablet => "Tablet",
            DeviceType::Server => "Servidor",
            DeviceType::Unknown => "Desconocido",
        }
    }
}

/// Estado de conexión del dispositivo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceStatus {
    /// Dispositivo desconectado
    Disconnected,
    /// Dispositivo conectado
    Connected,
    /// Dispositivo sincronizando
    Syncing,
    /// Dispositivo con error
    Error(String),
    /// Dispositivo en espera
    Waiting,
}

impl DeviceStatus {
    /// Obtener el ícono emoji para el estado
    pub fn emoji(&self) -> &'static str {
        match self {
            DeviceStatus::Disconnected => "🔴",
            DeviceStatus::Connected => "🟢",
            DeviceStatus::Syncing => "🔄",
            DeviceStatus::Error(_) => "❌",
            DeviceStatus::Waiting => "⏳",
        }
    }

    /// Obtener el nombre legible del estado
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceStatus::Disconnected => "Desconectado",
            DeviceStatus::Connected => "Conectado",
            DeviceStatus::Syncing => "Sincronizando",
            DeviceStatus::Error(_) => "Error",
            DeviceStatus::Waiting => "Esperando",
        }
    }

    /// Verificar si el dispositivo está conectado
    pub fn is_connected(&self) -> bool {
        matches!(self, DeviceStatus::Connected)
    }

    /// Verificar si el dispositivo está sincronizando
    pub fn is_syncing(&self) -> bool {
        matches!(self, DeviceStatus::Syncing)
    }

    /// Verificar si el dispositivo tiene error
    pub fn has_error(&self) -> bool {
        matches!(self, DeviceStatus::Error(_))
    }
}

/// Información de un dispositivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// ID único del dispositivo
    pub id: String,
    /// Nombre del dispositivo
    pub name: String,
    /// Tipo de dispositivo
    pub device_type: DeviceType,
    /// Sistema operativo
    pub os: String,
    /// Versión del sistema operativo
    pub os_version: String,
    /// Versión de Alohopass
    pub app_version: String,
    /// Dirección IP del dispositivo
    pub ip_address: Option<String>,
    /// Puerto de comunicación
    pub port: Option<u16>,
    /// Estado de conexión
    pub status: DeviceStatus,
    /// Última vez que se vio el dispositivo
    pub last_seen: Option<DateTime<Utc>>,
    /// Última vez que se sincronizó
    pub last_sync: Option<DateTime<Utc>>,
    /// Capacidades del dispositivo
    pub capabilities: DeviceCapabilities,
    /// Metadatos adicionales
    pub metadata: HashMap<String, String>,
    /// Dispositivo es confiable (verificado)
    pub is_trusted: bool,
    /// Dispositivo es el propietario
    pub is_owner: bool,
}

/// Capacidades del dispositivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Puede sincronizar contraseñas
    pub can_sync_passwords: bool,
    /// Puede sincronizar configuraciones
    pub can_sync_settings: bool,
    /// Puede sincronizar archivos
    pub can_sync_files: bool,
    /// Puede generar contraseñas
    pub can_generate_passwords: bool,
    /// Puede autocompletar en navegadores
    pub can_autocomplete: bool,
    /// Puede usar atajos globales
    pub can_use_shortcuts: bool,
    /// Versión mínima de Alohopass requerida
    pub min_app_version: String,
}

impl Default for DeviceCapabilities {
    fn default() -> Self {
        Self {
            can_sync_passwords: true,
            can_sync_settings: true,
            can_sync_files: false,
            can_generate_passwords: true,
            can_autocomplete: false,
            can_use_shortcuts: false,
            min_app_version: "1.0.0".to_string(),
        }
    }
}

impl DeviceInfo {
    /// Crear un nuevo dispositivo
    pub fn new(
        name: String,
        device_type: DeviceType,
        os: String,
        os_version: String,
        app_version: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            device_type,
            os,
            os_version,
            app_version,
            ip_address: None,
            port: None,
            status: DeviceStatus::Disconnected,
            last_seen: Some(Utc::now()),
            last_sync: None,
            capabilities: DeviceCapabilities::default(),
            metadata: HashMap::new(),
            is_trusted: false,
            is_owner: true, // El dispositivo actual es el propietario
        }
    }

    /// Crear un dispositivo desde información de red
    pub fn from_network(
        name: String,
        device_type: DeviceType,
        os: String,
        os_version: String,
        app_version: String,
        ip_address: String,
        port: u16,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            device_type,
            os,
            os_version,
            app_version,
            ip_address: Some(ip_address),
            port: Some(port),
            status: DeviceStatus::Disconnected,
            last_seen: Some(Utc::now()),
            last_sync: None,
            capabilities: DeviceCapabilities::default(),
            metadata: HashMap::new(),
            is_trusted: false,
            is_owner: false, // Dispositivo descubierto en la red
        }
    }

    /// Obtener el nombre de visualización completo
    pub fn display_name(&self) -> String {
        format!("{} {} ({})", self.device_type.emoji(), self.name, self.device_type.display_name())
    }

    /// Obtener el estado de visualización
    pub fn display_status(&self) -> String {
        format!("{} {}", self.status.emoji(), self.status.display_name())
    }

    /// Verificar si el dispositivo está disponible para sincronización
    pub fn is_available_for_sync(&self) -> bool {
        self.status.is_connected() && !self.status.is_syncing() && !self.status.has_error()
    }

    /// Verificar si el dispositivo es compatible
    pub fn is_compatible(&self) -> bool {
        // Verificar versión mínima requerida
        // TODO: Implementar comparación de versiones semántica
        true
    }

    /// Actualizar el estado del dispositivo
    pub fn update_status(&mut self, new_status: DeviceStatus) {
        self.status = new_status;
        self.last_seen = Some(Utc::now());
    }

    /// Marcar como sincronizado
    pub fn mark_synced(&mut self) {
        self.last_sync = Some(Utc::now());
        self.status = DeviceStatus::Connected;
    }

    /// Agregar metadato
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Obtener metadato
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Verificar si el dispositivo es el mismo (por nombre y tipo)
    pub fn is_same_device(&self, other: &DeviceInfo) -> bool {
        self.name == other.name && self.device_type == other.device_type
    }

    /// Obtener información de conexión
    pub fn connection_info(&self) -> Option<String> {
        match (&self.ip_address, self.port) {
            (Some(ip), Some(port)) => Some(format!("{}:{}", ip, port)),
            (Some(ip), None) => Some(ip.clone()),
            (None, Some(port)) => Some(format!("Puerto {}", port)),
            (None, None) => None,
        }
    }

    /// Obtener tiempo desde la última sincronización
    pub fn time_since_last_sync(&self) -> Option<chrono::Duration> {
        self.last_sync.map(|last_sync| Utc::now() - last_sync)
    }

    /// Obtener tiempo desde que se vio por última vez
    pub fn time_since_last_seen(&self) -> chrono::Duration {
        Utc::now() - self.last_seen.unwrap_or(Utc::now())
    }
}

/// Información del dispositivo local
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDeviceInfo {
    /// Información básica del dispositivo
    pub device: DeviceInfo,
    /// Clave pública para encriptación
    pub public_key: String,
    /// Certificado del dispositivo
    pub certificate: Option<String>,
    /// Configuración local
    pub local_config: LocalDeviceConfig,
}

/// Configuración local del dispositivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDeviceConfig {
    /// Nombre del usuario
    pub user_name: String,
    /// Email del usuario
    pub user_email: Option<String>,
    /// Preferencias de sincronización
    pub sync_preferences: SyncPreferences,
    /// Configuración de red
    pub network_config: NetworkConfig,
}

/// Preferencias de sincronización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPreferences {
    /// Sincronizar automáticamente
    pub auto_sync: bool,
    /// Intervalo de sincronización automática (segundos)
    pub auto_sync_interval: u64,
    /// Sincronizar solo en redes WiFi
    pub wifi_only: bool,
    /// Sincronizar en segundo plano
    pub background_sync: bool,
    /// Notificar sobre cambios
    pub notify_changes: bool,
}

/// Configuración de red
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Puerto para escuchar conexiones
    pub listen_port: u16,
    /// Interfaces de red permitidas
    pub allowed_interfaces: Vec<String>,
    /// Redes WiFi permitidas
    pub allowed_networks: Vec<String>,
    /// Usar mDNS para descubrimiento
    pub use_mdns: bool,
    /// Usar UPnP para NAT traversal
    pub use_upnp: bool,
}

impl Default for SyncPreferences {
    fn default() -> Self {
        Self {
            auto_sync: true,
            auto_sync_interval: 300, // 5 minutos
            wifi_only: true,
            background_sync: true,
            notify_changes: true,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_port: 0, // Puerto aleatorio
            allowed_interfaces: Vec::new(), // Todas las interfaces
            allowed_networks: Vec::new(), // Todas las redes
            use_mdns: true,
            use_upnp: false,
        }
    }
}

impl Default for LocalDeviceConfig {
    fn default() -> Self {
        Self {
            user_name: whoami::username(),
            user_email: None,
            sync_preferences: SyncPreferences::default(),
            network_config: NetworkConfig::default(),
        }
    }
}

/// Comparador de dispositivos por último visto
pub struct DeviceLastSeenComparator;

impl DeviceLastSeenComparator {
    /// Comparar dispositivos por último visto (más reciente primero)
    pub fn compare(a: &DeviceInfo, b: &DeviceInfo) -> std::cmp::Ordering {
        b.last_seen.unwrap_or(Utc::now()).cmp(&a.last_seen.unwrap_or(Utc::now()))
    }
}

/// Comparador de dispositivos por nombre
pub struct DeviceNameComparator;

impl DeviceNameComparator {
    /// Comparar dispositivos por nombre (alfabético)
    pub fn compare(a: &DeviceInfo, b: &DeviceInfo) -> std::cmp::Ordering {
        a.name.cmp(&b.name)
    }
}

/// Comparador de dispositivos por tipo
pub struct DeviceTypeComparator;

impl DeviceTypeComparator {
    /// Comparar dispositivos por tipo
    pub fn compare(a: &DeviceInfo, b: &DeviceInfo) -> std::cmp::Ordering {
        // Comparar por string representation
        a.device_type.to_string().cmp(&b.device_type.to_string())
    }
}
