use axum::{
    routing::{get, post},
    Router,
};
use crate::posts::service::get_ports;

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new()
    .route("/getlist", get(get_ports))
}
