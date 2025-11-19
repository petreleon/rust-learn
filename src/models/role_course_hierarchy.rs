use diesel::prelude::*;
use crate::db::schema::role_course_hierarchy;
use crate::models::role::CourseRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(CourseRole))]
#[diesel(table_name = role_course_hierarchy)]
pub struct RoleCourseHierarchy {
    pub id: i32,
    pub course_role_id: Option<i32>,
    pub hierarchy_level: i32,
}

impl RoleCourseHierarchy {
    pub fn get_min_level(conn: &mut PgConnection, p_user_id: i32, p_course_id: i32) -> QueryResult<Option<i32>> {
        use crate::db::schema::{role_course_hierarchy, user_role_course};
        use diesel::dsl::min;

        role_course_hierarchy::table
            .inner_join(user_role_course::table.on(
                role_course_hierarchy::course_role_id.eq(user_role_course::course_role_id),
            ))
            .filter(user_role_course::user_id.eq(p_user_id))
            .filter(user_role_course::course_id.eq(p_course_id))
            .select(min(role_course_hierarchy::hierarchy_level))
            .first::<Option<i32>>(conn)
    }
}
