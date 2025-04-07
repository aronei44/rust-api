use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::hello::hello_world,
    crate::handlers::hello::hello_world_with_id,
    crate::handlers::hello::create_hello_world
))]
pub struct ApiDoc;