// Includes
use axum::{
    Router, 
    extract::Path,
    response::Json, 
    routing::{delete, get, post}
};
use serde_json::{json, Value};

// Internal dependencies
use modules::{CreatePaste, PasteRequest};

// Modules
pub mod modules;
pub mod utils;

async fn get_all_pastes() -> Json<Value> {
    let pastes = match utils::read_all_pastes() {
        Ok(pastes) => pastes,
        Err(msg) => return Json(json!({
            "status": "ko",
            "message": "An error ocurred",
            "error_msg": msg
        }))
    };

    Json(json!({
        "status": "ok",
        "pastes": pastes
        // "pastes": serde_json::to_string_pretty(pastes)
    }))   
}

async fn get_paste(Path(id): Path<i32>) -> Json<Value> { 
    match utils::read_paste(id) {
        Ok(p) => {
            return Json(json!({
                "status": "ok",
                "paste": p
            }))
        },
        Err(msg) => {
            return Json(json!({
                "status": "ko",
                "error_msg": msg
            }))
        }
    }
}


async fn get_paste_slug(Path(slug): Path<String>, data: Option<Json<PasteRequest>>) -> Json<Value> {
    let password = data.and_then(|Json(body)| body.password);

    match utils::read_paste_slug(slug, password) {
        Ok(p) => {
            return Json(json!({
                "status": "ok",
                "paste": p
            }))
        },
        Err(msg) => {
            return Json(json!({
                "status": "ko",
                "error_msg": msg
            }))
        }
    }
    
}

async fn create_paste_endpoint(Json(data): Json<CreatePaste>) -> Json<Value> {
    match utils::create_paste(data) {
        Ok(d) => { 
            return Json(json!({
                "status": "ok",
                "id": d.0,
                "slug": d.1
            }))
        },
        Err(msg) => {
            return Json(json!({
                "status": "ko",
                "error_msg": msg
            }))
        }
    }
}


async fn remove_paste_endpoint(Path(id): Path<i32>, data: Option<Json<PasteRequest>>) -> Json<Value> {
    let password = data.and_then(|Json(body)| body.password);

    match utils::remove_paste(id, password) {
        Ok(id) => { 
            return Json(json!({
                "status": "ok",
                "id": id
            }))
        },
        Err(msg) => {
            return Json(json!({
                "status": "ko",
                "error_msg": msg
            }))
        }
    }
}


async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running perfectly"
    }))
}


#[tokio::main]
async fn main(){
    match utils::initialize_database() {
        Ok(_) => { println!("Database initializated"); },
        Err(e) => { println!("Error initializating database: {}", e); return }
    } 

    let app: Router = Router::new()
        .route("/", get(health_check))
        .route("/pastes", get(get_all_pastes))
        .route("/pastes", post(create_paste_endpoint))
        .route("/pastes/{id}", get(get_paste))
        .route("/pastes/{id}", delete(remove_paste_endpoint))
        .route("/p/{slug}", get(get_paste_slug));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("Running on http://localhost:8081");
    axum::serve(listener, app).await.unwrap()
}
