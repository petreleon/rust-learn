use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{delegated_permissions, user_role_organization};
use crate::domain::access_control::delegation::DELEGATED_SCOPE_ORGANIZATION;

pub(super) async fn has_any_organization_role(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> QueryResult<bool> {
    select(exists(
        user_role_organization::table
            .filter(user_role_organization::user_id.eq(Some(actor_user_id)))
            .filter(user_role_organization::organization_id.eq(Some(organization_id))),
    ))
    .get_result(conn)
    .await
}

pub(super) async fn has_any_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result(conn)
    .await
}
