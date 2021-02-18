use actix::Addr;
use actix_web::web;
use serde::Deserialize;

use blog_example_database::messages::QueryArticles;
use blog_example_database::DatabaseActor;

use crate::error::ServiceError;

#[derive(Deserialize, Debug)]
pub struct Path {
    pub user_id: i32,
}

impl From<Path> for QueryArticles {
    fn from(path: Path) -> Self {
        Self {
            user_id: path.user_id,
        }
    }
}

pub async fn create_article(
    path: web::Path<Path>,
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
