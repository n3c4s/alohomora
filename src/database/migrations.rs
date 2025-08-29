use rusqlite::Connection;
use anyhow::Result;
use log::{info, error};

/// Función de utilidad para verificar si una tabla existe
fn table_exists(connection: &Connection, table_name: &str) -> bool {
    match connection.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
        [table_name],
        |row| row.get::<_, i64>(0)
    ) {
        Ok(count) => {
            info!("Tabla {} existe, count: {}", table_name, count);
            count > 0
        },
        Err(e) => {
            info!("Error al verificar tabla {} (esto es normal si no existe): {}", table_name, e);
            false
        }
    }
}

pub fn run_migrations(connection: &Connection) -> Result<()> {
    info!("=== INICIO: Ejecutando migraciones de base de datos ===");
    info!("Conexión recibida: {:?}", connection);
    
    // Verificar si la tabla users ya existe antes de crearla
    info!("Verificando si la tabla users ya existe...");
    let table_exists_before = table_exists(connection, "users");
    info!("Tabla users existe antes de migraciones: {}", table_exists_before);
    
    // Crear tablas si no existen
    info!("Creando tabla users...");
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT,
            master_password_hash TEXT NOT NULL,
            salt TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_login TEXT
        )",
        [],
    ) {
        Ok(_) => info!("Tabla users creada/verificada correctamente"),
        Err(e) => {
            error!("ERROR al crear tabla users: {}", e);
            return Err(anyhow::anyhow!("Error al crear tabla users: {}", e));
        }
    }
    
    // Verificar si la tabla users existe después de crearla
    info!("Verificando si la tabla users existe después de crearla...");
    let table_exists_after = table_exists(connection, "users");
    info!("Tabla users existe después de crearla: {}", table_exists_after);
    
    if !table_exists_after {
        error!("ERROR CRÍTICO: La tabla users no existe después de intentar crearla");
        return Err(anyhow::anyhow!("La tabla users no existe después de intentar crearla"));
    }
    
    info!("Creando tabla categories...");
    match connection.execute(
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
    ) {
        Ok(_) => info!("Tabla categories creada/verificada correctamente"),
        Err(e) => {
            error!("ERROR al crear tabla categories: {}", e);
            return Err(anyhow::anyhow!("Error al crear tabla categories: {}", e));
        }
    }
    
    info!("Creando tabla password_entries...");
    match connection.execute(
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
    ) {
        Ok(_) => info!("Tabla password_entries creada/verificada correctamente"),
        Err(e) => {
            error!("ERROR al crear tabla password_entries: {}", e);
            return Err(anyhow::anyhow!("Error al crear tabla password_entries: {}", e));
        }
    }
    
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
    info!("Creando índices...");
    match connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_title ON password_entries (title)",
        [],
    ) {
        Ok(_) => info!("Índice idx_password_entries_title creado/verificado correctamente"),
        Err(e) => {
            error!("ERROR al crear índice idx_password_entries_title: {}", e);
            return Err(anyhow::anyhow!("Error al crear índice idx_password_entries_title: {}", e));
        }
    }
    
    match connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_category ON password_entries (category_id)",
        [],
    ) {
        Ok(_) => info!("Índice idx_password_entries_category creado/verificado correctamente"),
        Err(e) => {
            error!("ERROR al crear índice idx_password_entries_category: {}", e);
            return Err(anyhow::anyhow!("Error al crear índice idx_password_entries_category: {}", e));
        }
    }
    
    match connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_password_entries_username ON password_entries (username)",
        [],
    ) {
        Ok(_) => info!("Índice idx_password_entries_username creado/verificado correctamente"),
        Err(e) => {
            error!("ERROR al crear índice idx_password_entries_username: {}", e);
            return Err(anyhow::anyhow!("Error al crear índice idx_password_entries_username: {}", e));
        }
    }
    
    info!("=== FIN: Migraciones completadas exitosamente ===");
    Ok(())
} 