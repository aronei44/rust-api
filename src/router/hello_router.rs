use axum::{Router, routing::get};
use crate::handlers::hello::{
    hello_world,
    hello_world_with_id,
};

// our router
pub fn create_app() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/hello/{id}", get(hello_world_with_id))
}