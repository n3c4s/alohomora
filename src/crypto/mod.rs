mod encryption;
mod key_derivation;

pub use encryption::*;
pub use key_derivation::*;

use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
use chacha20poly1305::aead::Aead;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use base64::Engine;
use rand::{Rng, RngCore};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use log::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterKey {
    pub hash: String,
    pub salt: Vec<u8>,
}

pub struct CryptoManager {
    master_key: Option<Vec<u8>>,
}

impl CryptoManager {
    pub fn new() -> Self {
        Self { master_key: None }
    }
    
    pub fn set_master_key(&mut self, password: &str, salt: &[u8]) -> Result<(), String> {
        info!("🔄 CryptoManager: Iniciando set_master_key...");
        info!("🔄 CryptoManager: Longitud de contraseña: {} caracteres", password.len());
        info!("🔄 CryptoManager: Longitud de salt: {} bytes", salt.len());
        
        info!("🔄 CryptoManager: Llamando a derive_key_from_password...");
        let key = derive_key_from_password(password, salt)?;
        info!("✅ CryptoManager: Clave derivada correctamente, longitud: {} bytes", key.len());
        
        info!("🔄 CryptoManager: Estableciendo master_key...");
        self.master_key = Some(key);
        info!("✅ CryptoManager: master_key establecido correctamente");
        
        info!("🔄 CryptoManager: Verificando estado...");
        if self.is_unlocked() {
            info!("✅ CryptoManager: Estado verificado - está desbloqueado");
        } else {
            error!("❌ CryptoManager: Estado verificado - NO está desbloqueado");
        }
        
        Ok(())
    }
    
    pub fn is_unlocked(&self) -> bool {
        let unlocked = self.master_key.is_some();
        info!("🔍 CryptoManager: is_unlocked() llamado - resultado: {}", unlocked);
        if unlocked {
            info!("🔍 CryptoManager: master_key presente, longitud: {} bytes", 
                  self.master_key.as_ref().unwrap().len());
        } else {
            info!("🔍 CryptoManager: master_key NO presente");
        }
        unlocked
    }
    
    pub fn encrypt_data(&self, data: &[u8]) -> Result<EncryptedData> {
        let master_key = self.master_key.as_ref()
            .ok_or_else(|| anyhow!("Master key no establecida"))?;
        
        let key = Key::from_slice(master_key);
        let cipher = ChaCha20Poly1305::new(key);
        
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let mut salt_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt_bytes);
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| anyhow!("Error al encriptar: {}", e))?;
        
        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
            salt: salt_bytes.to_vec(),
        })
    }
    
    pub fn decrypt_data(&self, encrypted_data: &EncryptedData) -> Result<Vec<u8>> {
        let master_key = self.master_key.as_ref()
            .ok_or_else(|| anyhow!("Master key no establecida"))?;
        
        let key = Key::from_slice(master_key);
        let cipher = ChaCha20Poly1305::new(key);
        
        let nonce = Nonce::from_slice(&encrypted_data.nonce);
        
        let plaintext = cipher.decrypt(nonce, encrypted_data.ciphertext.as_slice())
            .map_err(|e| anyhow!("Error al desencriptar: {}", e))?;
        
        Ok(plaintext)
    }
    
    pub fn lock(&mut self) {
        self.master_key = None;
    }

    pub fn unlock(&mut self, password: &str, salt: &[u8]) -> Result<(), String> {
        let key = derive_key_from_password(password, salt)?;
        self.master_key = Some(key);
        Ok(())
    }
}

// Funciones estáticas del módulo
pub fn generate_recovery_key() -> Result<String, String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Generar 32 bytes aleatorios
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    
    // Convertir a base64
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

pub fn encrypt_with_recovery_key(data: &str, recovery_key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key_bytes = hex::decode(recovery_key)?;
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(b"recovery_nonce");
    
    let encrypted = cipher.encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Error al encriptar: {}", e))?;
    
    Ok(encrypted)
}

pub fn decrypt_with_recovery_key(encrypted_data: &[u8], recovery_key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key_bytes = hex::decode(recovery_key)?;
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(b"recovery_nonce");
    
    let encrypted = cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| format!("Error al desencriptar: {}", e))?;
    
    Ok(encrypted)
}

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    info!("🔄 derive_key_from_password: Iniciando...");
    info!("🔄 derive_key_from_password: Longitud de contraseña: {} caracteres", password.len());
    info!("🔄 derive_key_from_password: Longitud de salt: {} bytes", salt.len());
    
    info!("🔄 derive_key_from_password: Creando configuración Argon2...");
    let config = Argon2::default();
    info!("✅ derive_key_from_password: Configuración Argon2 creada");
    
    info!("🔄 derive_key_from_password: Codificando salt a base64...");
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| format!("Error al codificar salt: {}", e))?;
    info!("✅ derive_key_from_password: Salt codificado correctamente");
    
    info!("🔄 derive_key_from_password: Hasheando contraseña...");
    let password_hash = config.hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    info!("✅ derive_key_from_password: Contraseña hasheada correctamente");
    
    info!("🔄 derive_key_from_password: Extrayendo hash...");
    let hash = password_hash.hash.unwrap();
    let hash_bytes = hash.as_bytes().to_vec();
    info!("✅ derive_key_from_password: Hash extraído, longitud: {} bytes", hash_bytes.len());
    
    Ok(hash_bytes)
}

pub fn hash_password(password: &str, _salt: &[u8]) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt_string = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Error al parsear hash: {}", e))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt.to_vec()
}

pub fn generate_secure_password(length: usize) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
} 