use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use anyhow::{Result, anyhow};

pub struct KeyDerivation {
    argon2: Argon2<'static>,
}

impl Default for KeyDerivation {
    fn default() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl KeyDerivation {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let mut key = [0u8; 32];
        
        self.argon2.hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow!("Error en derivación de clave: {}", e))?;
        
        Ok(key.to_vec())
    }
    
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        
        let hash = self.argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Error al hashear contraseña: {}", e))?;
        
        Ok(hash.to_string())
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow!("Hash inválido: {}", e))?;
        
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    pub fn generate_salt() -> Vec<u8> {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt.to_vec()
    }
}

pub fn create_master_key(password: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let salt = KeyDerivation::generate_salt();
    let key_derivation = KeyDerivation::new();
    let key = key_derivation.derive_key(password, &salt)?;
    
    Ok((key, salt))
}

pub fn verify_master_key(password: &str, salt: &[u8], expected_key: &[u8]) -> Result<bool> {
    let key_derivation = KeyDerivation::new();
    let derived_key = key_derivation.derive_key(password, salt)?;
    
    Ok(derived_key == expected_key)
} 