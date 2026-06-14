use futures::future::BoxFuture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionScope {
    Platform,
    Course { course_id: i32 },
    Organization { organization_id: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionCheckError {
    Connection(String),
    Query(String),
}

pub trait PermissionCheckUseCase {
    fn has_permission(
        &self,
        actor_user_id: i32,
        scope: PermissionScope,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, PermissionCheckError>>;
}
