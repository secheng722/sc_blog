use std::sync::Arc;

use axum::{Extension, Json};

use crate::{http::api_response::ApiResponse, state::app_state::AppState};

use super::model::User;

pub async fn get_user(Extension(state): Extension<Arc<AppState>>) -> Json<ApiResponse<Vec<User>>> {
    let conn = state.get_conn().await;
    let users = sqlx::query_as("select username,password from plant_camera.t_user")
        .fetch_all(conn.as_ref())
        .await
        .unwrap();
    ApiResponse::ok(users)
}

pub async fn add_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(user): Json<User>,
) -> Json<ApiResponse<()>> {
    println!("user: {:?}", user);
    let conn = state.get_conn().await;
    sqlx::query("INSERT INTO plant_camera.t_user (username,password) VALUES ($1,$2)")
        .bind(user.username())
        .bind(user.password())
        .execute(conn.as_ref())
        .await
        .unwrap();
    ApiResponse::ok_msg("add user success")
}

pub async fn check_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(user): Json<User>,
) -> Json<ApiResponse<()>> {
    println!("user: {:?}", user);
    let conn = state.get_conn().await;
    match sqlx::query_as::<_, User>(
        "select username,password from plant_camera.t_user where username=$1 and password=$2",
    )
    .bind(user.username())
    .bind(user.password())
    .fetch_all(conn.as_ref())
    .await
    {
        Ok(res) => {
            if res.len() == 1 {
                println!("login success");
                return ApiResponse::ok_msg("login success");
            } else {
                println!("login failed");
                return ApiResponse::err("login failed");
            }
        }
        Err(_) => {
            println!("login failed");
            return ApiResponse::err("login failed");
        }
    };
}
