use std::time::Duration;

use actix::prelude::*;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use tracing::info;

use crate::{DatabaseError, Pool};

pub struct DatabaseActor {
    pub pool: Pool,
}

impl Actor for DatabaseActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("Database actor started");
    }
}

impl DatabaseActor {
    pub fn start(connection_info: String) -> Result<Addr<DatabaseActor>, DatabaseError> {
        let manager = ConnectionManager::<PgConnection>::new(connection_info);
        let pool = r2d2::Pool::builder()
            .connection_timeout(Duration::from_secs(3))
            .max_size(10)
            .build(manager)?;
        let addr = SyncArbiter::start(10, move || DatabaseActor { pool: pool.clone() });
        Ok(addr)
    }
}
