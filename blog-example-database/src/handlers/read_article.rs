use crate::{messages::QueryArticle, DatabaseActor};
use actix::prelude::*;
use blog_example_models::{schema, Article};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use tracing::debug;

impl Handler<QueryArticle> for DatabaseActor {
    type Result = <QueryArticle as Message>::Result;

    fn handle(&mut self, msg: QueryArticle, _: &mut Self::Context) -> Self::Result {
        let conn = &self.pool.get()?;

        let query = schema::articles::table
            .filter(schema::articles::id.eq(msg.article_id));
        let sql = debug_query::<Pg, _>(&query);
        debug!("{}", sql);

        let article = query.get_result::<Article>(conn)?;

        Ok(article)
    }
}
