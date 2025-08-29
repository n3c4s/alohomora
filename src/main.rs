// Pre-requisitos
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod crypto;
mod database;
mod models;

use tauri::Manager;
use std::sync::Mutex;
use serde_json;
use base64::Engine;
use log::{info, error};
use env_logger;

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
            // generate_recovery_key,
            // reset_master_password_with_recovery,
            
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
    
    if password.is_empty() {
        return Err("La contraseña no puede estar vacía".to_string());
    }
    
    info!("Longitud de contraseña: {} caracteres", password.len());
    
    let mut crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    info!("Crypto manager obtenido correctamente");
    
    // Generar salt y hash
    let salt = crypto::generate_salt();
    info!("Salt generado: {} bytes", salt.len());
    
    let hash = crypto::hash_password(&password, &salt)
        .map_err(|e| format!("Error al generar hash: {}", e))?;
    info!("Hash generado correctamente");
    
    // Crear base de datos
    let db_path = database::get_database_path()
        .map_err(|e| format!("Error al obtener ruta de BD: {}", e))?;
    
    let db_manager = database::DatabaseManager::new(&db_path)
        .map_err(|e| format!("Error al crear database manager: {}", e))?;
    info!("Database manager creado correctamente");
    
    // Guardar hash y salt en la base de datos
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    let salt_encoded = base64::engine::general_purpose::STANDARD.encode(&salt);
    info!("Salt codificado en base64: {} caracteres", salt_encoded.len());
    
    let result = conn.execute(
        "INSERT OR REPLACE INTO users (id, master_password_hash, salt, created_at) VALUES (?, ?, ?, ?)",
        ["user_1", &hash, &salt_encoded, "2024-01-01T00:00:00Z"],
    );
    
    match result {
        Ok(_) => info!("Usuario guardado en base de datos correctamente"),
        Err(e) => {
            error!("Error al guardar en base de datos: {:?}", e);
            return Err(format!("Error al guardar usuario en base de datos: {}", e));
        }
    }
    
    // Actualizar estado
    *state.database_manager.lock().map_err(|_| "Error al acceder al database manager")? = Some(db_manager);
    *state.is_initialized.lock().map_err(|_| "Error al acceder al estado")? = true;
    
    info!("Contraseña maestra inicializada correctamente");
    Ok(())
}

#[tauri::command]
async fn verify_master_password(
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    info!("Verificando contraseña maestra...");
    info!("Longitud de contraseña recibida: {} caracteres", password.len());
    
    if password.is_empty() {
        return Err("La contraseña no puede estar vacía".to_string());
    }
    
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    info!("Database manager obtenido correctamente");
    
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    info!("Base de datos inicializada correctamente");
    
    let conn = db_manager.get_connection();
    info!("Conexión a base de datos obtenida");
    
    let mut stmt = conn.prepare("SELECT master_password_hash, salt FROM users LIMIT 1")
        .map_err(|e| format!("Error al preparar consulta: {}", e))?;
    info!("Consulta preparada correctamente");
    
    let mut rows = stmt.query([])
        .map_err(|e| format!("Error al ejecutar consulta: {}", e))?;
    info!("Consulta ejecutada correctamente");
    
    if let Some(row) = rows.next().map_err(|e| format!("Error al leer fila: {}", e))? {
        info!("Fila encontrada en la base de datos");
        
        let hash: String = row.get(0)
            .map_err(|e| format!("Error al leer hash: {}", e))?;
        info!("Hash leído: {} caracteres", hash.len());
        
        let salt_base64: String = row.get(1)
            .map_err(|e| format!("Error al leer salt: {}", e))?;
        info!("Salt leído: {} caracteres", salt_base64.len());
        
        let salt = base64::engine::general_purpose::STANDARD.decode(&salt_base64)
            .map_err(|e| format!("Error al decodificar salt: {}", e))?;
        info!("Salt decodificado: {} bytes", salt.len());
        
        // Verificar contraseña
        info!("Llamando a verify_password...");
        let is_valid = crypto::verify_password(&password, &hash)
            .map_err(|e| format!("Error al verificar contraseña: {}", e))?;
        info!("Resultado de verificación: {}", is_valid);
        
        if is_valid {
            info!("Contraseña válida, estableciendo clave maestra...");
            let mut crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
            info!("Crypto manager obtenido correctamente");
            
            crypto_manager.set_master_key(&password, &salt)
                .map_err(|e| format!("Error al establecer clave maestra: {}", e))?;
            info!("Clave maestra establecida correctamente");
            
            info!("Contraseña maestra verificada correctamente");
            Ok(true)
        } else {
            info!("Contraseña maestra incorrecta");
            Ok(false)
        }
    } else {
        info!("No se encontró usuario en la base de datos");
        Err("No se encontró usuario en la base de datos".to_string())
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
    info!("Creando nueva entrada de contraseña...");
    
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
        .ok_or("Base de datos no inicializada")?;
    
    let id = "entry_1".to_string();
    let now = "2024-01-01T00:00:00Z".to_string();
    
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
) -> Result<Vec<models::PasswordEntry>, String> {
    info!("Obteniendo entradas de contraseñas...");
    
    let crypto_manager = state.crypto_manager.lock().map_err(|_| "Error al acceder al crypto manager")?;
    if !crypto_manager.is_unlocked() {
        return Err("Clave maestra no establecida".to_string());
    }
    
    let db_manager_guard = state.database_manager.lock().map_err(|_| "Error al acceder al database manager")?;
    let db_manager = db_manager_guard.as_ref()
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
        
        let entry = models::PasswordEntry {
            id: row.get::<_, String>(0).unwrap(),
            title,
            username,
            password,
            url: Some(row.get::<_, String>(4).unwrap()),
            notes: Some(row.get::<_, String>(5).unwrap()),
            category_id: Some(row.get::<_, String>(6).unwrap()),
            tags: serde_json::from_str(&row.get::<_, String>(7).unwrap()).unwrap_or_default(),
            created_at: row.get::<_, String>(8).unwrap(),
            updated_at: row.get::<_, String>(9).unwrap(),
            last_used: Some(row.get::<_, String>(10).unwrap()),
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
    _id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementar eliminación de entrada
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

// #[tauri::command]
// async fn generate_recovery_key(
//     password: String,
//     state: tauri::State<'_, AppState>,
// ) -> Result<String, String> {
//     // TODO: Implementar cuando se corrijan los errores de tipos
//     Ok("".to_string())
// }

// #[tauri::command]
// async fn reset_master_password_with_recovery(
//     recovery_key: String,
//     new_password: String,
//     state: tauri::State<'_, AppState>,
// ) -> Result<(), String> {
//     // TODO: Implementar cuando se corrijan los errores de tipos
//     Ok(())
// } 