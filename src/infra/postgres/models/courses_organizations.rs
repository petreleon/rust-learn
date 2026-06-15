use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::models::organization::Organization;
use crate::infra::postgres::schema::courses_organizations;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
