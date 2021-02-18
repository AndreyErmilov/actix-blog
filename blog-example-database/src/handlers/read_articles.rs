use crate::{messages::QueryArticles, DatabaseActor};
use actix::prelude::*;
use blog_example_models::{schema, Article};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use tracing::debug;

impl Handler<QueryArticles> for DatabaseActor {
    type Result = <QueryArticles as Message>::Result;

    fn handle(&mut self, msg: QueryArticles, _: &mut Self::Context) -> Self::Result {
        let conn = &self.pool.get()?;

        let query = schema::articles::table
            .filter(schema::articles::user_id.eq(msg.user_id))
            .order_by(schema::articles::publication_time);
        let sql = debug_query::<Pg, _>(&query);
        debug!("{}", sql);

        let articles = query.load::<Article>(conn)?;

        Ok(articles)
    }
}
