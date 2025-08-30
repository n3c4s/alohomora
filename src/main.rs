// Pre-requisitos
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod crypto;
mod database;
mod models;
mod sync;

use tauri::Manager;
use std::sync::Mutex;
use serde_json;
use base64::Engine;
use log::{info, error, warn};
use env_logger;
use crate::sync::commands::*;

/// Función de utilidad para verificar si una tabla existe
fn table_exists(connection: &rusqlite::Connection, table_name: &str) -> bool {
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

// Estado global de la aplicación
struct AppState {
    crypto_manager: Mutex<crypto::CryptoManager>,
    database_manager: Mutex<Option<database::DatabaseManager>>,
    is_initialized: Mutex<bool>,
    sync_manager: Mutex<Option<sync::SyncManager>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            crypto_manager: Mutex::new(crypto::CryptoManager::new()),
            database_manager: Mutex::new(None),
            is_initialized: Mutex::new(false),
            sync_manager: Mutex::new(None),
        }
    }
}

fn main() {
    // Inicializar logging
    env_logger::init();
    
    info!("Iniciando Alohopass...");
    
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            info!("Configurando Alohopass...");
            
            let app_handle = app.handle();
            
            // Inicializar database_manager si ya existe una base de datos
            info!("Verificando si ya existe una base de datos...");
            if let Ok(db_path) = database::get_database_path() {
                if std::path::Path::new(&db_path).exists() {
                    info!("Base de datos existente encontrada, inicializando database_manager...");
                    match database::DatabaseManager::new_without_migrations(&db_path) {
                        Ok(db_manager) => {
                            info!("Database manager creado exitosamente");
                            // Obtener el estado y configurar el database_manager
                            let state = app.state::<AppState>();
                            let mut db_state = state.database_manager.lock()
                                .map_err(|_| "Error al acceder al database manager")?;
                            *db_state = Some(db_manager);
                            info!("Database manager configurado en el estado");
                        }
                        Err(e) => {
                            warn!("No se pudo crear database manager: {}", e);
                            info!("Continuando sin database manager inicializado");
                        }
                    }
                } else {
                    info!("No se encontró base de datos existente");
                }
            } else {
                info!("No se pudo obtener ruta de base de datos");
            }
            
            // Emitir evento de inicialización
            app_handle.emit_all("app-ready", ()).unwrap();
            
            // Inicializar el gestor de sincronización
            info!("Inicializando gestor de sincronización...");
            let sync_manager = sync::SyncManager::new_default();
            let state = app.state::<AppState>();
            let mut sync_state = state.sync_manager.lock()
                .map_err(|_| "Error al acceder al sync manager")?;
            *sync_state = Some(sync_manager);
            info!("Sync manager inicializado exitosamente");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Autenticación
            initialize_master_password,
            verify_master_password,
            change_master_password,
            generate_recovery_key,
            // reset_master_password_with_recovery,
            
            // TEST - Verificar migraciones
            test_migrations,
            
            // Gestión de contraseñas
            create_password_entry,
            get_password_entries,
            get_password_entry,
            update_password_entry,
            delete_password_entry,
            search_passwords,
            
            // Generador de contraseñas
            generate_password,
            check_password_strength,
            
            // Categorías
            create_category,
            get_categories,
            update_category,
            delete_category,
            
            // Utilidades
            export_passwords,
            import_passwords,
            get_statistics,
            
            // Autocompletado
            get_autocomplete_suggestions,
            save_autocomplete_data,
            get_active_browser_url,
            check_database_status,

            // Sincronización
            get_sync_config,
            get_sync_status,
            get_sync_devices,
            get_sync_stats,
            start_sync,
            stop_sync,
            start_device_discovery,
            sync_now,
            update_sync_config,
            trust_device,
            remove_device,
        ])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación");
}

// ===== COMANDOS DE AUTENTICACIÓN =====

