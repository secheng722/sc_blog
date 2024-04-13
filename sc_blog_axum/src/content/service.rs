use std::sync::Arc;

use axum::extract::Query;
use axum::{Extension, Json};

use crate::content::model::{PostContent, PostContentQuery};
use crate::http::api_response::ApiResponse;
use crate::state::app_state::AppState;



pub async fn get_post_body(
    Extension(state): Extension<Arc<AppState>>,
    slug: Option<Query<PostContentQuery>>,
) -> Json<ApiResponse<PostContent>> {
    println!("get post body");
    return if let Some(Query(slug)) = slug {
        let conn = state.get_conn().await;
        let post = sqlx::query_as::<_, PostContent>(
            "select slug,body,date from sc_blog.t_post_content where slug=$1",
        )
        .bind(slug.slug())
        .fetch_one(conn.as_ref())
        .await
        .unwrap();
        println!("post slug: {}", post.slug());
        ApiResponse::ok(post)
    } else {
        ApiResponse::err("get post body failed")
    };
}

