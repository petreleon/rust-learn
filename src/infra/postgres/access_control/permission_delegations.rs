use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::delegated_permissions;
use crate::domain::access_control::delegation::{
    DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};

pub(crate) async fn has_active_platform_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
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

pub(crate) async fn has_active_course_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_COURSE))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.eq(Some(course_id)))
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

pub(crate) async fn has_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::permission.eq(permission))
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
