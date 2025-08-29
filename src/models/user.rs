use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: Option<String>,
    pub master_password_hash: String,
    pub salt: Vec<u8>,
    pub created_at: String,
    pub last_login: Option<String>,
} 