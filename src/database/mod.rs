mod connection;
mod migrations;
mod repository;

pub use connection::*;
pub use migrations::*;
pub use repository::*;

use rusqlite::Connection;
use anyhow::Result;
use std::path::Path;
use log::info;

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = rusqlite::Connection::open(path)?;
        let mut manager = Self { connection };
        
        // Ejecutar migraciones
        manager.run_migrations()?;
        
        info!("Base de datos inicializada correctamente");
        Ok(manager)
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    
    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }
    
    fn run_migrations(&mut self) -> Result<()> {
        migrations::run_migrations(&self.connection)
    }
}

pub fn get_database_path() -> Result<String> {
    let app_data = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| anyhow::anyhow!("No se pudo determinar el directorio de datos de la aplicación"))?;
    
    let db_dir = format!("{}/alohopass", app_data);
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| anyhow::anyhow!("No se pudo crear el directorio de la base de datos: {}", e))?;
    
    Ok(format!("{}/alohopass.db", db_dir))
} 