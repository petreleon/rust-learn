use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::schema::{
    organization_roles, role_organization_hierarchy, user_role_organization,
};

pub(super) async fn assign_role_with_hierarchy(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    target_user_id: i32,
    organization_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    let assigner_level = organization_min_level(conn, assigner_id, organization_id)
        .await?
        .ok_or(diesel::result::Error::NotFound)?;
    let target_level = organization_min_level(conn, target_user_id, organization_id).await?;
    let role_id = role_id_by_name(conn, role_name)
        .await?
        .ok_or(diesel::result::Error::NotFound)?;
    let role_level = role_hierarchy_level(conn, role_id)
        .await?
        .ok_or(diesel::result::Error::NotFound)?;

    if assigner_level >= role_level || target_level.is_some_and(|level| assigner_level >= level) {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    assign_role(conn, target_user_id, organization_id, role_id).await
}

pub(super) async fn organization_min_level(
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

pub(super) async fn role_id_by_name(
    conn: &mut AsyncPgConnection,
    role_name: &str,
) -> QueryResult<Option<i32>> {
    organization_roles::table
        .filter(organization_roles::name.eq(role_name))
        .select(organization_roles::id)
        .first::<i32>(conn)
        .await
        .optional()
}

pub(super) async fn role_hierarchy_level(
    conn: &mut AsyncPgConnection,
    role_id: i32,
) -> QueryResult<Option<i32>> {
    role_organization_hierarchy::table
        .filter(role_organization_hierarchy::organization_role_id.eq(role_id))
        .select(role_organization_hierarchy::hierarchy_level)
        .first::<i32>(conn)
        .await
        .optional()
}

pub(super) async fn assign_role(
    conn: &mut AsyncPgConnection,
    target_user_id: i32,
    organization_id: i32,
    role_id: i32,
) -> QueryResult<usize> {
    diesel::insert_into(user_role_organization::table)
        .values((
            user_role_organization::user_id.eq(target_user_id),
            user_role_organization::organization_id.eq(organization_id),
            user_role_organization::organization_role_id.eq(role_id),
        ))
        .execute(conn)
        .await
}
