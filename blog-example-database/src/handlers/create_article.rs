use crate::{messages::CreateArticle, DatabaseActor};
use actix::prelude::*;
use blog_example_models::{schema::articles, Article};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use tracing::debug;

impl Handler<CreateArticle> for DatabaseActor {
    type Result = <CreateArticle as Message>::Result;

    fn handle(&mut self, msg: CreateArticle, _: &mut Self::Context) -> Self::Result {
        let conn = &self.pool.get()?;

        Ok(())
    }
}