#[tauri::command]
async fn initialize_master_password(
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("=== INICIO: Inicializando contraseña maestra ===");
    
    // Validar contraseña
    if password.len() < 8 {
        return Err("La contraseña debe tener al menos 8 caracteres".to_string());
    }
    
    info!("Contraseña validada, obteniendo ruta de base de datos...");
    info!("Llamando a database::get_database_path()...");
    let db_path = database::get_database_path()
        .map_err(|e| format!("Error al obtener ruta de base de datos: {}", e))?;
    info!("Ruta de base de datos obtenida: {}", db_path);
    
    info!("Verificando si el archivo de base de datos existe...");
    let db_exists = std::path::Path::new(&db_path).exists();
    info!("Archivo de base de datos existe: {}", db_exists);
    
    // EJECUTAR MIGRACIONES PRIMERO
    info!("=== EJECUTANDO MIGRACIONES ANTES DE CREAR DATABASE MANAGER ===");
    let connection = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Error al abrir conexión SQLite: {}", e))?;
    info!("Conexión SQLite abierta para migraciones");
    
    info!("Ejecutando migraciones...");
    database::run_migrations(&connection)
        .map_err(|e| format!("Error al ejecutar migraciones: {}", e))?;
    info!("Migraciones ejecutadas exitosamente");
    
    // Verificar que las migraciones se ejecutaron correctamente
    info!("Verificando que la tabla users existe después de las migraciones...");
    let users_table_exists = table_exists(&connection, "users");
    info!("Tabla users existe después de migraciones: {}", users_table_exists);
    
    if !users_table_exists {
        error!("ERROR CRÍTICO: La tabla users no existe después de las migraciones");
        return Err("Error: La tabla users no existe después de ejecutar las migraciones.".to_string());
    }
    
    info!("Verificando estructura de la tabla users...");
    let table_info = connection.query_row("PRAGMA table_info(users)", [], |row| {
        let name: String = row.get(1)?;
        let typ: String = row.get(2)?;
        Ok((name, typ))
    });
    match table_info {
        Ok(_) => info!("Estructura de tabla users verificada correctamente"),
        Err(e) => {
            error!("Error al verificar estructura de tabla users: {}", e);
            return Err(format!("Error al verificar estructura de tabla users: {}", e));
        }
    }
    
    // AHORA crear el DatabaseManager (que ya no necesita ejecutar migraciones)
    info!("Creando database manager (sin migraciones)...");
    let db_manager = database::DatabaseManager::new_without_migrations(&db_path)
        .map_err(|e| format!("Error al crear database manager: {}", e))?;
    info!("Database manager creado correctamente");
    
    info!("Obteniendo conexión a base de datos...");
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    // Obtener crypto manager
    info!("Obteniendo crypto manager...");
    let mut crypto_manager = state.crypto_manager.lock()
        .map_err(|_| "Error al acceder al crypto manager")?;
    info!("Crypto manager obtenido");
    
    // Generar salt y hash
    info!("Generando salt...");
    let salt = crypto::generate_salt();
    info!("Salt generado, longitud: {} bytes", salt.len());
    
    info!("Generando hash de contraseña...");
    let hash = crypto::hash_password(&password, &salt)
        .map_err(|e| format!("Error al generar hash: {}", e))?;
    info!("Hash generado correctamente");
    
    // Codificar salt como string para la base de datos
    info!("Codificando salt para base de datos...");
    let salt_encoded = base64::engine::general_purpose::STANDARD.encode(&salt);
    info!("Salt codificado correctamente");
    
    // Crear usuario
    info!("Creando usuario en base de datos...");
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    info!("Insertando usuario con ID: {}", user_id);
    conn.execute(
        "INSERT INTO users (id, master_password_hash, salt, created_at) VALUES (?, ?, ?, ?)",
        [&user_id, &hash, &salt_encoded, &now],
    ).map_err(|e| format!("Error al insertar usuario: {}", e))?;
    info!("Usuario insertado correctamente");
    
    // Configurar crypto manager
    info!("Configurando crypto manager...");
    crypto_manager.set_master_key(&password, &salt)
        .map_err(|e| format!("Error al configurar crypto manager: {}", e))?;
    info!("Crypto manager configurado correctamente");
    
    // Actualizar estado
    info!("Actualizando estado de la aplicación...");
    {
        let mut db_state = state.database_manager.lock()
            .map_err(|_| "Error al acceder al database manager del estado")?;
        *db_state = Some(db_manager);
    }
    info!("Estado de la aplicación actualizado");
    
    info!("=== FIN: Contraseña maestra inicializada correctamente ===");
    Ok(())
}

