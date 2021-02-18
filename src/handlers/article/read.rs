use actix::Addr;
use actix_web::web;
use serde::Deserialize;

use blog_example_database::messages::{QueryArticles, QueryArticle};
use blog_example_database::DatabaseActor;

use crate::error::ServiceError;

/// Structure represents part of URI with `user_id`.
#[derive(Deserialize, Debug)]
pub struct PathArticles {
    pub user_id: i32,
}

impl From<PathArticles> for QueryArticles {
    fn from(path: PathArticles) -> Self {
        Self {
            user_id: path.user_id,
        }
    }
}

/// Handler returns list of articles by `user_id`
pub async fn read_articles(
    path: web::Path<PathArticles>,
    database: web::Data<Addr<DatabaseActor>>,
) -> Result<web::Json<Vec<blog_example_serializers::Article>>, ServiceError> {
    let message = QueryArticles::from(path.into_inner());
    let articles = database
        .send(message)
        .await??
        .into_iter()
        .map(blog_example_serializers::Article::from)
        .collect::<Vec<_>>();
    Ok(web::Json(articles))
}

/// Structure represents part of URI with `article_id`.
#[derive(Deserialize, Debug)]
pub struct PathArticle {
    pub article_id: i32,
}

impl From<PathArticle> for QueryArticle {
    fn from(path: PathArticle) -> Self {
        Self {
            article_id: path.article_id,
        }
    }
}

/// Handler return article by `article_id`
pub async fn read_article(
    path: web::Path<PathArticle>,
    database: web::Data<Addr<DatabaseActor>>,
) -> Result<web::Json<blog_example_serializers::Article>, ServiceError> {
    let message = QueryArticle::from(path.into_inner());
    let article = database
        .send(message)
        .await??;
    Ok(web::Json(blog_example_serializers::Article::from(article)))
}
