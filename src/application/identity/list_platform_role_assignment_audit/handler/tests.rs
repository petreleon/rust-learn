use chrono::{TimeZone, Utc};
use futures::future::{ready, BoxFuture, FutureExt};

use super::*;
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::domain::access_control::role_assignment_audit::PlatformRoleAssignmentAuditEventType;

#[derive(Default)]
struct FakeAuditStore {
    allowed: bool,
    exists: bool,
    listed_user_id: Option<i32>,
}

impl AccessDecisionStore for FakeAuditStore {
    type Error = PlatformRoleAssignmentAuditError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        _scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, Self::Error>> {
        let allowed = self.allowed && action.permission_name() == "VIEW_ROLE_ASSIGNMENTS";
        ready(Ok(allowed)).boxed()
    }
}

impl PlatformRoleAssignmentAuditStore for FakeAuditStore {
    fn target_user_exists(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRoleAssignmentAuditError>> {
        ready(Ok(self.exists && user_id == 9)).boxed()
    }

    fn list_assignment_audit(
        &mut self,
        target_user_id: i32,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError>,
    > {
        self.listed_user_id = Some(target_user_id);
        ready(Ok(vec![event()])).boxed()
    }
}

#[tokio::test]
async fn lists_audit_after_permission_and_user_checks() {
    let mut store = FakeAuditStore {
        allowed: true,
        exists: true,
        listed_user_id: None,
    };

    let events = list_platform_role_assignment_audit(&mut store, query())
        .await
        .unwrap();

    assert_eq!(store.listed_user_id, Some(9));
    assert_eq!(events, vec![event()]);
}

#[tokio::test]
async fn denies_without_role_assignment_view_permission() {
    let mut store = FakeAuditStore {
        allowed: false,
        exists: true,
        listed_user_id: None,
    };

    let error = list_platform_role_assignment_audit(&mut store, query())
        .await
        .unwrap_err();

    assert_eq!(
        error,
        PlatformRoleAssignmentAuditError::PermissionDenied("VIEW_ROLE_ASSIGNMENTS".to_string())
    );
    assert_eq!(store.listed_user_id, None);
}

#[tokio::test]
async fn returns_not_found_before_listing_audit() {
    let mut store = FakeAuditStore {
        allowed: true,
        exists: false,
        listed_user_id: None,
    };

    let error = list_platform_role_assignment_audit(&mut store, query())
        .await
        .unwrap_err();

    assert_eq!(error, PlatformRoleAssignmentAuditError::UserNotFound);
    assert_eq!(store.listed_user_id, None);
}

fn query() -> PlatformRoleAssignmentAuditQuery {
    PlatformRoleAssignmentAuditQuery {
        actor_user_id: 7,
        target_user_id: 9,
    }
}

fn event() -> PlatformRoleAssignmentAuditEventOutput {
    PlatformRoleAssignmentAuditEventOutput {
        actor_user_id: Some(7),
        created_at: Utc.with_ymd_and_hms(2026, 6, 16, 10, 0, 0).unwrap(),
        event_type: PlatformRoleAssignmentAuditEventType::RoleAssigned,
        id: 1,
        platform_role_id: Some(3),
        role_name: "SUPPORT_ADMIN".to_string(),
        target_user_id: 9,
    }
}
