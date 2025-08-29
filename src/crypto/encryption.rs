use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
use chacha20poly1305::aead::Aead;
use rand::RngCore;
use anyhow::{Result, anyhow};

pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher.encrypt(nonce, data)
        .map_err(|e| anyhow!("Error al encriptar: {}", e))?;
    
    Ok((ciphertext, nonce_bytes.to_vec()))
}

pub fn decrypt_data(data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    
    let nonce = Nonce::from_slice(nonce);
    
    let plaintext = cipher.decrypt(nonce, data)
        .map_err(|e| anyhow!("Error al desencriptar: {}", e))?;
    
    Ok(plaintext)
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