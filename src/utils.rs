
use crate::modules::{CreatePaste, PasteResponse, Paste};

// pub const PATH: &str = "pastes.json";
pub const PATH: &str = "pastes.sql";

// Hashing function
fn hash_password(password: Option<String>) -> String {
    match password {
        Some(pwd) => blake3::hash(pwd.as_bytes()).to_hex().to_string(),
        None => String::new(),
    }
}

// SQLITE Functions
pub fn create_connection() -> Result<rusqlite::Connection, String> {
    rusqlite::Connection::open(PATH)
        .map_err(|e| e.to_string())
}

pub fn initialize_database() -> Result<(), String> {
    let conn = rusqlite::Connection::open(PATH).unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS pastebins (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            is_protected BOOLEAN,
            password TEXT,
            public BOOLEAN,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP 
        )
    ";

    match conn.execute(query, ()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}


pub fn read_all_pastes() -> Result<Vec<PasteResponse>, String> {
    let conn: rusqlite::Connection = match create_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()) 
    };

    let query = "SELECT * FROM pastebins WHERE public = TRUE AND is_protected = FALSE;";
    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => { return Err(e.to_string()) }
    };

    let pastes_iter = stmt.query_map([], |row| {
        Ok(PasteResponse{
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_protected: row.get(3)?,
            public: row.get(5)?,
            created_at: row.get(6)?
        })
    }).map_err(|e| e.to_string())?;

    let mut pastes: Vec<PasteResponse> = Vec::new();
    for paste in pastes_iter {
        let paste = match paste {
            Ok(p) => p,
            Err(e) => { return Err(e.to_string()) }
        };

        pastes.push(paste)
    }

    Ok(pastes)
}


pub fn read_paste(id: i32, password: Option<String>) -> Result<PasteResponse, String> {
    let conn = create_connection()?;
    let query = "SELECT * FROM pastebins WHERE id = ?1";

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let paste = stmt.query_row([id], |row| {
        Ok(Paste{
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_protected: row.get(3)?,
            password: row.get(4)?,
            public: row.get(5)?,
            created_at: row.get(6)?
        })
    }).map_err(|e| e.to_string())?;

    if paste.is_protected && hash_password(password) != paste.password.as_str() { 
        return Err("Paste is password protected".to_string())
    }

    Ok(paste.into())
}


pub fn create_paste(paste_data: CreatePaste) -> Result<i32, String> {
    let conn = match create_connection() {
        Ok(conn) => conn, 
        Err(e) => return Err(e.to_string())
    };

    // Password protected
    let mut is_protected: bool = false;
    let mut is_public: bool = paste_data.public;
    if !paste_data.password.is_empty() { 
        is_protected = true; 
        is_public = false;
    }

    // Construct query
    let query = "INSERT INTO pastebins (title, content, is_protected, password, public) VALUES (?1, ?2, ?3, ?4, ?5)";

    match conn.execute(query, (paste_data.title, paste_data.content, is_protected, hash_password(Some(paste_data.password)), is_public)) {
        Ok(v) => Ok(v as i32),
        Err(e) => Err(e.to_string())
    }
}

pub fn remove_paste(id: i32, password: Option<String>) -> Result<i32, String> {
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string())
    };

    match read_paste(id, password) {
        Ok(p) => p,
        Err(e) => return Err(e)
    };

    let query = "DELETE from pastebins WHERE id = ?1";
    match conn.execute(query, [id]) {
        Ok(_) => Ok(id),
        Err(e) => return Err(e.to_string())
    }
}