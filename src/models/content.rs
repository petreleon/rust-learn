use crate::db::schema::contents;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = contents)]
pub struct Content {
    pub id: i32,
    pub chapter_id: Option<i32>,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}
