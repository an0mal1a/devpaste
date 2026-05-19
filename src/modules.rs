use serde::{Deserialize, Serialize}; 

#[derive(Clone, Deserialize, Serialize)]
pub struct CreatePaste {
    pub title: String,
    pub content: String,
    
    #[serde(default = "default_true")]
    pub public: bool,

    #[serde(default)]
    pub password: String
    
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paste {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_protected: bool,
    pub password: String,
    pub public: bool,
    pub created_at: String
}

#[derive(Clone, Serialize)]
pub struct PasteResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_protected: bool, 
    pub public: bool,
    pub created_at: String
}

#[derive(Deserialize)]
pub struct PasteRequest {
    pub password: Option<String>
}

fn default_true() -> bool {
    true
}

impl From<Paste> for PasteResponse {
    fn from(p: Paste) -> Self {
        Self {
            id: p.id,
            title: p.title,
            content: p.content,
            is_protected: p.is_protected,
            public: p.public,
            created_at: p.created_at
        }
    }
}