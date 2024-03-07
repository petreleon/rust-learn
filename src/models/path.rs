use crate::db::schema::paths;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = paths)]
pub struct Path {
    pub id: i32,
    pub name: String,
}
