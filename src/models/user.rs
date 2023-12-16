use diesel::prelude::*;
use crate::db::schema::users;

#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    // Other fields...
}
