use axum::{
    extract::Path,
    response::Json
};
use serde_json::{Value, json};

pub async fn hello_world() -> Json<Value> {
    let response = json!({
        "message": "Hello, World!"
    });
    Json(response)
}

pub async fn hello_world_with_id(Path(id): Path<u32>) -> Json<Value> {
    let response = json!({
        "message": format!("Hello, World! with id: {}", id)
    });
    Json(response)
}