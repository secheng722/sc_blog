use axum::{
    routing::{get, post},
    Router,
};

use self::service::{add_article, get_articles};

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new()
        .route("/getlist", get(get_articles))
        .route("/add", post(add_article))
}
