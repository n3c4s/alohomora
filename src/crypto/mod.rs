mod encryption;
mod key_derivation;

pub use encryption::*;
pub use key_derivation::*;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, PasswordHashString};
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
    
    pub fn set_master_key(&mut self, password: &str, salt: &[u8]) -> Result<()> {
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
        let cipher = Aes256Gcm::new(key);
        
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
        let cipher = Aes256Gcm::new(key);
        
        let nonce = Nonce::from_slice(&encrypted_data.nonce);
        
        let plaintext = cipher.decrypt(nonce, encrypted_data.ciphertext.as_slice())
            .map_err(|e| anyhow!("Error al desencriptar: {}", e))?;
        
        Ok(plaintext)
    }
    
    pub fn lock(&mut self) {
        self.master_key = None;
    }
}

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    
    argon2.hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Error en derivación de clave: {}", e))?;
    
    Ok(key.to_vec())
}

pub fn hash_password(password: &str, salt: &[u8]) -> Result<String> {
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt)
        .map_err(|e| anyhow!("Error al hashear contraseña: {}", e))?;
    
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow!("Hash inválido: {}", e))?;
    
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
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    password
} 

pub fn generate_secure_password_custom(
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    exclude_similar: bool,
) -> String {
    let mut charset = Vec::new();
    
    if include_uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    
    if include_lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    
    if include_numbers {
        charset.extend_from_slice(b"0123456789");
    }
    
    if include_symbols {
        if exclude_similar {
            charset.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");
        } else {
            charset.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?~`");
        }
    }
    
    // Si no se especificó ningún tipo, usar todos
    if charset.is_empty() {
        charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?".to_vec();
    }
    
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    
    password
} 