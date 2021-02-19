use crate::{messages::CreateArticle, DatabaseActor};
use actix::prelude::*;
use blog_example_models::{schema, Article};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use tracing::debug;
use blog_example_models::article::{ArticleInsertable, ArticleStatus};

impl From<CreateArticle> for ArticleInsertable {
    fn from(message: CreateArticle) -> Self {
        Self {
            user_id: message.user_id,
            title: message.title,
            body: message.body,
            publication_time: chrono::Utc::now(),
            modification_time: chrono::Utc::now(),
            status: ArticleStatus::Draft,
        }
    }
}

impl Handler<CreateArticle> for DatabaseActor {
    type Result = <CreateArticle as Message>::Result;

    fn handle(&mut self, msg: CreateArticle, _: &mut Self::Context) -> Self::Result {
        let conn = &self.pool.get()?;
        let insert = ArticleInsertable::from(msg);
        diesel::insert_into(schema::articles::table)
            .values(&insert)
            .execute(conn)?;
        Ok(())
    }
}
