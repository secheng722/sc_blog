use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Postgres>>,
}

impl AppState {
    pub async fn get_conn(&self) -> Arc<Pool<Postgres>> {
        self.pool.clone()
    }
}
