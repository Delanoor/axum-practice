use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
mod always_errors;
mod custom_json_extractor;
mod get_json;
mod hello;
mod middleware_message;
mod mirror;
mod mirror_custom_header;
mod mirror_json;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod returns_201;
mod set_middleware_custom_header;
mod validate_data;

use always_errors::always_errors;
use custom_json_extractor::custom_json_extractor;
use get_json::get_json;
use hello::hello;
use middleware_message::middleware_message;
use mirror::mirror;
use mirror_custom_header::mirror_custom_header;
use mirror_json::mirror_json;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use returns_201::returns_201;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use validate_data::validate_data;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_string(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/hello", get(hello))
        .route("/mirror", post(mirror))
        .route("/mirror-json", post(mirror_json))
        .route("/path/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
        .layer(cors) // layers at the bottom
        .layer(Extension(shared_data))
}
