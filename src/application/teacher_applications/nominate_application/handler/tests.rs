use super::*;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
#[derive(Default)]
struct FakeStore {
    can_nominate: bool,
    idempotent: Option<TeacherApplicationOutput>,
    latest: Option<TeacherApplicationOutput>,
    created: Option<TeacherApplicationSubmission>,
    permission_checked: Option<(i32, String)>,
}
impl TeacherApplicationNominationStore for FakeStore {
    fn has_organization_permission(
        &mut self,
        _: i32,
        organization_id: i32,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationNominationError>> {
        self.permission_checked = Some((organization_id, permission));
        async move { Ok(self.can_nominate) }.boxed()
    }
    fn find_application_by_idempotency_key(
        &mut self,
        _: String,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>
    {
        async move { Ok(self.idempotent.clone()) }.boxed()
    }
    fn find_latest_application_for_applicant(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>
    {
        async move { Ok(self.latest.clone()) }.boxed()
    }
    fn create_organization_nomination(
        &mut self,
        _: i32,
        submission: TeacherApplicationSubmission,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>> {
        self.created = Some(submission.clone());
        async move { Ok(application_from(submission)) }.boxed()
    }
}
#[tokio::test]
async fn denies_missing_organization_permission() {
    let mut store = FakeStore::default();
    let error = nominate_application(&mut store, command())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        TeacherApplicationNominationError::PermissionDenied(permission)
            if permission == "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW"
    ));
    assert_eq!(
        store.permission_checked,
        Some((7, "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW".to_string()))
    );
    assert!(store.created.is_none());
}
#[tokio::test]
async fn creates_default_organization_nomination() {
    let mut store = FakeStore {
        can_nominate: true,
        ..Default::default()
    };
    let application = nominate_application(&mut store, command()).await.unwrap();
    let created = store.created.expect("submission should be persisted");
    assert_eq!(created.applicant_user_id, 22);
    assert_eq!(created.requested_scope, "organization");
    assert_eq!(created.requested_organization_id, Some(7));
    assert_eq!(created.organization_sponsor_id, Some(7));
    assert_eq!(created.experience_summary, "Org-backed Rust mentor");
    assert_eq!(
        created.portfolio_links,
        serde_json::json!(["https://example.test"])
    );
    assert_eq!(application.status, "submitted");
}
#[tokio::test]
async fn rejects_active_application_for_applicant() {
    let mut store = FakeStore {
        can_nominate: true,
        latest: Some(application("submitted")),
        ..Default::default()
    };
    let error = nominate_application(&mut store, command())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        TeacherApplicationNominationError::InvalidTransition(message)
            if message.contains("teacher application already exists")
    ));
    assert!(store.created.is_none());
}
#[tokio::test]
async fn rejects_idempotency_key_for_different_application() {
    let mut command = command();
    command.idempotency_key = Some("nominate-once".to_string());
    let mut existing = application("submitted");
    existing.experience_summary = "Different".to_string();
    let mut store = FakeStore {
        can_nominate: true,
        idempotent: Some(existing),
        ..Default::default()
    };
    let error = nominate_application(&mut store, command).await.unwrap_err();
    assert!(matches!(
        error,
        TeacherApplicationNominationError::InvalidInput(message)
            if message.contains("idempotency key")
    ));
    assert!(store.created.is_none());
}
fn command() -> TeacherApplicationNominationCommand {
    TeacherApplicationNominationCommand {
        actor_user_id: 11,
        applicant_user_id: 22,
        experience_summary: "  Org-backed Rust mentor  ".to_string(),
        idempotency_key: None,
        organization_id: 7,
        portfolio_links: Some(vec![" https://example.test ".to_string(), " ".to_string()]),
        requested_course_id: None,
        requested_scope: None,
    }
}
fn application(status: &str) -> TeacherApplicationOutput {
    let mut submission = submission();
    submission.status = status.to_string();
    application_from(submission)
}
fn application_from(submission: TeacherApplicationSubmission) -> TeacherApplicationOutput {
    let now = Utc::now();
    TeacherApplicationOutput {
        applicant_user_id: submission.applicant_user_id,
        created_at: now,
        decided_at: None,
        decision_reason: None,
        experience_summary: submission.experience_summary,
        id: 101,
        idempotency_key: submission.idempotency_key,
        organization_sponsor_id: submission.organization_sponsor_id,
        portfolio_links: submission.portfolio_links,
        requested_course_id: submission.requested_course_id,
        requested_organization_id: submission.requested_organization_id,
        requested_scope: submission.requested_scope,
        reviewer_id: None,
        status: submission.status,
        updated_at: now,
    }
}
fn submission() -> TeacherApplicationSubmission {
    TeacherApplicationSubmission {
        applicant_user_id: 22,
        experience_summary: "Org-backed Rust mentor".to_string(),
        idempotency_key: None,
        organization_sponsor_id: Some(7),
        portfolio_links: serde_json::json!(["https://example.test"]),
        requested_course_id: None,
        requested_organization_id: Some(7),
        requested_scope: "organization".to_string(),
        status: "submitted".to_string(),
    }
}
