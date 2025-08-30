//! Sistema de descubrimiento automático de dispositivos
//! 
//! Este módulo implementa el descubrimiento automático de dispositivos
//! Alohopass en la red local usando mDNS (multicast DNS)

use crate::sync::{
    DeviceInfo, DeviceType, SyncEvent,
};
use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    time::Duration,
    sync::Arc,
};
use tokio::{
    sync::{mpsc, RwLock},
    time::interval,
};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use uuid::Uuid;
use chrono::Utc;

const SERVICE_TYPE: &str = "_alohopass._tcp";

/// Configuración del sistema de descubrimiento
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub port: u16,
    pub device_name: String,
    pub device_type: DeviceType,
    pub os: String,
    pub os_version: String,
    pub app_version: String,
    pub announce_interval: u64,
    pub ttl: u32,
    pub use_mdns: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            port: 0,
            device_name: whoami::hostname(),
            device_type: detect_device_type(),
            os: whoami::platform().to_string(),
            os_version: "Unknown".to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            announce_interval: 30,
            ttl: 120,
            use_mdns: true,
        }
    }
}

/// Detectar el tipo de dispositivo basado en el hostname
fn detect_device_type() -> DeviceType {
    let hostname = whoami::hostname().to_lowercase();
    
    if hostname.contains("macbook") {
        DeviceType::Laptop
    } else if hostname.contains("desktop") || hostname.contains("pc") {
        DeviceType::Desktop
    } else if hostname.contains("laptop") {
        DeviceType::Laptop
    } else if hostname.contains("phone") || hostname.contains("mobile") {
        DeviceType::Mobile
    } else if hostname.contains("tablet") {
        DeviceType::Tablet
    } else {
        DeviceType::Unknown
    }
}

/// Sistema de descubrimiento automático de dispositivos
pub struct DeviceDiscovery {
    config: DiscoveryConfig,
    mdns_daemon: Option<ServiceDaemon>,
    local_service: Option<ServiceInfo>,
    discovered_devices: Arc<RwLock<HashMap<String, DeviceInfo>>>,
    event_sender: mpsc::Sender<SyncEvent>,
    discovery_task: Option<tokio::task::JoinHandle<Result<(), anyhow::Error>>>,
    announce_task: Option<tokio::task::JoinHandle<Result<(), anyhow::Error>>>,
    is_running: Arc<RwLock<bool>>,
}

