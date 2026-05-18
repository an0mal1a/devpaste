use crate::modules::{CreatePaste, Paste};

// pub const PATH: &str = "pastes.json";
pub const PATH: &str = "pastes.sql";

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
            public BOOLEAN,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP 
        )
    ";

    match conn.execute(query, ()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}


pub fn read_all_pastes() -> Result<Vec<Paste>, String> {
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
        Ok(Paste{
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_protected: row.get(3)?,
            public: row.get(4)?,
            created_at: row.get(5)?
        })
    }).map_err(|e| e.to_string())?;

    let mut pastes: Vec<Paste> = Vec::new();
    for paste in pastes_iter {
        let paste = match paste {
            Ok(p) => p,
            Err(e) => { return Err(e.to_string()) }
        };

        pastes.push(paste);
    }


    Ok(pastes)
}


pub fn read_paste(id: i32) -> Result<Paste, String> {
    let conn = create_connection()?;
    let query = "SELECT * FROM pastes WHERE id = ?1 AND is_protected = FALSE";

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let paste = stmt.query_row([id], |row| {
        Ok(Paste{
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_protected: row.get(3)?,
            public: row.get(4)?,
            created_at: row.get(5)?
        })
    }).map_err(|e| e.to_string())?;

    Ok(paste)
}


pub fn create_paste(paste_data: CreatePaste) -> Result<i32, String> {
    let conn = match create_connection() {
        Ok(conn) => conn, 
        Err(e) => return Err(e.to_string())
    };
    let query = "INSERT INTO pastebins (title, content, is_protected, public) VALUES (?1, ?2, ?3, ?4)";

    match conn.execute(query, (paste_data.title, paste_data.content, false, paste_data.public)) {
        Ok(v) => Ok(v as i32),
        Err(e) => Err(e.to_string())
    }
}

pub fn remove_paste(id: i32) -> Result<i32, String> {
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string())
    };

    let query = "DELETE from pastebins WHERE id = ?1";
    match conn.execute(query, [id]) {
        Ok(_) => Ok(id),
        Err(e) => return Err(e.to_string())
    }
}