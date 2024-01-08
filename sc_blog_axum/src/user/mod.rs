use axum::{routing::post, Router}; // Add the missing import for the Handler trait

use self::service::add_user;

pub mod model;
pub mod service;

pub fn init() -> Router {
    Router::new().route("/register", post(add_user))
}
