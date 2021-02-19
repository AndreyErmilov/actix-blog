use crate::DatabaseError;
use actix::prelude::*;
use chrono::{DateTime, Utc};
use blog_example_models::ArticleStatus;

#[derive(Default, Message, Debug)]
#[rtype(result = "Result<blog_example_models::Article, DatabaseError>")]
pub struct QueryArticle {
    pub article_id: i32,
}

#[derive(Default, Message, Debug)]
#[rtype(result = "Result<Vec<blog_example_models::Article>, DatabaseError>")]
pub struct QueryArticles {
    pub user_id: i32,
}

#[derive(Default, Message, Debug)]
#[rtype(result = "Result<(), DatabaseError>")]
pub struct CreateArticle {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
