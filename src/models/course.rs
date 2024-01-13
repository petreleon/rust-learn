use super::schema::courses;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[table_name = "courses"]
pub struct Course {
    pub id: i32,
    pub title: String,
}