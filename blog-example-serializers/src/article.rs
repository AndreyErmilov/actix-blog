use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub enum ArticleStatus {
    Draft,
    Approved,
    Hidden,
}

#[derive(Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publication_time: DateTime<Utc>,
    pub modification_time: DateTime<Utc>,
    pub status: ArticleStatus,
}

impl From<blog_example_models::ArticleStatus> for ArticleStatus {
    fn from(model: blog_example_models::ArticleStatus) -> Self {
        match model {
            blog_example_models::ArticleStatus::Approved => ArticleStatus::Approved,
            blog_example_models::ArticleStatus::Draft => ArticleStatus::Draft,
            blog_example_models::ArticleStatus::Hidden => ArticleStatus::Hidden,
        }
    }
}

impl From<blog_example_models::Article> for Article {
    fn from(model: blog_example_models::Article) -> Self {
        Self {
            id: model.id,
            title: model.title,
            body: model.body,
            publication_time: model.publication_time,
            modification_time: model.modification_time,
            status: ArticleStatus::from(model.status),
        }
    }
}
