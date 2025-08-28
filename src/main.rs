// Pre-requisites
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod crypto;
mod database;
mod models;

use tauri::Manager;
use log::{info, error};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

// Estructuras de datos para la comunicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePasswordRequest {
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub id: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGenerationRequest {
    pub length: usize,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub exclude_similar: bool,
}

// Estado global de la aplicación
struct AppState {
    crypto_manager: Mutex<crypto::CryptoManager>,
    database_manager: Mutex<Option<database::DatabaseManager>>,
    is_initialized: Mutex<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            crypto_manager: Mutex::new(crypto::CryptoManager::new()),
            database_manager: Mutex::new(None),
            is_initialized: Mutex::new(false),
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
            
            // Emitir evento de inicialización
            app_handle.emit_all("app-ready", ()).unwrap();
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Autenticación
            initialize_master_password,
            verify_master_password,
            change_master_password,
            
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
    info!("Inicializando contraseña maestra...");
    
    let mut crypto_manager = state.crypto_manager.lock().unwrap();
    let salt = crypto::generate_salt();
    
    // Crear hash de la contraseña maestra
    let hash = crypto::hash_password(&password, &salt)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    
    // Establecer la clave maestra
    crypto_manager.set_master_key(&password, &salt)
        .map_err(|e| format!("Error al establecer clave maestra: {}", e))?;
    
    // Inicializar base de datos
    let db_path = database::get_database_path()
        .map_err(|e| format!("Error al obtener ruta de BD: {}", e))?;
    
    let db_manager = database::DatabaseManager::new(&db_path)
        .map_err(|e| format!("Error al inicializar BD: {}", e))?;
    
    // Guardar hash y salt en la base de datos
    let conn = db_manager.get_connection();
    conn.execute(
        "INSERT OR REPLACE INTO users (id, master_password_hash, salt, created_at) VALUES (?, ?, ?, ?)",
        [&Uuid::new_v4().to_string(), &hash, &salt, &Utc::now().to_rfc3339()],
    ).map_err(|e| format!("Error al guardar usuario: {}", e))?;
    
    // Actualizar estado
    *state.database_manager.lock().unwrap() = Some(db_manager);
    *state.is_initialized.lock().unwrap() = true;
    
    info!("Contraseña maestra inicializada correctamente");
    Ok(())
}

#[tauri::command]
async fn verify_master_password(
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    info!("Verificando contraseña maestra...");
    
    let db_manager = state.database_manager.lock().unwrap()
        .as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    let conn = db_manager.get_connection();
    let mut stmt = conn.prepare("SELECT master_password_hash, salt FROM users LIMIT 1")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    
    let mut rows = stmt.query([])
        .map_err(|e| format!("Error al ejecutar consulta: {}", e))?;
    
    if let Some(row) = rows.next().map_err(|e| format!("Error al leer fila: {}", e))? {
        let hash: String = row.get(0)
            .map_err(|e| format!("Error al leer hash: {}", e))?;
        let salt: Vec<u8> = row.get(1)
            .map_err(|e| format!("Error al leer salt: {}", e))?;
        
        // Verificar contraseña
        let is_valid = crypto::verify_password(&password, &hash)
            .map_err(|e| format!("Error al verificar contraseña: {}", e))?;
        
        if is_valid {
            let mut crypto_manager = state.crypto_manager.lock().unwrap();
            crypto_manager.set_master_key(&password, &salt)
                .map_err(|e| format!("Error al establecer clave maestra: {}", e))?;
            
            info!("Contraseña maestra verificada correctamente");
            Ok(true)
        } else {
            info!("Contraseña maestra incorrecta");
            Ok(false)
        }
    } else {
        Err("No se encontró usuario en la base de datos".to_string())
    }
}

// ===== COMANDOS DE GESTIÓN DE CONTRASEÑAS =====

#[tauri::command]
async fn create_password_entry(
    request: CreatePasswordRequest,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    info!("Creando nueva entrada de contraseña...");
    
    let crypto_manager = state.crypto_manager.lock().unwrap();
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager = state.database_manager.lock().unwrap()
        .as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Encriptar datos sensibles
    let encrypted_password = crypto_manager.encrypt_data(request.password.as_bytes())
        .map_err(|e| format!("Error al encriptar contraseña: {}", e))?;
    
    let encrypted_username = crypto_manager.encrypt_data(request.username.as_bytes())
        .map_err(|e| format!("Error al encriptar usuario: {}", e))?;
    
    let encrypted_title = crypto_manager.encrypt_data(request.title.as_bytes())
        .map_err(|e| format!("Error al encriptar título: {}", e))?;
    
    // Guardar en base de datos
    let conn = db_manager.get_connection();
    conn.execute(
        "INSERT INTO password_entries (id, title, username, password, url, notes, category_id, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
            &id,
            &serde_json::to_string(&encrypted_title).unwrap(),
            &serde_json::to_string(&encrypted_username).unwrap(),
            &serde_json::to_string(&encrypted_password).unwrap(),
            &request.url.unwrap_or_default(),
            &request.notes.unwrap_or_default(),
            &request.category_id.unwrap_or_default(),
            &serde_json::to_string(&request.tags).unwrap(),
            &now,
            &now,
        ],
    ).map_err(|e| format!("Error al guardar entrada: {}", e))?;
    
    info!("Entrada de contraseña creada con ID: {}", id);
    Ok(id)
}

