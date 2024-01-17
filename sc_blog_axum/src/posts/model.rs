use serde::{Deserialize, Serialize};
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::types::chrono;

use crate::utils::date_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    slug: String,
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
}


impl<'r> FromRow<'r, PgRow> for Post {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let slug: String = row.try_get("slug")?;
        let title: String = row.try_get("title")?;
        let date: chrono::NaiveDate = row.try_get("date")?;
        let summary: String = row.try_get("summary")?;
        let tags: Vec<String> = row.try_get("tags")?;
        Ok(Self {
            slug,
            title,
            date: date_to_string(date),
            summary,
            tags,
        })
    }
}


impl Post {
    pub fn new(slug: String, title: String, date: String, summary: String, tags: Vec<String>) -> Self {
        Self {
            slug,
            title,
            date,
            summary,
            tags,
        }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}


