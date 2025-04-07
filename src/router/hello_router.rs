use axum::{
    Router, 
    routing::{
        get,
        post
    }
};
use crate::handlers::hello::{
    hello_world,
    hello_world_with_id,
    create_hello_world
};

// our router
pub fn create_app() -> Router {
    Router::new()
        .route("/hello", post(create_hello_world))
        .route("/hello", get(hello_world))
        .route("/hello/{id}", get(hello_world_with_id))
}