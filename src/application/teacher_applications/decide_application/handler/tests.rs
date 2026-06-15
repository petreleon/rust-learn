use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};

use super::*;

#[derive(Default)]
struct FakeStore {
    has_permission: bool,
    application: Option<TeacherApplicationOutput>,
    permission_checked: Option<String>,
    applied: Option<(String, Option<String>)>,
}

impl AccessDecisionStore for FakeStore {
    type Error = TeacherApplicationDecisionError;

    fn can(
        &mut self,
        _: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationDecisionError>> {
        assert!(matches!(scope, AccessScope::Platform(_)));
        self.permission_checked = Some(action.permission_name().to_string());
        async move { Ok(self.has_permission) }.boxed()
    }
}

impl TeacherApplicationDecisionStore for FakeStore {
    fn application(
        &mut self,
        _: i64,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        async move {
            self.application
                .clone()
                .ok_or(TeacherApplicationDecisionError::NotFound)
        }
        .boxed()
    }

    fn apply_decision(
        &mut self,
        _: i32,
        mut current: TeacherApplicationOutput,
        target_status: String,
        decision_reason: Option<String>,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        self.applied = Some((target_status.clone(), decision_reason.clone()));
        async move {
            current.status = target_status;
            current.decision_reason = decision_reason;
            Ok(current)
        }
        .boxed()
    }
}

#[tokio::test]
async fn denies_missing_target_permission() {
    let mut store = FakeStore {
        application: Some(application("submitted")),
        ..Default::default()
    };

    let error = decide_application(&mut store, command("approved"))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationDecisionError::PermissionDenied(permission)
            if permission == "APPROVE_TEACHER_APPLICATION"
    ));
    assert_eq!(
        store.permission_checked.as_deref(),
        Some("APPROVE_TEACHER_APPLICATION")
    );
    assert!(store.applied.is_none());
}

#[tokio::test]
async fn rejects_submitted_as_decision_status() {
    let mut store = FakeStore {
        has_permission: true,
        application: Some(application("submitted")),
        ..Default::default()
    };

    let error = decide_application(&mut store, command("submitted"))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationDecisionError::InvalidInput(message)
            if message.contains("decision status")
    ));
    assert!(store.permission_checked.is_none());
}

#[tokio::test]
async fn rejects_final_application_without_applying_decision() {
    let mut store = FakeStore {
        has_permission: true,
        application: Some(application("approved")),
        ..Default::default()
    };

    let error = decide_application(&mut store, command("rejected"))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationDecisionError::InvalidTransition(message)
            if message.contains("final teacher applications")
    ));
    assert!(store.applied.is_none());
}

#[tokio::test]
async fn applies_approved_decision() {
    let mut store = FakeStore {
        has_permission: true,
        application: Some(application("submitted")),
        ..Default::default()
    };

    let output = decide_application(&mut store, command(" approved "))
        .await
        .unwrap();

    assert_eq!(output.status, "approved");
    assert_eq!(
        store.applied,
        Some(("approved".to_string(), Some("ready".to_string())))
    );
}

fn command(status: &str) -> TeacherApplicationDecisionCommand {
    TeacherApplicationDecisionCommand {
        actor_user_id: 7,
        application_id: 11,
        decision_reason: Some("ready".to_string()),
        status: status.to_string(),
    }
}

fn application(status: &str) -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: 9,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: "Experience".to_string(),
        id: 11,
        idempotency_key: None,
        organization_sponsor_id: None,
        portfolio_links: serde_json::json!([]),
        requested_course_id: None,
        requested_organization_id: None,
        requested_scope: "platform".to_string(),
        reviewer_id: None,
        status: status.to_string(),
        updated_at: now,
    }
}
