use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use rand::RngCore;
use anyhow::{Result, anyhow};

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    
    argon2.hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Error en derivación de clave: {}", e))?;
    
    Ok(key.to_vec())
}

pub fn hash_password(password: &str, _salt: &[u8]) -> Result<String> {
    let argon2 = Argon2::default();
    let salt_string = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| anyhow!("Error al hashear contraseña: {}", e))?;
    
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow!("Error al parsear hash: {}", e))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt.to_vec()
}

pub fn create_master_key(password: &str) -> Result<(String, Vec<u8>)> {
    let salt = generate_salt();
    let hash = hash_password(password, &salt)?;
    Ok((hash, salt))
}

pub fn verify_master_key(password: &str, hash: &str, salt: &[u8]) -> Result<bool> {
    let computed_hash = hash_password(password, salt)?;
    Ok(computed_hash == hash)
} 