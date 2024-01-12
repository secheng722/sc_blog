use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    id: i32,
    title: String,
    content: String,
    intro: String,
}

impl Article {
    pub fn new(id: i32, title: String, content: String, intro: String) -> Self {
        Self {
            id,
            title,
            content,
            intro,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn intro(&self) -> &str {
        &self.intro
    }
}
