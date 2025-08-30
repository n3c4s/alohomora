//! Conexión P2P usando WebRTC
//! 
//! Este módulo implementa la conexión directa entre dispositivos
//! usando WebRTC para la sincronización de datos

use crate::sync::{DeviceInfo, SyncEvent, SyncEventHandler};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use webrtc::{
    api::APIBuilder,
    data_channel::data_channel_init::RTCDataChannelInit,
    peer_connection::configuration::RTCConfiguration,
    peer_connection::peer_connection_state::RTCPeerConnectionState,
    peer_connection::RTCPeerConnection,
};

/// Configuración de la conexión P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    /// Puerto para la conexión
    pub port: u16,
    /// ICE servers para NAT traversal
    pub ice_servers: Vec<String>,
    /// Tiempo de espera para conexión (segundos)
    pub connection_timeout: u64,
    /// Tamaño máximo del buffer de datos
    pub max_buffer_size: usize,
    /// Usar conexión encriptada
    pub encrypted: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            port: 0, // Puerto aleatorio
            ice_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            connection_timeout: 30,
            max_buffer_size: 1024 * 1024, // 1MB
            encrypted: true,
        }
    }
}

/// Estado de la conexión P2P
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum P2PConnectionState {
    /// Desconectado
    Disconnected,
    /// Conectando
    Connecting,
    /// Conectado
    Connected,
    /// Reconectando
    Reconnecting,
    /// Error
    Error(String),
}

impl P2PConnectionState {
    /// Obtener el ícono emoji para el estado
    pub fn emoji(&self) -> &'static str {
        match self {
            P2PConnectionState::Disconnected => "🔴",
            P2PConnectionState::Connecting => "🟡",
            P2PConnectionState::Connected => "🟢",
            P2PConnectionState::Reconnecting => "🔄",
            P2PConnectionState::Error(_) => "❌",
        }
    }

    /// Obtener el nombre legible del estado
    pub fn display_name(&self) -> &'static str {
        match self {
            P2PConnectionState::Disconnected => "Desconectado",
            P2PConnectionState::Connecting => "Conectando",
            P2PConnectionState::Connected => "Conectado",
            P2PConnectionState::Reconnecting => "Reconectando",
            P2PConnectionState::Error(_) => "Error",
        }
    }
}

/// Conexión P2P con WebRTC
pub struct P2PConnection {
    /// Configuración de la conexión
    config: P2PConfig,
    /// Conexión WebRTC
    peer_connection: Option<RTCPeerConnection>,
    /// Canal de datos
    data_channel: Option<Arc<webrtc::data_channel::RTCDataChannel>>,
    /// Estado de la conexión
    state: Arc<RwLock<P2PConnectionState>>,
    /// Dispositivo remoto
    remote_device: Option<DeviceInfo>,
    /// Canal para eventos
    event_sender: mpsc::Sender<SyncEvent>,
    /// Manejador de eventos
    event_handler: Arc<dyn SyncEventHandler + Send + Sync>,
    /// Buffer de datos pendientes
    pending_data: Arc<RwLock<Vec<Vec<u8>>>>,
}

