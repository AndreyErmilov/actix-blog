use crate::schema::users;
use diesel::Queryable;
use serde::Deserialize;

/// `users` table structure.
#[derive(Queryable, Debug, Deserialize, Identifiable, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_pic: String,
}
