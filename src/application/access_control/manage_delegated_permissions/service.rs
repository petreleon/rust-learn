use futures::future::BoxFuture;

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionOutput, GrantDelegatedPermissionCommand,
    ListDelegatedPermissionsQuery, RevokeDelegatedPermissionCommand,
};

pub trait DelegatedPermissionUseCase: Send + Sync {
    fn grant_delegated_permission(
        &self,
        command: GrantDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>>;

    fn list_delegated_permissions(
        &self,
        query: ListDelegatedPermissionsQuery,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>>;

    fn revoke_delegated_permission(
        &self,
        command: RevokeDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>>;
}
