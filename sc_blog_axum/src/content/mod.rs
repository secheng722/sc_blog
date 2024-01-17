use axum::Router;
use axum::routing::get;

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new()
        .route("/getlist", get(service::get_post_body))
}
