use crate::db::schema::roles;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
