use axum::{
    Router,
    routing::{get, post},
};

use crate::posts::service::{get_ports_all_count, get_ports_pagination};

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new()
        .route("/get_list_pagination", post(get_ports_pagination))
        .route("/get_ports_all_count",get(get_ports_all_count))
}
