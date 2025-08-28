mod connection;
mod migrations;
mod repository;

pub use connection::*;
pub use migrations::*;
pub use repository::*;

use rusqlite::{Connection, Result};
use anyhow::anyhow;
use std::path::Path;
use log::{info, error};

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path)?;
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
        info!("Ejecutando migraciones de base de datos...");
        
        // Crear tablas si no existen
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT,
                master_password_hash TEXT NOT NULL,
                salt BLOB NOT NULL,
                created_at TEXT NOT NULL,
                last_login TEXT
            )",
            [],
        )?;
        
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT NOT NULL,
                icon TEXT,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES categories (id)
            )",
            [],
        )?;
        
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS password_entries (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                url TEXT,
                notes TEXT,
                category_id TEXT,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_used TEXT,
                FOREIGN KEY (category_id) REFERENCES categories (id)
            )",
            [],
        )?;
        
        // Crear índices para mejor performance
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_password_entries_title ON password_entries (title)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_password_entries_category ON password_entries (category_id)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_password_entries_username ON password_entries (username)",
            [],
        )?;
        
        info!("Migraciones completadas");
        Ok(())
    }
}

pub fn get_database_path() -> Result<String> {
    let app_data = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| anyhow!("No se pudo determinar el directorio de datos de la aplicación"))?;
    
    let db_dir = format!("{}/alohopass", app_data);
    std::fs::create_dir_all(&db_dir)
        .map_err(|e| anyhow!("No se pudo crear el directorio de la base de datos: {}", e))?;
    
    Ok(format!("{}/alohopass.db", db_dir))
} 