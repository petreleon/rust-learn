use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionFilter, DelegatedPermissionOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, organizations};
use crate::infra::postgres::access_control::delegated_permissions::mappers::{
    delegated_permission_output_from_record, map_error,
};
use crate::infra::postgres::access_control::delegated_permissions::records::{
    self, DelegatedPermissionRecordFilter,
};
use crate::infra::postgres::access_control::permission_checks;

pub(super) async fn can_delegate_reward_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    permission_checks::can(
        conn,
        AccessActor::user(user_id),
        AccessAction::permission(Permissions::DELEGATE_REWARD_APPROVAL.to_string()),
        AccessScope::platform(),
    )
    .await
    .map_err(map_error)
}

pub(super) async fn organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    select(exists(
        organizations::table.filter(organizations::id.eq(organization_id)),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(map_error)
}

pub(super) async fn course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    select(exists(courses::table.filter(courses::id.eq(course_id))))
        .get_result::<bool>(conn)
        .await
        .map_err(map_error)
}

pub(super) async fn find_active_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<Option<DelegatedPermissionOutput>, DelegatedPermissionError> {
    records::find_active_delegated_permission(
        conn,
        grantee_user_id,
        permission,
        scope_type,
        organization_id,
        course_id,
    )
    .await
    .map_err(map_error)?
    .map(delegated_permission_output_from_record)
    .transpose()
}

pub(super) async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionFilter,
) -> Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError> {
    records::list_delegated_permissions(conn, DelegatedPermissionRecordFilter::from(filter))
        .await
        .map_err(map_error)?
        .into_iter()
        .map(delegated_permission_output_from_record)
        .collect()
}

impl From<DelegatedPermissionFilter> for DelegatedPermissionRecordFilter {
    fn from(filter: DelegatedPermissionFilter) -> Self {
        Self {
            active: filter.active,
            course_id: filter.course_id,
            grantee_user_id: filter.grantee_user_id,
            grantor_user_id: filter.grantor_user_id,
            limit: filter.limit,
            offset: filter.offset,
            organization_id: filter.organization_id,
            permission: filter.permission,
            scope_type: filter
                .scope_type
                .map(|scope_type| scope_type.as_str().to_string()),
        }
    }
}
