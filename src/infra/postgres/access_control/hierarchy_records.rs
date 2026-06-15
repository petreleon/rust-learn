use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::schema::{
    role_course_hierarchy, role_organization_hierarchy, role_platform_hierarchy, user_role_course,
    user_role_organization, user_role_platform,
};

pub async fn platform_min_level_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Option<i32>> {
    role_platform_hierarchy::table
        .inner_join(
            user_role_platform::table
                .on(role_platform_hierarchy::platform_role_id
                    .eq(user_role_platform::platform_role_id)),
        )
        .filter(user_role_platform::user_id.eq(user_id))
        .select(diesel::dsl::min(role_platform_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)
        .await
}

pub async fn platform_role_level(conn: &mut AsyncPgConnection, role_id: i32) -> QueryResult<i32> {
    role_platform_hierarchy::table
        .filter(role_platform_hierarchy::platform_role_id.eq(role_id))
        .select(role_platform_hierarchy::hierarchy_level)
        .first::<i32>(conn)
        .await
}

pub async fn organization_min_level_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
) -> QueryResult<Option<i32>> {
    role_organization_hierarchy::table
        .inner_join(
            user_role_organization::table.on(role_organization_hierarchy::organization_role_id
                .eq(user_role_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(user_id))
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(diesel::dsl::min(
            role_organization_hierarchy::hierarchy_level,
        ))
        .first::<Option<i32>>(conn)
        .await
}

pub async fn organization_role_level(
    conn: &mut AsyncPgConnection,
    role_id: i32,
) -> QueryResult<i32> {
    role_organization_hierarchy::table
        .filter(role_organization_hierarchy::organization_role_id.eq(role_id))
        .select(role_organization_hierarchy::hierarchy_level)
        .first::<i32>(conn)
        .await
}

pub async fn course_min_level_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> QueryResult<Option<i32>> {
    role_course_hierarchy::table
        .inner_join(
            user_role_course::table
                .on(role_course_hierarchy::course_role_id.eq(user_role_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(user_id))
        .filter(user_role_course::course_id.eq(course_id))
        .select(diesel::dsl::min(role_course_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)
        .await
}

pub async fn course_role_level(conn: &mut AsyncPgConnection, role_id: i32) -> QueryResult<i32> {
    role_course_hierarchy::table
        .filter(role_course_hierarchy::course_role_id.eq(role_id))
        .select(role_course_hierarchy::hierarchy_level)
        .first::<i32>(conn)
        .await
}
