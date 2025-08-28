use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::{Rng, RngCore};
use anyhow::{Result, anyhow};

pub struct EncryptionEngine {
    cipher: Aes256Gcm,
}

impl EncryptionEngine {
    pub fn new(key: &[u8]) -> Result<Self> {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher.encrypt(nonce, data)
            .map_err(|e| anyhow!("Error al encriptar: {}", e))?;
        
        Ok((ciphertext, nonce_bytes.to_vec()))
    }
    
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce);
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Error al desencriptar: {}", e))?;
        
        Ok(plaintext)
    }
}

pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
} 