#[tauri::command]
async fn get_password_entries(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<PasswordEntry>, String> {
    info!("Obteniendo entradas de contraseñas...");
    
    let crypto_manager = state.crypto_manager.lock().unwrap();
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager = state.database_manager.lock().unwrap()
        .as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    let conn = db_manager.get_connection();
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
        
        let entry = PasswordEntry {
            id: row.get(0).unwrap(),
            title,
            username,
            password,
            url: row.get(4).unwrap(),
            notes: row.get(5).unwrap(),
            category_id: row.get(6).unwrap(),
            tags: serde_json::from_str(&row.get::<_, String>(7).unwrap()).unwrap_or_default(),
            created_at: row.get(8).unwrap(),
            updated_at: row.get(9).unwrap(),
            last_used: row.get(10).unwrap(),
        };
        
        entries.push(entry);
    }
    
    info!("Obtenidas {} entradas de contraseñas", entries.len());
    Ok(entries)
}

// ===== GENERADOR DE CONTRASEÑAS =====

#[tauri::command]
async fn generate_password(
    request: PasswordGenerationRequest,
) -> Result<String, String> {
    info!("Generando contraseña segura...");
    
    let password = crypto::generate_secure_password_custom(
        request.length,
        request.include_uppercase,
        request.include_lowercase,
        request.include_numbers,
        request.include_symbols,
        request.exclude_similar,
    );
    
    info!("Contraseña generada exitosamente");
    Ok(password)
}

#[tauri::command]
async fn check_password_strength(
    password: String,
) -> Result<HashMap<String, serde_json::Value>, String> {
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
    
    let result = HashMap::from([
        ("score".to_string(), serde_json::Value::Number(normalized_score.into())),
        ("feedback".to_string(), serde_json::Value::Array(feedback.into_iter().map(|f| serde_json::Value::String(f)).collect())),
        ("suggestions".to_string(), serde_json::Value::Array(suggestions.into_iter().map(|s| serde_json::Value::String(s)).collect())),
    ]);
    
    info!("Fortaleza de contraseña verificada: {}%", normalized_score);
    Ok(result)
}

// ===== AUTOMÁTICO COMPLETADO =====

#[tauri::command]
async fn get_autocomplete_suggestions(
    url: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<HashMap<String, String>>, String> {
    info!("Obteniendo sugerencias de autocompletado para: {}", url);
    
    let crypto_manager = state.crypto_manager.lock().unwrap();
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager = state.database_manager.lock().unwrap()
        .as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    // Buscar entradas que coincidan con la URL
    let conn = db_manager.get_connection();
    let mut stmt = conn.prepare("SELECT title, username, password FROM password_entries WHERE url LIKE ? OR title LIKE ?")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    
    let search_pattern = format!("%{}%", url);
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
        
        let suggestion = HashMap::from([
            ("title".to_string(), title),
            ("username".to_string(), username),
            ("password".to_string(), password),
        ]);
        
        suggestions.push(suggestion);
    }
    
    info!("Encontradas {} sugerencias de autocompletado", suggestions.len());
    Ok(suggestions)
}

// Implementar comandos restantes...
#[tauri::command]
async fn change_master_password(
    _old_password: String,
    _new_password: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar cambio de contraseña maestra
    Ok(())
}

#[tauri::command]
async fn get_password_entry(
    _id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<PasswordEntry, String> {
    // TODO: Implementar obtención de entrada específica
    Err("No implementado".to_string())
}

#[tauri::command]
async fn update_password_entry(
    _request: UpdatePasswordRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar actualización de entrada
    Ok(())
}

#[tauri::command]
async fn delete_password_entry(
    _id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar eliminación de entrada
    Ok(())
}

#[tauri::command]
async fn search_passwords(
    _request: SearchRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<Vec<PasswordEntry>, String> {
    // TODO: Implementar búsqueda
    Ok(Vec::new())
}

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
) -> Result<Vec<HashMap<String, String>>, String> {
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
) -> Result<HashMap<String, serde_json::Value>, String> {
    // TODO: Implementar estadísticas
    Ok(HashMap::new())
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