#[tauri::command]
async fn verify_master_password(
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    info!("🚨🚨🚨 COMANDO verify_master_password EJECUTÁNDOSE 🚨🚨🚨");
    info!("=== INICIO: Verificando contraseña maestra ===");
    info!("Longitud de contraseña recibida: {} caracteres", password.len());
    
    info!("🔍 Verificando estado del AppState...");
    info!("🔍 database_manager lock obtenido: {}", state.database_manager.try_lock().is_ok());
    
    if password.is_empty() {
        return Err("La contraseña no puede estar vacía".to_string());
    }
    
    info!("Obteniendo database manager...");
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    info!("Database manager guard obtenido");
    
    info!("Verificando si database_manager está presente en el estado...");
    if db_manager_guard.is_none() {
        error!("❌ Database manager es None en el estado");
        return Err("Base de datos no inicializada - database_manager es None".to_string());
    }
    info!("✅ Database manager presente en el estado");
    
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    info!("Base de datos inicializada correctamente");
    
    info!("Obteniendo conexión...");
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    info!("Preparando consulta...");
    let mut stmt = conn.prepare("SELECT master_password_hash, salt FROM users LIMIT 1")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    info!("Consulta preparada correctamente");
    
    info!("Ejecutando consulta...");
    let mut rows = stmt.query([])
        .map_err(|e| format!("Error al ejecutar consulta: {}", e))?;
    info!("Consulta ejecutada correctamente");
    
    info!("Leyendo fila...");
    if let Some(row) = rows.next().map_err(|e| format!("Error al leer fila: {}", e))? {
        info!("Fila encontrada en la base de datos");
        
        let hash: String = row.get(0)
            .map_err(|e| format!("Error al leer hash: {}", e))?;
        info!("Hash leído: {} caracteres", hash.len());
        
        let salt_base64: String = row.get(1)
            .map_err(|e| format!("Error al leer salt: {}", e))?;
        info!("Salt leído: {} caracteres", salt_base64.len());
        
        info!("Decodificando salt...");
        let salt = base64::engine::general_purpose::STANDARD.decode(&salt_base64)
            .map_err(|e| format!("Error al decodificar salt: {}", e))?;
        info!("Salt decodificado: {} bytes", salt.len());
        
        // Verificar contraseña usando la misma función que se usó para crear
        info!("Verificando contraseña usando crypto::verify_password...");
        info!("Hash almacenado en BD: {} caracteres", hash.len());
        info!("Salt decodificado: {} bytes", salt.len());
        
        let is_valid = crypto::verify_password(&password, &hash)
            .map_err(|e| {
                error!("❌ Error en crypto::verify_password: {}", e);
                format!("Error al verificar contraseña: {}", e)
            })?;
        info!("Resultado de verificación: {}", is_valid);
        
        if is_valid {
            info!("Contraseña válida, estableciendo clave maestra...");
            {
                let mut crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
                info!("Crypto manager obtenido correctamente");
                
                crypto_manager.set_master_key(&password, &salt)
                    .map_err(|e| format!("Error al establecer clave maestra: {}", e))?;
                info!("Clave maestra establecida correctamente");
                
                // Verificar que el crypto manager esté desbloqueado
                info!("Verificando estado del crypto manager...");
                if crypto_manager.is_unlocked() {
                    info!("✅ Crypto manager está desbloqueado correctamente");
                } else {
                    error!("❌ Crypto manager NO está desbloqueado después de set_master_key");
                }
            } // El lock se libera aquí
            
            // Verificar nuevamente el estado después de liberar el lock
            info!("Verificando estado del crypto manager después de liberar lock...");
            let crypto_manager_check = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
            if crypto_manager_check.is_unlocked() {
                info!("✅ Crypto manager sigue desbloqueado en el estado global");
            } else {
                error!("❌ Crypto manager NO está desbloqueado en el estado global");
            }
            
            info!("=== FIN: Contraseña maestra verificada correctamente ===");
            info!("Retornando true - login exitoso");
            Ok(true)
        } else {
            info!("=== FIN: Contraseña maestra incorrecta ===");
            info!("Retornando false - contraseña incorrecta");
            Ok(false)
        }
    } else {
        info!("No se encontró usuario en la base de datos");
        info!("=== FIN: No hay usuario para verificar ===");
        Err("No se encontró usuario en la base de datos. Debes crear una contraseña maestra primero.".to_string())
    }
}

