use std::fs;
use chrono::Local;
use crate::modules::{CreatePaste, Paste};

pub const PATH: &str = "pastes.json";


pub fn read_all_pastes() -> Result<Vec<Paste>, String> {
    let pastes_raw = fs::read_to_string(PATH).unwrap();

    let pastes: Vec<Paste> = match serde_json::from_str::<Vec<Paste>>(&pastes_raw) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string())
    };

    Ok(pastes)
}

pub fn read_paste(id: i32) -> Result<Paste, String> {
    let pastes = read_all_pastes()?;
    let paste = pastes.iter().find(|p| p.id == id).cloned().ok_or_else(|| "No paste found.")?;
    Ok(paste)
}


pub fn create_paste(paste_data: CreatePaste) -> Result<i32, String> {
    let mut pastes = read_all_pastes()?;
    let new_paste = paste_data.clone();

    let id = get_last_id(&pastes) + 1;
    let real_paste = Paste{
        id: id,
        title: new_paste.title,
        content: new_paste.content,
        created_at: Local::now().format("%d-%m-%Y %H:%M").to_string()
    };

    pastes.push(real_paste);
    save_pastes(pastes, id)   
}

pub fn remove_paste(id: i32) -> Result<i32, String> {
    let pastes = read_all_pastes()?;
    let pastes = pastes.iter().filter(|p| p.id != id).cloned().collect();

    save_pastes(pastes, id)
}


fn save_pastes(pastes: Vec<Paste>, id: i32) -> Result<i32, String> {
    let content: String = match serde_json::to_string_pretty(&pastes) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    match fs::write(PATH, content) {
        Ok(_) => return Ok(id),
        Err(e) => return Err(e.to_string())
    }
}


fn get_last_id(pastes: &Vec<Paste>) -> i32 {
    pastes.iter().map(|s| s.id).max().unwrap_or(0)
}