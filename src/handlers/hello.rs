use axum::extract::Path;

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn hello_world_with_id(Path(id): Path<u32>) -> String {
    format!("Hello, World! with id: {}", id)
}