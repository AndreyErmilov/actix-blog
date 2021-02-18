use chrono::{DateTime, Utc};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ArticleStatus {
    Draft,
    Approved,
    Hidden,
}

/// `articles` table structure.
#[derive(Queryable, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub publication_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
    pub status: ArticleStatus,
}
