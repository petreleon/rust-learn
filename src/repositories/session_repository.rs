use crate::db::schema::{
    courses, organizations, platform_roles, role_permission_platform, user_role_platform, users,
};
use crate::models::user::User;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

mod courses_scope;
mod organizations_scope;

pub use courses_scope::{
    list_course_permissions, list_course_roles, CoursePermissionRow, CourseRoleRow,
};
pub use organizations_scope::{
    list_organization_permissions, list_organization_roles, OrganizationPermissionRow,
    OrganizationRoleRow,
};

pub async fn find_user(conn: &mut AsyncPgConnection, user_id: i32) -> QueryResult<User> {
    users::table.find(user_id).first(conn).await
}

pub async fn list_platform_roles(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<String>> {
    user_role_platform::table
        .inner_join(
            platform_roles::table
                .on(user_role_platform::platform_role_id.eq(platform_roles::id.nullable())),
        )
        .filter(user_role_platform::user_id.eq(current_user_id))
        .select(platform_roles::name)
        .load(conn)
        .await
}

pub async fn list_platform_permissions(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<String>> {
    user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .filter(user_role_platform::user_id.eq(current_user_id))
        .select(role_permission_platform::permission)
        .load(conn)
        .await
}

pub async fn organization_labels(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> QueryResult<Vec<(i32, String)>> {
    if organization_ids.is_empty() {
        return Ok(Vec::new());
    }

    organizations::table
        .filter(organizations::id.eq_any(organization_ids))
        .select((organizations::id, organizations::name))
        .load(conn)
        .await
}

pub async fn course_labels(
    conn: &mut AsyncPgConnection,
    course_ids: &[i32],
) -> QueryResult<Vec<(i32, String, String)>> {
    if course_ids.is_empty() {
        return Ok(Vec::new());
    }

    courses::table
        .filter(courses::id.eq_any(course_ids))
        .select((courses::id, courses::title, courses::lifecycle_status))
        .load(conn)
        .await
}
