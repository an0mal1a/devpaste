use serde::{Deserialize, Serialize}; 

#[derive(Clone, Deserialize, Serialize)]
pub struct CreatePaste {
    pub title: String,
    pub content: String,
    
    #[serde(default = "default_true")]
    pub public: bool
    
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Paste {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_protected: bool,
    pub public: bool,
    pub created_at: String
}

fn default_true() -> bool {
    true
}