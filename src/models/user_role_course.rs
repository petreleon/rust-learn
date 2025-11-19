use diesel::prelude::*;
use crate::db::schema::user_role_course;
use crate::models::user::User;
use crate::models::role::CourseRole;
use crate::models::course::Course;

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

impl UserRoleCourse {
    pub fn has_permission(conn: &mut PgConnection, p_user_id: i32, p_course_id: i32, p_permission: &str) -> QueryResult<bool> {
        use crate::db::schema::{course_roles, role_permission_course, user_role_course};
        
        let has_permission = diesel::select(diesel::dsl::exists(
            user_role_course::table
                .inner_join(course_roles::table.on(
                    user_role_course::course_role_id.eq(course_roles::id.nullable()),
                ))
                .inner_join(role_permission_course::table.on(
                    course_roles::id
                        .nullable()
                        .eq(role_permission_course::course_role_id),
                ))
                .filter(user_role_course::user_id.eq(p_user_id))
                .filter(user_role_course::course_id.eq(p_course_id))
                .filter(role_permission_course::permission.eq(p_permission)),
        ))
        .get_result(conn)?;

        Ok(has_permission)
    }

    pub fn assign(conn: &mut PgConnection, p_user_id: i32, p_course_id: i32, p_course_role_id: i32) -> QueryResult<usize> {
        use crate::db::schema::user_role_course::dsl::*;
        
        let new_user_role = (
            user_id.eq(p_user_id),
            course_role_id.eq(p_course_role_id),
            course_id.eq(p_course_id),
        );

        diesel::insert_into(user_role_course)
            .values(&new_user_role)
            .execute(conn)
    }
}