#[tauri::command]
async fn change_master_password(
    _old_password: String,
    _new_password: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar cambio de contraseña maestra
    Ok(())
}

// ===== COMANDOS DE GESTIÓN DE CONTRASEÑAS =====

#[tauri::command]
async fn create_password_entry(
    request: models::CreatePasswordRequest,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    info!("🚨🚨🚨 COMANDO create_password_entry EJECUTÁNDOSE 🚨🚨🚨");
    info!("=== INICIO: Creando nueva entrada de contraseña ===");
    info!("Datos recibidos: title={}, username={}, password_length={}", 
          request.title, request.username, request.password.len());
    
    info!("Verificando crypto manager...");
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    info!("Crypto manager obtenido");
    
    info!("Verificando si crypto manager está desbloqueado...");
    if !crypto_manager.is_unlocked() {
        error!("❌ Crypto manager NO está desbloqueado en create_password_entry");
        return Err("Clave maestra no establecida. Debes hacer login primero.".to_string());
    }
    info!("✅ Crypto manager está desbloqueado correctamente");
    
    info!("Verificando database manager...");
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    info!("Database manager obtenido correctamente");
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    info!("ID generado: {}, timestamp: {}", id, now);
    
    info!("Encriptando datos sensibles...");
    let encrypted_password = crypto_manager.encrypt_data(request.password.as_bytes())
        .map_err(|e| format!("Error al encriptar contraseña: {}", e))?;
    info!("Contraseña encriptada correctamente");
    
    let encrypted_username = crypto_manager.encrypt_data(request.username.as_bytes())
        .map_err(|e| format!("Error al encriptar usuario: {}", e))?;
    info!("Usuario encriptado correctamente");
    
    let encrypted_title = crypto_manager.encrypt_data(request.title.as_bytes())
        .map_err(|e| format!("Error al encriptar título: {}", e))?;
    info!("Título encriptado correctamente");
    
    info!("Guardando en base de datos...");
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    // Manejar category_id correctamente para evitar errores de clave foránea
    let category_id: Option<&str> = request.category_id.as_ref()
        .filter(|&id| !id.is_empty())
        .map(|x| x.as_str());
    
    info!("Category ID a insertar: {:?}", category_id);
    
    // Usar rusqlite::params! para manejar Option correctamente
    conn.execute(
        "INSERT INTO password_entries (id, title, username, password, url, notes, category_id, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            id,
            serde_json::to_string(&encrypted_title).unwrap(),
            serde_json::to_string(&encrypted_username).unwrap(),
            serde_json::to_string(&encrypted_password).unwrap(),
            request.url.unwrap_or_default(),
            request.notes.unwrap_or_default(),
            category_id,
            serde_json::to_string(&request.tags).unwrap(),
            now,
            now,
        ],
    ).map_err(|e| format!("Error al guardar entrada: {}", e))?;
    
    info!("=== FIN: Entrada de contraseña creada exitosamente con ID: {} ===", id);
    Ok(id)
}

