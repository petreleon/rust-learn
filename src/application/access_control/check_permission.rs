use futures::future::BoxFuture;
use std::sync::Arc;

pub type AccessDecisionService = Arc<dyn AccessDecisionUseCase + Send + Sync>;

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
pub struct PlatformScope(());

impl PlatformScope {
    pub fn new() -> Self {
        Self(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseScope {
    course_id: i32,
}

impl CourseScope {
    pub fn new(course_id: i32) -> Self {
        Self { course_id }
    }

    pub fn course_id(&self) -> i32 {
        self.course_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganizationScope {
    organization_id: i32,
}

impl OrganizationScope {
    pub fn new(organization_id: i32) -> Self {
        Self { organization_id }
    }

    pub fn organization_id(&self) -> i32 {
        self.organization_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessScope {
    Platform(PlatformScope),
    Course(CourseScope),
    Organization(OrganizationScope),
}

impl AccessScope {
    pub fn platform() -> Self {
        Self::Platform(PlatformScope::new())
    }

    pub fn course(course_id: i32) -> Self {
        Self::Course(CourseScope::new(course_id))
    }

    pub fn organization(organization_id: i32) -> Self {
        Self::Organization(OrganizationScope::new(organization_id))
    }
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

pub trait AccessDecisionUseCase {
    fn can(
        &self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, AccessDecisionError>>;
}

pub trait AccessDecisionStore {
    type Error;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, Self::Error>>;
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use futures::future::{ready, FutureExt};

    use crate::domain::access_control::permissions::Permissions;

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
    fn access_decision_use_case_captures_actor_action_and_scope() {
        let use_case = FakeDecisionUseCase {
            calls: Mutex::new(Vec::new()),
        };

        let allowed = futures::executor::block_on(use_case.can(
            AccessActor::user(7),
            AccessAction::permission(Permissions::VIEW_REPORT),
            AccessScope::platform(),
        ))
        .unwrap();

        assert!(allowed);
        assert_eq!(
            use_case.calls.into_inner().unwrap(),
            vec![(7, "VIEW_REPORT".to_string(), AccessScope::platform())]
        );
    }
}
