// Includes
use axum::{
    Router, 
    extract::Path,
    response::Json, 
    routing::{delete, get, post}
};
use serde_json::{json, Value};

// Internal dependencies
use modules::CreatePaste;

// Modules
pub mod modules;
pub mod utils;


#[tokio::main]
async fn main(){
    let app: Router = Router::new()
        .route("/", get(health_check))
        .route("/pastes", get(get_all_pastes))
        .route("/pastes", post(create_paste_endpoint))
        .route("/pastes/{id}", get(get_paste))
        .route("/pastes/{id}", delete(remove_paste_endpoint));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap()
}

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


async fn create_paste_endpoint(Json(data): Json<CreatePaste>) -> Json<Value> {
    match utils::create_paste(data) {
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


async fn remove_paste_endpoint(Path(id): Path<i32>) -> Json<Value> {
    match utils::remove_paste(id) {
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