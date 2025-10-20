use diesel::prelude::*;
use crate::db::schema::{platform_roles, organization_roles, course_roles};

#[derive(Queryable, Insertable)]
#[diesel(table_name = platform_roles)]
pub struct PlatformRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = organization_roles)]
pub struct OrganizationRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = course_roles)]
pub struct CourseRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}