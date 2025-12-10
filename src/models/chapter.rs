use crate::db::schema::chapters;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(crate::models::course::Course))]
#[diesel(table_name = chapters)]
pub struct Chapter {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = chapters)]
pub struct UpdateChapter {
    pub title: Option<String>,
    pub order: Option<i32>,
}
