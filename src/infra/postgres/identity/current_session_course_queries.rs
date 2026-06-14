use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{course_roles, courses, role_permission_course, user_role_course};

#[derive(Debug)]
pub(super) struct CourseRoleRow {
    pub(super) course_id: i32,
    pub(super) course_title: String,
    pub(super) lifecycle_status: String,
    pub(super) role_name: String,
}

#[derive(Debug)]
pub(super) struct CoursePermissionRow {
    pub(super) course_id: i32,
    pub(super) course_title: String,
    pub(super) lifecycle_status: String,
    pub(super) permission: String,
}

pub(super) async fn list_course_roles(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<CourseRoleRow>> {
    let rows = user_role_course::table
        .inner_join(courses::table.on(user_role_course::course_id.eq(courses::id.nullable())))
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::user_id.eq(user_id))
        .select((
            courses::id,
            courses::title,
            courses::lifecycle_status,
            course_roles::name,
        ))
        .load::<(i32, String, String, String)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(course_id, course_title, lifecycle_status, role_name)| CourseRoleRow {
                course_id,
                course_title,
                lifecycle_status,
                role_name,
            },
        )
        .collect())
}

pub(super) async fn list_course_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<CoursePermissionRow>> {
    let rows = user_role_course::table
        .inner_join(courses::table.on(user_role_course::course_id.eq(courses::id.nullable())))
        .inner_join(
            role_permission_course::table
                .on(user_role_course::course_role_id.eq(role_permission_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(user_id))
        .select((
            courses::id,
            courses::title,
            courses::lifecycle_status,
            role_permission_course::permission,
        ))
        .load::<(i32, String, String, String)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(course_id, course_title, lifecycle_status, permission)| CoursePermissionRow {
                course_id,
                course_title,
                lifecycle_status,
                permission,
            },
        )
        .collect())
}
