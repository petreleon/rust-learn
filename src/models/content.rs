use crate::db::schema::contents;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(crate::models::chapter::Chapter))]
#[diesel(table_name = contents)]
pub struct Content {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = contents)]
pub struct NewContent {
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = contents)]
pub struct UpdateContent {
    pub order: Option<i32>,
    pub content_type: Option<String>,
    pub data: Option<String>,
}
