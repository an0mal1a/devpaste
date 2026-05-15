use serde::{Deserialize, Serialize}; 

#[derive(Clone, Deserialize, Serialize)]
pub struct CreatePaste {
    pub title: String,
    pub content: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Paste {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String
}