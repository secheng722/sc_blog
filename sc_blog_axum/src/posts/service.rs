use std::sync::Arc;

use axum::{Extension, Json};

use crate::{http::api_response::ApiResponse, state::app_state::AppState};

use super::model::Post;

pub async fn get_ports(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ApiResponse<Vec<Post>>> {
    println!("get all posts");
    let conn = state.get_conn().await;
    let posts = sqlx::query_as("select slug,title,date,summary,tags from sc_blog.t_post")
        .fetch_all(conn.as_ref())
        .await
        .unwrap();
    println!("posts: {:?}", posts);
    ApiResponse::ok(posts)
}