impl DeviceDiscovery {
    pub fn new(config: DiscoveryConfig, event_sender: mpsc::Sender<SyncEvent>) -> Self {
        Self {
            config,
            mdns_daemon: None,
            local_service: None,
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            discovery_task: None,
            announce_task: None,
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Iniciar el sistema de descubrimiento
    pub async fn start(&mut self) -> Result<()> {
        if *self.is_running.read().await {
            return Ok(());
        }

        // Inicializar mDNS
        self.init_mdns().await?;

        // Iniciar tareas
        self.start_discovery_task().await?;
        self.start_announce_task().await?;

        // Marcar como ejecutándose
        *self.is_running.write().await = true;

        log::info!("Sistema de descubrimiento iniciado correctamente");
        Ok(())
    }

    /// Detener el sistema de descubrimiento
    pub async fn stop(&mut self) -> Result<()> {
        if !*self.is_running.read().await {
            return Ok(());
        }

        // Detener tareas
        if let Some(task) = self.discovery_task.take() {
            task.abort();
        }
        if let Some(task) = self.announce_task.take() {
            task.abort();
        }

        // Limpiar mDNS
        if let Some(daemon) = self.mdns_daemon.take() {
            if let Some(service) = self.local_service.take() {
                daemon.unregister(service.get_fullname())?;
            }
        }

        // Marcar como detenido
        *self.is_running.write().await = false;

        log::info!("Sistema de descubrimiento detenido correctamente");
        Ok(())
    }

    /// Inicializar el sistema mDNS
    async fn init_mdns(&mut self) -> Result<()> {
        let daemon = ServiceDaemon::new()?;
        
        // Crear información del servicio
        let hostname = whoami::hostname();
        let service_name = format!("{}-{}", hostname, Uuid::new_v4().to_string()[..8].to_string());
        
        let mut properties = HashMap::new();
        properties.insert("device_type".to_string(), self.config.device_type.to_string());
        properties.insert("os".to_string(), self.config.os.clone());
        properties.insert("os_version".to_string(), self.config.os_version.clone());
        properties.insert("app_version".to_string(), self.config.app_version.clone());
        properties.insert("device_name".to_string(), self.config.device_name.clone());

        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &service_name,
            &hostname,
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            self.config.port,
            properties,
        )?;

        // Registrar el servicio
        daemon.register(service_info.clone())?;

        self.mdns_daemon = Some(daemon);
        self.local_service = Some(service_info);

        log::info!("Servicio mDNS registrado: {}", service_name);
        Ok(())
    }

    /// Iniciar la tarea de descubrimiento
    async fn start_discovery_task(&mut self) -> Result<()> {
        if !self.config.use_mdns {
            return Ok(());
        }

        let daemon = self.mdns_daemon.as_ref()
            .ok_or_else(|| anyhow!("mDNS daemon no inicializado"))?
            .clone();

        let event_sender = self.event_sender.clone();
        let discovered_devices = self.discovered_devices.clone();

        let task = tokio::spawn(async move {
            let receiver = daemon.browse(SERVICE_TYPE)?;
            
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        if let Err(e) = Self::handle_service_resolved(
                            info,
                            &event_sender,
                            &discovered_devices
                        ).await {
                            log::error!("Error al resolver servicio: {}", e);
                        }
                    }
                    ServiceEvent::ServiceRemoved(_, _) => {
                        // TODO: Implementar eliminación de servicios
                    }
                    _ => {}
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        self.discovery_task = Some(task);
        Ok(())
    }

    /// Iniciar la tarea de anuncio
    async fn start_announce_task(&mut self) -> Result<()> {
        if !self.config.use_mdns {
            return Ok(());
        }

        let announce_interval = self.config.announce_interval;
        let event_sender = self.event_sender.clone();
        let discovered_devices = self.discovered_devices.clone();

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(announce_interval));
            
            loop {
                interval.tick().await;
                
                // Enviar evento de heartbeat
                if let Err(e) = event_sender.send(SyncEvent::Heartbeat).await {
                    log::error!("Error enviando heartbeat: {}", e);
                    break;
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        self.announce_task = Some(task);
        Ok(())
    }

    /// Manejar servicio resuelto
    async fn handle_service_resolved(
        info: ServiceInfo,
        event_sender: &mpsc::Sender<SyncEvent>,
        discovered_devices: &Arc<RwLock<HashMap<String, DeviceInfo>>>,
    ) -> Result<()> {
        let hostname = whoami::hostname();
        let properties = info.get_properties();
        
        // Procesar propiedades TXT
        let device_type = properties
            .get_property_val_str("device_type")
            .and_then(|s| s.parse::<DeviceType>().ok())
            .unwrap_or(DeviceType::Unknown);

        let os = properties
            .get_property_val_str("os")
            .unwrap_or("Unknown");

        let os_version = properties
            .get_property_val_str("os_version")
            .unwrap_or("Unknown");

        let app_version = properties
            .get_property_val_str("app_version")
            .unwrap_or("Unknown");

        let device_name = properties
            .get_property_val_str("device_name")
            .unwrap_or(&hostname);

        let device_info = DeviceInfo::from_network(
            device_name.to_string(),
            device_type,
            os.to_string(),
            os_version.to_string(),
            app_version.to_string(),
            "127.0.0.1".to_string(), // IP por defecto, se actualizará cuando se conecte
            0, // Puerto por defecto, se actualizará cuando se conecte
        );

        // Agregar dispositivo descubierto
        let mut devices = discovered_devices.write().await;
        devices.insert(device_info.id.clone(), device_info.clone());

        // Enviar evento de dispositivo descubierto
        if let Err(e) = event_sender.send(SyncEvent::DeviceDiscovered(device_info)).await {
            log::error!("Error enviando evento de dispositivo descubierto: {}", e);
        }

        Ok(())
    }

    /// Obtener dispositivos descubiertos
    pub async fn get_discovered_devices(&self) -> Vec<DeviceInfo> {
        let devices = self.discovered_devices.read().await;
        devices.values().cloned().collect()
    }

    /// Limpiar dispositivos antiguos
    pub async fn cleanup_old_devices(&self, max_age: Duration) -> Result<()> {
        let mut devices = self.discovered_devices.write().await;
        let now = Utc::now();
        
        devices.retain(|_, device| {
            if let Some(last_seen) = device.last_seen {
                now.signed_duration_since(last_seen).num_seconds() < max_age.as_secs() as i64
            } else {
                true
            }
        });

        Ok(())
    }
}

impl Drop for DeviceDiscovery {
    fn drop(&mut self) {
        // Crear una tarea para limpiar recursos de forma asíncrona
        let mut daemon = None;
        let mut service = None;
        
        if let Some(d) = self.mdns_daemon.take() {
            daemon = Some(d);
        }
        if let Some(s) = self.local_service.take() {
            service = Some(s);
        }
        
        if let (Some(daemon), Some(service)) = (daemon, service) {
            tokio::spawn(async move {
                if let Err(e) = daemon.unregister(service.get_fullname()) {
                    log::error!("Error al desregistrar servicio en drop: {}", e);
                }
            });
        }
    }
}
