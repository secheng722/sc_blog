use std::sync::Arc;

use axum::{Extension, Json};
use sc_blog_axum::{http::api_response::ApiResponse, state::app_state::AppState};

use super::model::Article;

pub async fn get_articles(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ApiResponse<Vec<Article>>> {
    println!("get articles");
    let conn = state.get_conn().await;
    let articles = sqlx::query_as("select * from sc_blog.t_article")
        .fetch_all(conn.as_ref())
        .await
        .unwrap();
    ApiResponse::ok(articles)
}

pub async fn add_article(
    Extension(state): Extension<Arc<AppState>>,
    Json(article): Json<Article>,
) -> Json<ApiResponse<()>> {
    println!("article: {:?}", article);
    let conn = state.get_conn().await;
    sqlx::query("INSERT INTO sc_blog.t_article (title,content) VALUES ($1,$2)")
        .bind(article.title())
        .bind(article.content())
        .execute(conn.as_ref())
        .await
        .unwrap();
    ApiResponse::ok_msg("add article success")
}
