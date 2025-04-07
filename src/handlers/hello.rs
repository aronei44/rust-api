use axum::{
    extract::Path,
    response::Json
};

use serde_json::{json, Value};
use utoipa::ToSchema;
use serde::{
    Deserialize,
    Serialize
};

#[utoipa::path(
    get,
    path = "/api/hello",
    responses(
        (status = 200, description = "Say hello")
    )
)]
pub async fn hello_world() -> Json<Value> {
    let response = json!({
        "message": "Hello, World!"
    });
    Json(response)
}

#[utoipa::path(
    get,
    path = "/api/hello/{id}",
    params(
        ("id" = String, Path, description = "ID of the user")
    ),
    responses(
        (status = 200, description = "Say hello with ID")
    )
)]
pub async fn hello_world_with_id(Path(id): Path<u32>) -> Json<Value> {
    let response = json!({
        "message": format!("Hello, World! with id: {}", id)
    });
    Json(response)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct HelloWorldRequest {
    message: String,
}

#[utoipa::path(
    post,
    path = "/api/hello",
    request_body = HelloWorldRequest,
    responses(
        (status = 200, description = "Say hello with payload")
    )
)]

pub async fn create_hello_world(Json(req): Json<HelloWorldRequest>) -> Json<Value> {
    let body = req.message;
    let response = json!({
        "message": format!("Hello, World! with payload: {}", body)
    });
    Json(response)
}