#[tauri::command]
async fn get_password_entries(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<models::PasswordEntry>, String> {
    info!("=== INICIO: Obteniendo entradas de contraseñas ===");
    
    info!("Verificando crypto manager...");
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    info!("Crypto manager obtenido");
    
    info!("Verificando si crypto manager está desbloqueado...");
    if !crypto_manager.is_unlocked() {
        error!("Crypto manager NO está desbloqueado");
        return Err("Clave maestra no establecida. Debes hacer login primero.".to_string());
    }
    info!("Crypto manager está desbloqueado correctamente");
    
    info!("Verificando database manager...");
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    info!("Database manager obtenido correctamente");
    
    info!("Obteniendo conexión a base de datos...");
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    let mut stmt = conn.prepare("SELECT id, title, username, password, url, notes, category_id, tags, created_at, updated_at, last_used FROM password_entries ORDER BY updated_at DESC")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    
    let mut entries = Vec::new();
    let mut rows = stmt.query([])
        .map_err(|e| format!("Error al ejecutar consulta: {}", e))?;
    
    while let Some(row) = rows.next().map_err(|e| format!("Error al leer fila: {}", e))? {
        let encrypted_title: String = row.get(1)
            .map_err(|e| format!("Error al leer título: {}", e))?;
        let encrypted_username: String = row.get(2)
            .map_err(|e| format!("Error al leer usuario: {}", e))?;
        let encrypted_password: String = row.get(3)
            .map_err(|e| format!("Error al leer contraseña: {}", e))?;
        
        // Desencriptar datos
        let encrypted_title_data: crypto::EncryptedData = serde_json::from_str(&encrypted_title)
            .map_err(|e| format!("Error al parsear título: {}", e))?;
        let encrypted_username_data: crypto::EncryptedData = serde_json::from_str(&encrypted_username)
            .map_err(|e| format!("Error al parsear usuario: {}", e))?;
        let encrypted_password_data: crypto::EncryptedData = serde_json::from_str(&encrypted_password)
            .map_err(|e| format!("Error al parsear contraseña: {}", e))?;
        
        let title = String::from_utf8(crypto_manager.decrypt_data(&encrypted_title_data)
            .map_err(|e| format!("Error al desencriptar título: {}", e))?)
            .map_err(|e| format!("Error al convertir título: {}", e))?;
        
        let username = String::from_utf8(crypto_manager.decrypt_data(&encrypted_username_data)
            .map_err(|e| format!("Error al desencriptar usuario: {}", e))?)
            .map_err(|e| format!("Error al convertir usuario: {}", e))?;
        
        let password = String::from_utf8(crypto_manager.decrypt_data(&encrypted_password_data)
            .map_err(|e| format!("Error al desencriptar contraseña: {}", e))?)
            .map_err(|e| format!("Error al convertir contraseña: {}", e))?;
        
        let entry = models::PasswordEntry {
            id: row.get::<_, String>(0).unwrap(),
            title,
            username,
            password,
            url: Some(row.get::<_, String>(4).unwrap()),
            notes: Some(row.get::<_, String>(5).unwrap()),
            category_id: row.get::<_, Option<String>>(6).unwrap_or(None),
            tags: serde_json::from_str(&row.get::<_, String>(7).unwrap()).unwrap_or_default(),
            created_at: row.get::<_, String>(8).unwrap(),
            updated_at: row.get::<_, String>(9).unwrap(),
            last_used: row.get::<_, Option<String>>(10).unwrap_or(None),
        };
        
        entries.push(entry);
    }
    
    info!("Obtenidas {} entradas de contraseñas", entries.len());
    Ok(entries)
}

#[tauri::command]
async fn get_password_entry(
    _id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<models::PasswordEntry, String> {
    // TODO: Implementar obtención de entrada específica
    Err("No implementado".to_string())
}

#[tauri::command]
async fn update_password_entry(
    _request: models::UpdatePasswordRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar actualización de entrada
    Ok(())
}

#[tauri::command]
async fn delete_password_entry(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("🚨🚨🚨 COMANDO delete_password_entry EJECUTÁNDOSE 🚨🚨🚨");
    info!("=== INICIO: Eliminando entrada de contraseña ===");
    info!("ID a eliminar: {}", id);
    
    info!("Verificando crypto manager...");
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    info!("Crypto manager obtenido");
    
    info!("Verificando si crypto manager está desbloqueado...");
    if !crypto_manager.is_unlocked() {
        error!("❌ Crypto manager NO está desbloqueado en delete_password_entry");
        return Err("Clave maestra no establecida. Debes hacer login primero.".to_string());
    }
    info!("✅ Crypto manager está desbloqueado correctamente");
    
    info!("Verificando database manager...");
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    info!("Database manager obtenido correctamente");
    
    info!("Eliminando entrada de la base de datos...");
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    let rows_affected = conn.execute(
        "DELETE FROM password_entries WHERE id = ?",
        rusqlite::params![id]
    ).map_err(|e| format!("Error al eliminar entrada: {}", e))?;
    
    if rows_affected == 0 {
        info!("⚠️ No se encontró entrada con ID: {}", id);
        return Err("No se encontró la entrada de contraseña".to_string());
    }
    
    info!("✅ Entrada eliminada exitosamente. Filas afectadas: {}", rows_affected);
    info!("=== FIN: Entrada de contraseña eliminada exitosamente ===");
    Ok(())
}

#[tauri::command]
async fn search_passwords(
    _request: models::SearchRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<Vec<models::PasswordEntry>, String> {
    // TODO: Implementar búsqueda
    Ok(Vec::new())
}

// ===== GENERADOR DE CONTRASEÑAS =====

#[tauri::command]
async fn generate_password(
    request: models::PasswordGenerationRequest,
) -> Result<String, String> {
    info!("Generando contraseña segura...");
    
    let password = crypto::generate_secure_password(request.length);
    
    info!("Contraseña generada exitosamente");
    Ok(password)
}

#[tauri::command]
async fn check_password_strength(
    password: String,
) -> Result<serde_json::Value, String> {
    info!("Verificando fortaleza de contraseña...");
    
    let mut score = 0;
    let mut feedback = Vec::new();
    let mut suggestions = Vec::new();
    
    // Verificar longitud
    if password.len() >= 12 {
        score += 2;
    } else if password.len() >= 8 {
        score += 1;
        suggestions.push("Usa al menos 12 caracteres para mayor seguridad");
    } else {
        feedback.push("La contraseña es muy corta");
        suggestions.push("Usa al menos 8 caracteres");
    }
    
    // Verificar mayúsculas
    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    } else {
        suggestions.push("Incluye al menos una letra mayúscula");
    }
    
    // Verificar minúsculas
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    } else {
        suggestions.push("Incluye al menos una letra minúscula");
    }
    
    // Verificar números
    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    } else {
        suggestions.push("Incluye al menos un número");
    }
    
    // Verificar símbolos
    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    } else {
        suggestions.push("Incluye al menos un símbolo especial");
    }
    
    // Verificar patrones comunes
    if password.to_lowercase().contains("password") || 
       password.to_lowercase().contains("123") ||
       password.to_lowercase().contains("qwerty") {
        score -= 2;
        feedback.push("Evita patrones comunes y secuencias");
        suggestions.push("No uses palabras o secuencias comunes");
    }
    
    // Normalizar score a 0-100
    let normalized_score = ((score as f32 / 6.0) * 100.0).max(0.0).min(100.0) as u8;
    
    let result = serde_json::json!({
        "score": normalized_score,
        "feedback": feedback,
        "suggestions": suggestions
    });
    
    info!("Fortaleza de contraseña verificada: {}%", normalized_score);
    Ok(result)
}

