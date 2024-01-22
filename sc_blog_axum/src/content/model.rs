use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;

use crate::utils::date_to_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostContent {
    slug: String,
    body: String,
    date: String,
}

impl<'r> FromRow<'r, PgRow> for PostContent {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let slug: String = row.try_get("slug")?;
        let body: String = row.try_get("body")?;
        let date: NaiveDate = row.try_get("date")?;
        Ok(Self {
            slug,
            body,
            date: date_to_string(date),
        })
    }
}

impl PostContent {
    pub fn new(slug: String, body: String, date: String) -> Self {
        Self {
            slug,
            body,
            date,
        }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn date(&self) -> &str {
        &self.date
    }
}

#[derive(Debug, Deserialize)]
pub struct PostContentQuery {
    slug: String,
}

impl PostContentQuery {
    pub fn new(slug: String) -> Self {
        Self { slug }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Author {
    name: String,
    avatar: String,
    slug: String,
}

impl Author {
    pub fn new(name: String, avatar: String, slug: String) -> Self {
        Self { name, avatar, slug }
    }
}
