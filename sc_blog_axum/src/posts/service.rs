use std::sync::Arc;

use axum::{Extension, Json};

use crate::{http::api_response::ApiResponse, state::app_state::AppState};
use crate::posts::model::PaginationRequest;

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

pub async fn get_ports_pagination(
    Extension(state): Extension<Arc<AppState>>,
    pagination_request: Option<Json<PaginationRequest>>,
) -> Json<ApiResponse<Vec<Post>>> {
    println!("get all posts pagination");
    let conn = state.get_conn().await;
    return if let Some(Json(paginationRequest)) = pagination_request {
        println!("pagination request: {:?}", paginationRequest);
        let posts = sqlx::query_as::<_, Post>(
            "select slug,title,date,summary,tags from sc_blog.t_post where ($1 = '' or $1 is null or $1 = any(tags)) limit $2 offset $3",
        )
            .bind(paginationRequest.tag())
            .bind(paginationRequest.page_size())
            .bind((paginationRequest.page() - 1) * paginationRequest.page_size())
            .fetch_all(conn.as_ref())
            .await
            .unwrap();
        println!("posts: {:?}", posts);
        ApiResponse::ok(posts)
    } else {
        println!("get all posts pagination failed");
        ApiResponse::err("get all posts pagination failed")
    };
}


pub async fn get_ports_all_count(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ApiResponse<i64>> {
    println!("get all posts count");
    let conn = state.get_conn().await;
    let count = sqlx::query_scalar("select count(*) from sc_blog.t_post")
        .fetch_one(conn.as_ref())
        .await
        .unwrap();
    println!("posts count: {:?}", count);
    ApiResponse::ok(count)
}