// ===== CATEGORÍAS =====

#[tauri::command]
async fn create_category(
    _name: String,
    _color: String,
    _state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementar creación de categoría
    Ok("".to_string())
}

#[tauri::command]
async fn get_categories(
    _state: tauri::State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implementar obtención de categorías
    Ok(Vec::new())
}

#[tauri::command]
async fn update_category(
    _id: String,
    _name: String,
    _color: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar actualización de categoría
    Ok(())
}

#[tauri::command]
async fn delete_category(
    _id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar eliminación de categoría
    Ok(())
}

// ===== UTILIDADES =====

#[tauri::command]
async fn export_passwords(
    _state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementar exportación
    Ok("".to_string())
}

#[tauri::command]
async fn import_passwords(
    _data: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar importación
    Ok(())
}

#[tauri::command]
async fn get_statistics(
    _state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // TODO: Implementar estadísticas
    Ok(serde_json::json!({
        "total_passwords": 0,
        "weak_passwords": 0,
        "strong_passwords": 0,
        "security_score": 0
    }))
}

// ===== AUTOMÁTICO COMPLETADO =====

#[tauri::command]
async fn get_autocomplete_suggestions(
    request: models::AutofillRequest,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    info!("Obteniendo sugerencias de autocompletado para: {}", request.url);
    
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    // Buscar entradas que coincidan con la URL
    let conn = db_manager.get_connection();
    let mut stmt = conn.prepare("SELECT title, username, password FROM password_entries WHERE url LIKE ? OR title LIKE ?")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    
    let search_pattern = format!("%{}%", request.url);
    let mut rows = stmt.query([&search_pattern, &search_pattern])
        .map_err(|e| format!("Error al ejecutar consulta: {}", e))?;
    
    let mut suggestions = Vec::new();
    while let Some(row) = rows.next().map_err(|e| format!("Error al leer fila: {}", e))? {
        let encrypted_title: String = row.get(0).unwrap();
        let encrypted_username: String = row.get(1).unwrap();
        let encrypted_password: String = row.get(2).unwrap();
        
        // Desencriptar datos
        let encrypted_title_data: crypto::EncryptedData = serde_json::from_str(&encrypted_title)
            .map_err(|e| format!("Error al parsear título: {}", e))?;
        let encrypted_username_data: crypto::EncryptedData = serde_json::from_str(&encrypted_username)
            .map_err(|e| format!("Error al parsear usuario: {}", e))?;
        let encrypted_password_data: crypto::EncryptedData = serde_json::from_str(&encrypted_password)
            .map_err(|e| format!("Error al parsear contraseña: {}", e))?;
        
        let title = String::from_utf8(crypto_manager.decrypt_data(&encrypted_title_data)
            .map_err(|e| format!("Error al desencriptar título: {}", e))?)
            .map_err(|e| format!("Error al convertir título: {}", e))?;
        
        let username = String::from_utf8(crypto_manager.decrypt_data(&encrypted_username_data)
            .map_err(|e| format!("Error al desencriptar usuario: {}", e))?)
            .map_err(|e| format!("Error al convertir usuario: {}", e))?;
        
        let password = String::from_utf8(crypto_manager.decrypt_data(&encrypted_password_data)
            .map_err(|e| format!("Error al desencriptar contraseña: {}", e))?)
            .map_err(|e| format!("Error al convertir contraseña: {}", e))?;
        
        let suggestion = serde_json::json!({
            "title": title,
            "username": username,
            "password": password
        });
        
        suggestions.push(suggestion);
    }
    
    info!("Encontradas {} sugerencias de autocompletado", suggestions.len());
    Ok(suggestions)
}

