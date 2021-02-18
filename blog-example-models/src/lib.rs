#[macro_use]
extern crate diesel;

pub mod article;
pub mod user;
pub use article::{Article, ArticleStatus};

#[allow(unused_imports)]
pub mod schema;

// We create schema.rs automatically.
//
// Diesel creates `Article_status` field and we should rename diesel_derive_enum
// name `ArticleStatusMapping` to `Article_status`.
pub mod exports {
    pub use crate::article::ArticleStatusMapping as Article_status;
}
