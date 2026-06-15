use crate::infra::postgres::schema::courses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, PartialEq, Debug, Selectable, Serialize)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub title: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = courses)]
pub struct UpdateCourse {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub topics: Option<Option<String>>,
    pub prerequisites: Option<Option<String>>,
}