#[tauri::command]
async fn save_autocomplete_data(
    _url: String,
    _username: String,
    _password: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar guardado de datos de autocompletado
    Ok(())
} 

#[tauri::command]
async fn get_active_browser_url() -> Result<String, String> {
    // Por ahora retornamos una URL de ejemplo
    // En una implementación real, esto requeriría permisos del sistema
    // para detectar la ventana activa del navegador
    Ok("https://example.com".to_string())
} 

#[tauri::command]
async fn generate_recovery_key(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    info!("Generando clave de recuperación...");
    
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    
    if !crypto_manager.is_unlocked() {
        return Err("Debes estar autenticado para generar una clave de recuperación".to_string());
    }
    
    // Generar clave de recuperación aleatoria
    let recovery_key = crypto::generate_recovery_key()
        .map_err(|e| format!("Error al generar clave de recuperación: {}", e))?;
    
    info!("Clave de recuperación generada correctamente");
    Ok(recovery_key)
}

#[tauri::command]
async fn check_database_status(_state: tauri::State<'_, AppState>) -> Result<bool, String> {
    info!("=== INICIO: Verificando estado de la base de datos ===");
    
    // Crear un nuevo database manager temporal solo para verificar
    let db_path = database::get_database_path()
        .map_err(|e| format!("Error al obtener ruta de BD: {}", e))?;
    info!("Ruta de base de datos obtenida: {}", db_path);
    
    let db_manager = database::DatabaseManager::new(&db_path)
        .map_err(|e| format!("Error al crear database manager: {}", e))?;
    info!("Database manager creado exitosamente");
    
    // Usar la nueva función de verificación
    let is_initialized = db_manager.check_database_status()
        .map_err(|e| format!("Error al verificar estado de BD: {}", e))?;
    
    info!("Estado de inicialización: {}", is_initialized);
    info!("=== FIN: Verificación completada ===");
    Ok(is_initialized)
}