impl P2PConnection {
    /// Crear una nueva conexión P2P
    pub fn new(config: P2PConfig, event_sender: mpsc::Sender<SyncEvent>) -> Self {
        Self {
            config,
            peer_connection: None,
            data_channel: None,
            state: Arc::new(RwLock::new(P2PConnectionState::Disconnected)),
            remote_device: None,
            event_sender,
            event_handler: Arc::new(crate::sync::DefaultSyncEventHandler),
            pending_data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Crear con configuración por defecto
    pub fn new_default(event_sender: mpsc::Sender<SyncEvent>) -> Self {
        Self::new(P2PConfig::default(), event_sender)
    }

    /// Iniciar conexión con un dispositivo
    pub async fn connect(&mut self, device: DeviceInfo) -> Result<()> {
        if *self.state.read().await == P2PConnectionState::Connected {
            return Err(anyhow!("Ya hay una conexión activa"));
        }

        log::info!("Iniciando conexión P2P con: {} ({})", device.name, device.device_type.display_name());

        // Actualizar estado
        *self.state.write().await = P2PConnectionState::Connecting;
        self.remote_device = Some(device.clone());

        // Crear conexión WebRTC
        self.create_peer_connection().await?;

        // Crear canal de datos
        self.create_data_channel().await?;

        // Generar oferta
        let offer = self.create_offer().await?;

        // TODO: Enviar oferta al dispositivo remoto
        log::info!("Oferta WebRTC generada, enviando al dispositivo remoto...");

        Ok(())
    }

    /// Crear la conexión peer
    async fn create_peer_connection(&mut self) -> Result<()> {
        let config = RTCConfiguration {
            ice_servers: self.config.ice_servers.iter()
                .map(|server| webrtc::ice_transport::ice_server::RTCIceServer {
                    urls: vec![server.clone()],
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };

        let api = APIBuilder::new()
            .with_setting_engine(Default::default())
            .build();

        let peer_connection = api.new_peer_connection(config).await?;

        // Configurar manejadores de eventos
        self.setup_peer_connection_handlers(&peer_connection).await?;

        self.peer_connection = Some(peer_connection);
        Ok(())
    }

    /// Configurar manejadores de eventos de la conexión peer
    async fn setup_peer_connection_handlers(&self, pc: &RTCPeerConnection) -> Result<()> {
        let state = self.state.clone();
        let event_sender = self.event_sender.clone();

        // Manejador de cambio de estado
        pc.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
            let state = state.clone();
            let event_sender = event_sender.clone();
            
            Box::pin(async move {
                let new_state = match s {
                    RTCPeerConnectionState::Connected => P2PConnectionState::Connected,
                    RTCPeerConnectionState::Disconnected => P2PConnectionState::Disconnected,
                    RTCPeerConnectionState::Failed => P2PConnectionState::Error("Conexión falló".to_string()),
                    _ => P2PConnectionState::Connecting,
                };

                *state.write().await = new_state.clone();
            })
        }));

        Ok(())
    }

    /// Crear canal de datos
    async fn create_data_channel(&mut self) -> Result<()> {
        let pc = self.peer_connection.as_ref()
            .ok_or_else(|| anyhow!("Conexión peer no inicializada"))?;

        let data_channel_init = RTCDataChannelInit {
            ordered: Some(true),
            ..Default::default()
        };

        let data_channel = pc.create_data_channel("alohopass-sync", Some(data_channel_init)).await?;

        // Configurar manejadores del canal de datos
        self.setup_data_channel_handlers(&data_channel).await?;

        self.data_channel = Some(data_channel);
        Ok(())
    }

    /// Configurar manejadores del canal de datos
    async fn setup_data_channel_handlers(&self, dc: &Arc<webrtc::data_channel::RTCDataChannel>) -> Result<()> {
        let pending_data = self.pending_data.clone();
        let event_sender = self.event_sender.clone();

        // Manejador de datos recibidos
        dc.on_message(Box::new(move |msg: webrtc::data_channel::data_channel_message::DataChannelMessage| {
            Box::pin(async move {
                match msg.is_string {
                    true => {
                        // Mensaje de texto
                        if let Ok(text) = String::from_utf8(msg.data.to_vec()) {
                            log::info!("Mensaje de texto recibido: {}", text);
                            // TODO: Procesar mensaje de texto
                        }
                    }
                    false => {
                        // Mensaje binario
                        log::info!("Mensaje binario recibido: {} bytes", msg.data.len());
                        // TODO: Procesar mensaje binario
                    }
                }
            })
        }));

        Ok(())
    }

    /// Crear oferta WebRTC
    async fn create_offer(&self) -> Result<String> {
        let pc = self.peer_connection.as_ref()
            .ok_or_else(|| anyhow!("Conexión peer no inicializada"))?;

        let offer = pc.create_offer(None).await?;

        // Establecer descripción local
        pc.set_local_description(offer).await?;

        // Obtener oferta SDP
        let sdp = pc.local_description().await
            .ok_or_else(|| anyhow!("No se pudo obtener la descripción local"))?;

        Ok(sdp.sdp.clone())
    }

    /// Procesar respuesta del dispositivo remoto
    pub async fn process_answer(&mut self, answer_sdp: String) -> Result<()> {
        let pc = self.peer_connection.as_mut()
            .ok_or_else(|| anyhow!("Conexión peer no inicializada"))?;

        let answer = webrtc::peer_connection::sdp::session_description::RTCSessionDescription::answer(
            answer_sdp,
        )?;

        pc.set_remote_description(answer).await?;

        log::info!("Respuesta del dispositivo remoto procesada");
        Ok(())
    }

    /// Enviar datos a través de la conexión P2P
    pub async fn send_data(&self, data: Vec<u8>) -> Result<()> {
        let dc = self.data_channel.as_ref()
            .ok_or_else(|| anyhow!("Canal de datos no disponible"))?;

        if *self.state.read().await != P2PConnectionState::Connected {
            return Err(anyhow!("Conexión no está establecida"));
        }

        dc.send(&bytes::Bytes::from(data.clone())).await?;
        log::debug!("Datos enviados: {} bytes", data.len());

        Ok(())
    }

    /// Enviar texto a través de la conexión P2P
    pub async fn send_text(&self, text: String) -> Result<()> {
        let dc = self.data_channel.as_ref()
            .ok_or_else(|| anyhow!("Canal de datos no disponible"))?;

        if *self.state.read().await != P2PConnectionState::Connected {
            return Err(anyhow!("Conexión no está establecida"));
        }

        dc.send_text(&text).await?;
        log::debug!("Texto enviado: {}", text);

        Ok(())
    }

    /// Obtener datos pendientes
    pub async fn get_pending_data(&self) -> Vec<Vec<u8>> {
        let mut pending = self.pending_data.write().await;
        let data = pending.clone();
        pending.clear();
        data
    }

    /// Desconectar
    pub async fn disconnect(&mut self) -> Result<()> {
        log::info!("Desconectando conexión P2P...");

        // Cerrar canal de datos
        if let Some(dc) = self.data_channel.take() {
            dc.close().await?;
        }

        // Cerrar conexión peer
        if let Some(pc) = self.peer_connection.take() {
            pc.close().await?;
        }

        // Actualizar estado
        *self.state.write().await = P2PConnectionState::Disconnected;

        // Limpiar dispositivo remoto
        self.remote_device = None;

        log::info!("Conexión P2P desconectada");
        Ok(())
    }

    /// Obtener el estado de la conexión
    pub async fn get_state(&self) -> P2PConnectionState {
        self.state.read().await.clone()
    }

    /// Verificar si está conectado
    pub async fn is_connected(&self) -> bool {
        *self.state.read().await == P2PConnectionState::Connected
    }

    /// Obtener información del dispositivo remoto
    pub fn get_remote_device(&self) -> Option<DeviceInfo> {
        self.remote_device.clone()
    }

    /// Establecer manejador de eventos personalizado
    pub fn set_event_handler(&mut self, handler: Arc<dyn SyncEventHandler + Send + Sync>) {
        self.event_handler = handler;
    }

    /// Obtener estadísticas de la conexión
    pub async fn get_stats(&self) -> P2PConnectionStats {
        let state = self.state.read().await;
        let is_connected = *state == P2PConnectionState::Connected;
        let remote_device = self.remote_device.clone();

        P2PConnectionStats {
            state: state.clone(),
            is_connected,
            remote_device,
            pending_data_count: self.pending_data.read().await.len(),
        }
    }
}

/// Estadísticas de la conexión P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConnectionStats {
    /// Estado de la conexión
    pub state: P2PConnectionState,
    /// Si está conectado
    pub is_connected: bool,
    /// Dispositivo remoto
    pub remote_device: Option<DeviceInfo>,
    /// Cantidad de datos pendientes
    pub pending_data_count: usize,
}

