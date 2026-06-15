use crate::db::schema::user_role_course;
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::models::role::CourseRole;
use crate::infra::postgres::models::user::User;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CourseRole))]
#[diesel(belongs_to(Course))]
#[diesel(table_name = user_role_course)]
pub struct UserRoleCourse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub course_role_id: Option<i32>,
    pub course_id: Option<i32>,
}
