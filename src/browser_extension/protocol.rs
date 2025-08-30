use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipos de mensajes que puede enviar el plugin del navegador
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum BrowserMessage {
    /// Verificar estado de conexión
    ConnectionStatus,
    
    /// Obtener contraseñas para un dominio
    GetPasswords {
        domain: String,
        form_type: FormType,
    },
    
    /// Crear nueva contraseña
    CreatePassword {
        entry: PasswordEntry,
    },
    
    /// Buscar contraseñas
    SearchPasswords {
        query: String,
    },
    
    /// Sincronizar ahora
    SyncNow,
    
    /// Obtener estadísticas
    GetStats,
}

/// Tipos de formularios que puede detectar el plugin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FormType {
    Login,
    Signup,
}

/// Entrada de contraseña desde el plugin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub title: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub url: String,
    pub domain: String,
    pub form_type: FormType,
}

/// Respuesta a los mensajes del plugin
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl BrowserResponse {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
    
    pub fn simple_success() -> Self {
        Self {
            success: true,
            data: None,
            error: None,
        }
    }
}

/// Estadísticas para el plugin
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserStats {
    pub total_passwords: u64,
    pub last_sync: Option<String>,
    pub connected_devices: u64,
    pub sync_status: String,
}

/// Contraseña para el plugin (sin datos sensibles)
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserPassword {
    pub id: String,
    pub title: String,
    pub username: String,
    pub email: Option<String>,
    pub url: String,
    pub domain: String,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Configuración del plugin
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub auto_fill_enabled: bool,
    pub show_indicators: bool,
    pub auto_sync: bool,
    pub sync_interval: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            auto_fill_enabled: true,
            show_indicators: true,
            auto_sync: true,
            sync_interval: 300, // 5 minutos
        }
    }
}

/// Eventos que puede enviar Tauri al plugin
#[derive(Debug, Serialize, Deserialize)]
pub enum TauriEvent {
    /// Estado de conexión cambiado
    ConnectionStatusChanged {
        connected: bool,
    },
    
    /// Contraseñas actualizadas
    PasswordsUpdated,
    
    /// Sincronización completada
    SyncCompleted {
        success: bool,
        message: String,
    },
    
    /// Error en la aplicación
    Error {
        message: String,
    },
    
    /// Estadísticas actualizadas
    StatsUpdated {
        stats: BrowserStats,
    },
}

/// Mensaje completo para Native Messaging
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeMessage {
    pub id: Option<String>,
    pub message: BrowserMessage,
}

/// Respuesta completa para Native Messaging
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeResponse {
    pub id: Option<String>,
    pub response: BrowserResponse,
}

/// Configuración del host nativo
#[derive(Debug, Serialize, Deserialize)]
pub struct NativeHostConfig {
    pub name: String,
    pub description: String,
    pub path: String,
    pub allowed_origins: Vec<String>,
}

impl Default for NativeHostConfig {
    fn default() -> Self {
        Self {
            name: "com.alohopass.browser".to_string(),
            description: "AlohoPass Browser Extension Host".to_string(),
            path: std::env::current_exe().unwrap_or_default().to_string_lossy().to_string(),
            allowed_origins: vec![
                "chrome-extension://*".to_string(),
                "moz-extension://*".to_string(),
            ],
        }
    }
}
