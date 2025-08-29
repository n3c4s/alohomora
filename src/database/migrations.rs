use rusqlite::Connection;
use anyhow::Result;
use log::info;

pub fn run_migrations(connection: &Connection) -> Result<()> {
    info!("Ejecutando migraciones de base de datos...");
    
    // Crear tablas si no existen
    connection.execute(
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
    
    connection.execute(
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
    
    connection.execute(
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
    
    // Crear tabla de recovery keys (comentada temporalmente)
    // connection.execute(
    //     "CREATE TABLE IF NOT EXISTS recovery_keys (
    //         id TEXT PRIMARY KEY,
    //         encrypted_master TEXT NOT NULL,
    //         created_at TEXT NOT NULL
    //     )",
    //     [],
    // ).map_err(|e| format!("Error creando tabla recovery_keys: {}", e))?;
    
    // Crear índices para mejor performance
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_title ON password_entries (title)",
        [],
    )?;
    
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_category ON password_entries (category_id)",
        [],
    )?;
    
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_username ON password_entries (username)",
        [],
    )?;
    
    info!("Migraciones completadas exitosamente");
    Ok(())
} 