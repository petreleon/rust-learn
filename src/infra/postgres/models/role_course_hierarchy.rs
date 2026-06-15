use crate::infra::postgres::models::role::CourseRole;
use crate::infra::postgres::schema::role_course_hierarchy;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(CourseRole))]
#[diesel(table_name = role_course_hierarchy)]
pub struct RoleCourseHierarchy {
    pub id: i32,
    pub course_role_id: Option<i32>,
    pub hierarchy_level: i32,
}
