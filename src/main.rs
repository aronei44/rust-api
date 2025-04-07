use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod router;
mod handlers;
use router::hello_router::create_app;

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::hello::hello_world,
    crate::handlers::hello::hello_world_with_id
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Generate Swagger UI
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    // Build our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", Router::new().merge(create_app()))
        .merge(swagger_ui) // Add Swagger UI
        .fallback(fallback_handler);

    // Run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Fallback handler for 404 errors
async fn fallback_handler() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "404: Not Found")
}