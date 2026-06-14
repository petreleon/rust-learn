use crate::infra::postgres::access_control::delegated_permissions::records::{
    self as delegated_permission_records, DelegatedPermissionRecordFilter,
};
use crate::models::delegated_permission::{DelegatedPermission, NewDelegatedPermission};
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

#[derive(Debug, Default)]
pub struct DelegatedPermissionFilter {
    pub grantor_user_id: Option<i32>,
    pub grantee_user_id: Option<i32>,
    pub permission: Option<String>,
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    new_delegation: NewDelegatedPermission,
) -> QueryResult<DelegatedPermission> {
    delegated_permission_records::create_delegated_permission(conn, new_delegation).await
}

pub async fn find_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
) -> QueryResult<DelegatedPermission> {
    delegated_permission_records::find_delegated_permission(conn, delegation_id).await
}

pub async fn find_active_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> QueryResult<Option<DelegatedPermission>> {
    delegated_permission_records::find_active_delegated_permission(
        conn,
        grantee_user_id,
        permission,
        scope_type,
        organization_id,
        course_id,
    )
    .await
}

pub async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionFilter,
) -> QueryResult<Vec<DelegatedPermission>> {
    delegated_permission_records::list_delegated_permissions(conn, filter.into()).await
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
            scope_type: filter.scope_type,
        }
    }
}
