use std::sync::Arc;

use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use sc_blog_axum::{article, content, posts, state::app_state::AppState, user};

#[tokio::main]
async fn main() {
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://@localhost:5432/lishaocheng")
        .await
        .expect("Failed to connect to Postgres.");
    let web_addr = "localhost:9527";
    let app = Router::new()
        .nest("/user", user::init())
        .nest("/blog", article::init())
        .nest("/post", posts::init())
        .nest("/content", content::init())
        .layer(Extension(Arc::new(AppState {
            pool: Arc::new(pgpool),
        })));
    let listener = TcpListener::bind(web_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
