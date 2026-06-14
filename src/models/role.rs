use crate::db::schema::{course_roles, organization_roles, platform_roles};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = platform_roles)]
pub struct PlatformRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = organization_roles)]
pub struct OrganizationRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = course_roles)]
pub struct CourseRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
