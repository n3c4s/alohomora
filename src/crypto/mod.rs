mod encryption;
mod key_derivation;

pub use encryption::*;
pub use key_derivation::*;

use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
use chacha20poly1305::aead::Aead;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use rand::{Rng, RngCore};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

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
        let key = derive_key_from_password(password, salt)?;
        self.master_key = Some(key);
        Ok(())
    }
    
    pub fn is_unlocked(&self) -> bool {
        self.master_key.is_some()
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
pub fn generate_recovery_key() -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);
    Ok(hex::encode(key))
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
    let config = Argon2::default();
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| format!("Error al codificar salt: {}", e))?;
    
    let password_hash = config.hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| format!("Error al hashear contraseña: {}", e))?;
    
    let hash = password_hash.hash.unwrap();
    Ok(hash.as_bytes().to_vec())
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