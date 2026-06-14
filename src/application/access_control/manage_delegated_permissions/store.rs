use chrono::{DateTime, Utc};
use futures::future::BoxFuture;

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedPermissionCreate {
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

pub trait DelegatedPermissionStore {
    fn can_delegate_reward_permissions(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>>;

    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>>;

    fn find_active_delegated_permission(
        &mut self,
        grantee_user_id: i32,
        permission: String,
        scope_type: String,
        organization_id: Option<i32>,
        course_id: Option<i32>,
    ) -> BoxFuture<'_, Result<Option<DelegatedPermissionOutput>, DelegatedPermissionError>>;

    fn create_delegated_permission(
        &mut self,
        delegation: DelegatedPermissionCreate,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>>;

    fn list_delegated_permissions(
        &mut self,
        filter: DelegatedPermissionFilter,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>>;

    fn revoke_delegated_permission(
        &mut self,
        delegation_id: i64,
        revoked_by_user_id: i32,
        revoke_reason: Option<String>,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>>;
}
