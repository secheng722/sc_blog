use std::sync::Arc;

use axum::{Extension, Json};

use crate::state::app_state::AppState;

use super::model::User;

pub async fn add_user(Extension(state): Extension<Arc<AppState>>, Json(user): Json<User>) {
    println!("user: {:?}", user);
    let conn = state.get_conn().await;
    sqlx::query("INSERT INTO plant_camera.t_user (username,password) VALUES ($1,$2)")
        .bind(user.username())
        .bind(user.password())
        .execute(conn.as_ref())
        .await
        .unwrap();
}
