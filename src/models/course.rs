use crate::db::schema::courses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, PartialEq, Debug, Selectable, Serialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = courses)]
pub struct UpdateCourse {
    pub title: Option<String>,
}