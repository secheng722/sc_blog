use axum::{
    routing::{get, post},
    Router,
}; // Add the missing import for the Handler trait

use self::service::{add_user, check_user, get_user};

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new()
        .route("/getlist", get(get_user))
        .route("/register", post(add_user))
        .route("/login", post(check_user))
}
