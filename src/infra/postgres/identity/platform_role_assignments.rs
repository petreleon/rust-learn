use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{platform_roles, role_platform_hierarchy, user_role_platform};

pub async fn assign_platform_role_with_hierarchy(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    target_user_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    let assigner_level = min_platform_hierarchy_level(conn, assigner_id)
        .await?
        .ok_or(diesel::result::Error::NotFound)?;
    let target_level = min_platform_hierarchy_level(conn, target_user_id).await?;
    let role_id = platform_role_id_by_name(conn, role_name).await?;
    let target_role_level = platform_role_hierarchy_level(conn, role_id).await?;

    if assigner_level >= target_role_level {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    if let Some(target_level) = target_level {
        if assigner_level >= target_level {
            return Err(diesel::result::Error::RollbackTransaction);
        }
    }

    diesel::insert_into(user_role_platform::table)
        .values((
            user_role_platform::user_id.eq(Some(target_user_id)),
            user_role_platform::platform_role_id.eq(Some(role_id)),
        ))
        .execute(conn)
        .await
}

async fn min_platform_hierarchy_level(
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

async fn platform_role_id_by_name(
    conn: &mut AsyncPgConnection,
    role_name: &str,
) -> QueryResult<i32> {
    platform_roles::table
        .filter(platform_roles::name.eq(role_name))
        .select(platform_roles::id)
        .first::<i32>(conn)
        .await
}

async fn platform_role_hierarchy_level(
    conn: &mut AsyncPgConnection,
    role_id: i32,
) -> QueryResult<i32> {
    role_platform_hierarchy::table
        .filter(role_platform_hierarchy::platform_role_id.eq(Some(role_id)))
        .select(role_platform_hierarchy::hierarchy_level)
        .first::<i32>(conn)
        .await
}
