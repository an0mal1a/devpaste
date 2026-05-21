
use constant_time_eq::constant_time_eq;
use crate::modules::{CreatePaste, PasteResponse, Paste};

// pub const PATH: &str = "pastes.json";
pub const PATH: &str = "pastes.sql";

// Hashing function
fn hash_password(password: Option<String>) -> String {
    match password {
        Some(pwd) if !pwd.is_empty() => blake3::hash(pwd.as_bytes()).to_hex().to_string(),
        None => String::new(),
        _ => String::new(),
    }
}

fn verify_password(password: Option<String>, stored_hash: &str) -> bool {
    let Some(password) = password else {
        return false
    };

    let input_hash = blake3::hash(password.as_bytes()).to_hex().to_string();
    constant_time_eq(input_hash.as_bytes(), stored_hash.as_bytes())
}

fn generate_paste_slug(title: &String, content: &String) -> String {
    let mut hasher = blake3::Hasher::new();

    hasher.update(title.as_bytes());
    hasher.update(b":");
    hasher.update(content.as_bytes());

    hasher.finalize().to_hex().to_string()
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
            slug TEXT UNIQUE,
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
            public: row.get(6)?,
            created_at: row.get(7)?
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
            slug: row.get(5)?,
            public: row.get(6)?,
            created_at: row.get(7)?
        })
    }).map_err(|e| e.to_string())?;

    if paste.is_protected && !verify_password(password, paste.password.as_str()) { 
        return Err("Paste is password protected".to_string())
    
    } else if !paste.public {
        return Err("Paste not found".to_string());
    }

    Ok(paste.into())
}

pub fn read_paste_slug(slug: String, password: Option<String>) -> Result<PasteResponse, String> {
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(e) => { return Err(e.to_string()); }
    };

    let query = "SELECT * FROM pastebins WHERE slug = ?1";
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let paste = stmt.query_row([slug], | row | {
        Ok({
            Paste{
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                is_protected: row.get(3)?,
                password: row.get(4)?,
                slug: row.get(5)?,
                public: row.get(6)?,
                created_at: row.get(7)?
            }
        })
    }).map_err(|e| e.to_string())?;

    if paste.is_protected && !verify_password(password, paste.password.as_str()) { 
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
    let mut slug: Option<String>= None;
    
    // Check if its not puyblic
    if !is_public {
        slug = if !is_public {
            Some(generate_paste_slug(&paste_data.title, &paste_data.content))
        } else { None };
    }

    // Check if has password
    if !paste_data.password.is_empty() { 
        is_protected = true; 
        is_public = false;
    }

    // Construct query
    let query = "INSERT INTO pastebins (title, content, is_protected, password, slug, public) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

    match conn.execute(query, (paste_data.title, paste_data.content, is_protected, hash_password(Some(paste_data.password)), slug, is_public)) {
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