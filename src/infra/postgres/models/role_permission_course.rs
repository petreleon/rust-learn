use crate::db::schema::role_permission_course;
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::models::role::CourseRole;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(CourseRole))]
#[diesel(belongs_to(Course))]
#[diesel(table_name = role_permission_course)]
pub struct RolePermissionCourse {
    pub id: i32,
    pub course_id: Option<i32>,
    pub course_role_id: Option<i32>,
    pub permission: String,
}
