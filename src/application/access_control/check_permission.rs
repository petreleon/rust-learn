use futures::future::BoxFuture;
use std::sync::Arc;

pub type PermissionCheckService = Arc<dyn PermissionCheckUseCase + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessActor {
    pub user_id: i32,
}

impl AccessActor {
    pub fn user(user_id: i32) -> Self {
        Self { user_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessScope {
    Platform,
    Course { course_id: i32 },
    Organization { organization_id: i32 },
}

pub type PermissionScope = AccessScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessAction {
    Permission(String),
}

impl AccessAction {
    pub fn permission(permission: impl Into<String>) -> Self {
        Self::Permission(permission.into())
    }

    pub fn permission_name(&self) -> &str {
        match self {
            Self::Permission(permission) => permission,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessDecisionError {
    Connection(String),
    Query(String),
}

pub type PermissionCheckError = AccessDecisionError;

pub trait AccessDecisionUseCase {
    fn can(
        &self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, AccessDecisionError>>;
}

pub trait PermissionCheckUseCase: AccessDecisionUseCase {
    fn has_permission(
        &self,
        actor_user_id: i32,
        scope: AccessScope,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, AccessDecisionError>> {
        self.can(
            AccessActor::user(actor_user_id),
            AccessAction::permission(permission),
            scope,
        )
    }
}

impl<T> PermissionCheckUseCase for T where T: AccessDecisionUseCase + ?Sized {}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use futures::future::{ready, FutureExt};

    use super::*;

    struct FakeDecisionUseCase {
        calls: Mutex<Vec<(i32, String, AccessScope)>>,
    }

    impl AccessDecisionUseCase for FakeDecisionUseCase {
        fn can(
            &self,
            actor: AccessActor,
            action: AccessAction,
            scope: AccessScope,
        ) -> BoxFuture<'_, Result<bool, AccessDecisionError>> {
            let permission = action.permission_name().to_string();
            self.calls
                .lock()
                .unwrap()
                .push((actor.user_id, permission.clone(), scope));
            ready(Ok(permission == "VIEW_REPORT")).boxed()
        }
    }

    #[test]
    fn legacy_permission_check_delegates_to_can_decision() {
        let use_case = FakeDecisionUseCase {
            calls: Mutex::new(Vec::new()),
        };

        let allowed = futures::executor::block_on(use_case.has_permission(
            7,
            AccessScope::Platform,
            "VIEW_REPORT".to_string(),
        ))
        .unwrap();

        assert!(allowed);
        assert_eq!(
            use_case.calls.into_inner().unwrap(),
            vec![(7, "VIEW_REPORT".to_string(), AccessScope::Platform)]
        );
    }
}
