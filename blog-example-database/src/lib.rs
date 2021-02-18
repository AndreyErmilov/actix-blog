use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub use actor::DatabaseActor;
pub use error::DatabaseError;

mod actor;
mod error;
mod handlers;
pub mod messages;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
