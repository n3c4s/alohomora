mod connection;
mod migrations;
mod repository;

pub use connection::*;
pub use migrations::*;
pub use repository::*;

use rusqlite::Connection;
use anyhow::Result;
use std::path::Path;
use log::{info, error};

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("=== INICIO: Creando DatabaseManager ===");
        info!("Ruta de base de datos: {:?}", path.as_ref());
        
        info!("Abriendo conexión a SQLite...");
        let connection = match rusqlite::Connection::open(path.as_ref()) {
            Ok(conn) => {
                info!("Conexión a SQLite abierta exitosamente");
                conn
            },
            Err(e) => {
                error!("ERROR al abrir conexión SQLite: {}", e);
                return Err(anyhow::anyhow!("Error al abrir conexión SQLite: {}", e));
            }
        };
        
        let mut manager = Self { connection };
        info!("DatabaseManager creado, ejecutando migraciones...");
        
        // Ejecutar migraciones
        match manager.run_migrations() {
            Ok(_) => info!("Migraciones ejecutadas exitosamente"),
            Err(e) => {
                error!("ERROR al ejecutar migraciones: {}", e);
                return Err(anyhow::anyhow!("Error al ejecutar migraciones: {}", e));
            }
        }
        
        info!("=== FIN: Base de datos inicializada correctamente ===");
        Ok(manager)
    }
    
    pub fn new_without_migrations<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("=== INICIO: Creando DatabaseManager SIN migraciones ===");
        info!("Ruta de base de datos: {:?}", path.as_ref());
        
        info!("Abriendo conexión a SQLite...");
        let connection = match rusqlite::Connection::open(path.as_ref()) {
            Ok(conn) => {
                info!("Conexión a SQLite abierta exitosamente");
                conn
            },
            Err(e) => {
                error!("ERROR al abrir conexión SQLite: {}", e);
                return Err(anyhow::anyhow!("Error al abrir conexión SQLite: {}", e));
            }
        };
        
        let manager = Self { connection };
        info!("DatabaseManager creado SIN migraciones");
        
        info!("=== FIN: DatabaseManager creado correctamente ===");
        Ok(manager)
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    
    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }
    
    fn run_migrations(&mut self) -> Result<()> {
        info!("=== INICIO: Ejecutando migraciones ===");
        let result = migrations::run_migrations(&self.connection);
        match &result {
            Ok(_) => info!("=== FIN: Migraciones ejecutadas exitosamente ==="),
            Err(e) => error!("=== ERROR: Migraciones fallaron: {} ===", e),
        }
        result
    }
    
    /// Verifica el estado de la base de datos
    pub fn check_database_status(&self) -> Result<bool> {
        info!("=== INICIO: Verificando estado de la base de datos ===");
        
        // Verificar si la tabla users existe
        let users_exists = match self.connection.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
            [],
            |row| row.get::<_, i64>(0)
        ) {
            Ok(count) => {
                info!("Tabla users existe, count: {}", count);
                count > 0
            },
            Err(e) => {
                error!("Error al verificar tabla users: {}", e);
                false
            }
        };
        
        if !users_exists {
            info!("Tabla users no existe");
            return Ok(false);
        }
        
        // Verificar si hay usuarios en la tabla
        let user_count = match self.connection.query_row(
            "SELECT COUNT(*) FROM users WHERE master_password_hash IS NOT NULL",
            [],
            |row| row.get::<_, i64>(0)
        ) {
            Ok(count) => {
                info!("Usuarios encontrados: {}", count);
                count
            },
            Err(e) => {
                error!("Error al contar usuarios: {}", e);
                return Err(anyhow::anyhow!("Error al contar usuarios: {}", e));
            }
        };
        
        let is_initialized = user_count > 0;
        info!("Base de datos inicializada: {}", is_initialized);
        
        info!("=== FIN: Verificación completada ===");
        Ok(is_initialized)
    }
}

pub fn get_database_path() -> Result<String> {
    info!("=== INICIO: Obteniendo ruta de base de datos ===");
    
    info!("Obteniendo variable de entorno APPDATA...");
    let app_data = std::env::var("APPDATA")
        .or_else(|_| {
            info!("APPDATA no encontrada, intentando HOME...");
            std::env::var("HOME")
        })
        .map_err(|_| {
            error!("No se pudo determinar el directorio de datos de la aplicación");
            anyhow::anyhow!("No se pudo determinar el directorio de datos de la aplicación")
        })?;
    info!("Directorio base obtenido: {}", app_data);
    
    let db_dir = format!("{}/alohopass", app_data);
    info!("Directorio de base de datos: {}", db_dir);
    
    info!("Creando directorio si no existe...");
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| {
            error!("No se pudo crear el directorio de la base de datos: {}", e);
            anyhow::anyhow!("No se pudo crear el directorio de la base de datos: {}", e)
        })?;
    info!("Directorio creado/verificado correctamente");
    
    let db_path = format!("{}/alohopass.db", db_dir);
    info!("Ruta final de base de datos: {}", db_path);
    info!("=== FIN: Ruta de base de datos obtenida ===");
    
    Ok(db_path)
} 