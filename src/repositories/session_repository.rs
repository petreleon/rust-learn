use crate::db::schema::{
    course_roles, courses, organization_roles, organizations, platform_roles,
    role_permission_course, role_permission_organization, role_permission_platform,
    user_role_course, user_role_organization, user_role_platform, users,
};
use crate::models::user::User;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug)]
pub struct OrganizationRoleRow {
    pub organization_id: i32,
    pub organization_name: String,
    pub role_name: String,
}

#[derive(Debug)]
pub struct OrganizationPermissionRow {
    pub organization_id: i32,
    pub organization_name: String,
    pub permission: String,
}

#[derive(Debug)]
pub struct CourseRoleRow {
    pub course_id: i32,
    pub course_title: String,
    pub lifecycle_status: String,
    pub role_name: String,
}

#[derive(Debug)]
pub struct CoursePermissionRow {
    pub course_id: i32,
    pub course_title: String,
    pub lifecycle_status: String,
    pub permission: String,
}

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

pub async fn list_organization_roles(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<OrganizationRoleRow>> {
    let rows =
        user_role_organization::table
            .inner_join(
                organizations::table
                    .on(user_role_organization::organization_id.eq(organizations::id.nullable())),
            )
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable()),
            ))
            .filter(user_role_organization::user_id.eq(current_user_id))
            .select((
                organizations::id,
                organizations::name,
                organization_roles::name,
            ))
            .load::<(i32, String, String)>(conn)
            .await?;

    Ok(rows
        .into_iter()
        .map(
            |(organization_id, organization_name, role_name)| OrganizationRoleRow {
                organization_id,
                organization_name,
                role_name,
            },
        )
        .collect())
}

pub async fn list_organization_permissions(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<OrganizationPermissionRow>> {
    let rows = user_role_organization::table
        .inner_join(
            organizations::table
                .on(user_role_organization::organization_id.eq(organizations::id.nullable())),
        )
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(current_user_id))
        .select((
            organizations::id,
            organizations::name,
            role_permission_organization::permission,
        ))
        .load::<(i32, String, String)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(organization_id, organization_name, permission)| OrganizationPermissionRow {
                organization_id,
                organization_name,
                permission,
            },
        )
        .collect())
}

pub async fn list_course_roles(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<CourseRoleRow>> {
    let rows = user_role_course::table
        .inner_join(courses::table.on(user_role_course::course_id.eq(courses::id.nullable())))
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::user_id.eq(current_user_id))
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

pub async fn list_course_permissions(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> QueryResult<Vec<CoursePermissionRow>> {
    let rows = user_role_course::table
        .inner_join(courses::table.on(user_role_course::course_id.eq(courses::id.nullable())))
        .inner_join(
            role_permission_course::table
                .on(user_role_course::course_role_id.eq(role_permission_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(current_user_id))
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
