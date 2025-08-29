use rusqlite::{Connection, Result, params};
use crate::models::{PasswordEntry, Category, User};

pub struct PasswordRepository<'a> {
    connection: &'a Connection,
}

impl<'a> PasswordRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }
    
    pub fn create_password(&self, entry: &PasswordEntry) -> Result<()> {
        let tags_json = serde_json::to_string(&entry.tags)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        
        self.connection.execute(
            "INSERT INTO password_entries (id, title, username, password, url, notes, category_id, tags, created_at, updated_at, last_used)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                entry.id,
                entry.title,
                entry.username,
                entry.password,
                entry.url,
                entry.notes,
                entry.category_id,
                tags_json,
                entry.created_at,
                entry.updated_at,
                entry.last_used
            ],
        )?;
        
        Ok(())
    }
    
    pub fn get_all_passwords(&self) -> Result<Vec<PasswordEntry>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, username, password, url, notes, category_id, tags, created_at, updated_at, last_used
             FROM password_entries ORDER BY updated_at DESC"
        )?;
        
        let entries = stmt.query_map([], |row| {
            let tags_json: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json)
                .unwrap_or_default();
            
            Ok(PasswordEntry {
                id: row.get(0)?,
                title: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
                notes: row.get(5)?,
                category_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                last_used: row.get(10)?,
            })
        })?;
        
        entries.collect()
    }
    
    pub fn get_password_by_id(&self, id: &str) -> Result<Option<PasswordEntry>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, username, password, url, notes, category_id, tags, created_at, updated_at, last_used
             FROM password_entries WHERE id = ?"
        )?;
        
        let mut entries = stmt.query_map(params![id], |row| {
            let tags_json: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json)
                .unwrap_or_default();
            
            Ok(PasswordEntry {
                id: row.get(0)?,
                title: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
                notes: row.get(5)?,
                category_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                last_used: row.get(10)?,
            })
        })?;
        
        Ok(entries.next().transpose()?)
    }
    
    pub fn update_password(&self, entry: &PasswordEntry) -> Result<()> {
        let tags_json = serde_json::to_string(&entry.tags)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        
        self.connection.execute(
            "UPDATE password_entries 
             SET title = ?, username = ?, password = ?, url = ?, notes = ?, category_id = ?, tags = ?, updated_at = ?
             WHERE id = ?",
            params![
                entry.title,
                entry.username,
                entry.password,
                entry.url,
                entry.notes,
                entry.category_id,
                tags_json,
                entry.updated_at,
                entry.id
            ],
        )?;
        
        Ok(())
    }
    
    pub fn delete_password(&self, id: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM password_entries WHERE id = ?",
            params![id],
        )?;
        
        Ok(())
    }
    
    pub fn search_passwords(&self, query: &str) -> Result<Vec<PasswordEntry>> {
        let search_query = format!("%{}%", query);
        let mut stmt = self.connection.prepare(
            "SELECT id, title, username, password, url, notes, category_id, tags, created_at, updated_at, last_used
             FROM password_entries 
             WHERE title LIKE ? OR username LIKE ? OR url LIKE ? OR notes LIKE ?
             ORDER BY updated_at DESC"
        )?;
        
        let entries = stmt.query_map(params![search_query, search_query, search_query, search_query], |row| {
            let tags_json: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json)
                .unwrap_or_default();
            
            Ok(PasswordEntry {
                id: row.get(0)?,
                title: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
                notes: row.get(5)?,
                category_id: row.get(6)?,
                tags,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                last_used: row.get(10)?,
            })
        })?;
        
        entries.collect()
    }
} 