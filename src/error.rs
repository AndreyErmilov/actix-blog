use blog_example_database::DatabaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}