// #[tauri::command]
// async fn reset_master_password_with_recovery(
//     recovery_key: String,
//     new_password: String,
//     state: tauri::State<'_, AppState>,
// ) -> Result<(), String> {
//     // TODO: Implementar cuando se corrijan los errores de tipos
//     Ok(())
// } 

// ===== COMANDO DE TEST =====

#[tauri::command]
async fn test_migrations() -> Result<String, String> {
    info!("=== INICIO: TEST DE MIGRACIONES ===");
    
    // Obtener ruta de base de datos
    let db_path = database::get_database_path()
        .map_err(|e| format!("Error al obtener ruta de base de datos: {}", e))?;
    info!("Ruta de base de datos: {}", db_path);
    
    // Crear conexión
    let connection = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Error al abrir conexión SQLite: {}", e))?;
    info!("Conexión SQLite abierta");
    
    // Ejecutar migraciones
    info!("Ejecutando migraciones...");
    database::run_migrations(&connection)
        .map_err(|e| format!("Error al ejecutar migraciones: {}", e))?;
    info!("Migraciones ejecutadas");
    
    // Verificar tablas
    let tables = connection.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
        [],
        |row| row.get::<_, i64>(0)
    ).map_err(|e| format!("Error al consultar tablas: {}", e))?;
    info!("Número de tablas: {}", tables);
    
    // Verificar tabla users específicamente
    let users_exists = table_exists(&connection, "users");
    info!("Tabla users existe: {}", users_exists);
    
    if users_exists {
        let user_count = connection.query_row(
            "SELECT COUNT(*) FROM users",
            [],
            |row| row.get::<_, i64>(0)
        ).map_err(|e| format!("Error al contar usuarios: {}", e))?;
        info!("Número de usuarios: {}", user_count);
    }
    
    info!("=== FIN: TEST DE MIGRACIONES COMPLETADO ===");
    Ok("Migraciones funcionando correctamente".to_string())
} 