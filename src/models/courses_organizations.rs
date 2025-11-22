use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::courses_organizations;
use crate::models::course::Course;
use crate::models::organization::Organization;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = courses_organizations)]
#[diesel(primary_key(id))]
pub struct CourseOrganization {
    pub id: i32,
    pub course_id: i32,
    pub organization_id: i32,
    pub order: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = courses_organizations)]
pub struct NewCourseOrganization {
    pub course_id: i32,
    pub organization_id: i32,
    pub order: i32,
}