impl Default for P2PConnectionStats {
    fn default() -> Self {
        Self {
            state: P2PConnectionState::Disconnected,
            is_connected: false,
            remote_device: None,
            pending_data_count: 0,
        }
    }
}

/// Implementar Drop para limpiar recursos
impl Drop for P2PConnection {
    fn drop(&mut self) {
        // Intentar desconectar si aún está conectado
        let should_disconnect = {
            if let Ok(state) = self.state.try_read() {
                *state == P2PConnectionState::Connected
            } else {
                false
            }
        };
        
        if should_disconnect {
            let _ = tokio::runtime::Handle::current().block_on(self.disconnect());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_p2p_connection_creation() {
        let (sender, _) = mpsc::channel(10);
        let connection = P2PConnection::new_default(sender);
        
        assert!(!connection.is_connected().await);
        assert_eq!(connection.get_state().await, P2PConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_p2p_config_default() {
        let config = P2PConfig::default();
        
        assert_eq!(config.port, 0);
        assert!(!config.ice_servers.is_empty());
        assert!(config.encrypted);
    }

    #[test]
    fn test_p2p_connection_state_display() {
        let state = P2PConnectionState::Connected;
        
        assert_eq!(state.emoji(), "🟢");
        assert_eq!(state.display_name(), "Conectado");
    }
}
