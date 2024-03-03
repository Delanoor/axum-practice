use axum::{
    routing::{get, post},
    Router,
};
mod hello;
mod mirror;
mod mirror_custom_header;
mod mirror_json;
mod mirror_user_agent;
mod path_variables;
mod query_params;

use hello::hello;
use mirror::mirror;
use mirror_custom_header::mirror_custom_header;
use mirror_json::mirror_json;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/mirror", post(mirror))
        .route("/mirror-json", post(mirror_json))
        .route("/path/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
}
