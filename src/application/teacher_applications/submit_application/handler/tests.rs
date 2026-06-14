use futures::future::{BoxFuture, FutureExt};

use super::*;
use fixtures::{application, application_from_submission, command};

mod fixtures;

#[derive(Default)]
struct FakeStore {
    can_submit: bool,
    existing_by_key: Option<TeacherApplicationOutput>,
    latest_application: Option<TeacherApplicationOutput>,
    created: Option<TeacherApplicationSubmission>,
}

impl TeacherApplicationSubmitStore for FakeStore {
    fn can_submit_teacher_application(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationSubmitError>> {
        async move { Ok(self.can_submit) }.boxed()
    }

    fn find_application_by_idempotency_key(
        &mut self,
        _: String,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSubmitError>>
    {
        async move { Ok(self.existing_by_key.clone()) }.boxed()
    }

    fn find_latest_application_for_applicant(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSubmitError>>
    {
        async move { Ok(self.latest_application.clone()) }.boxed()
    }

    fn create_submitted_application(
        &mut self,
        _: i32,
        submission: TeacherApplicationSubmission,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationSubmitError>> {
        self.created = Some(submission.clone());
        async move { Ok(application_from_submission(1, submission)) }.boxed()
    }
}

#[tokio::test]
async fn denies_actor_without_submit_permission() {
    let mut store = FakeStore::default();

    let error = submit_application(&mut store, command()).await.unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationSubmitError::PermissionDenied(permission)
            if permission == "SUBMIT_TEACHER_APPLICATION"
    ));
    assert!(store.created.is_none());
}

#[tokio::test]
async fn trims_and_creates_submitted_application() {
    let mut store = FakeStore {
        can_submit: true,
        ..Default::default()
    };
    let mut command = command();
    command.experience_summary = "  Experience  ".to_string();
    command.portfolio_links = Some(vec![" https://a.test ".to_string(), " ".to_string()]);
    command.idempotency_key = Some(" retry ".to_string());

    let output = submit_application(&mut store, command).await.unwrap();

    assert_eq!(output.experience_summary, "Experience");
    assert_eq!(
        output.portfolio_links,
        serde_json::json!(["https://a.test"])
    );
    assert_eq!(output.idempotency_key.as_deref(), Some("retry"));
    assert_eq!(output.status, "submitted");
}

#[tokio::test]
async fn returns_matching_idempotent_application_without_creating() {
    let existing = application();
    let mut store = FakeStore {
        can_submit: true,
        existing_by_key: Some(existing.clone()),
        ..Default::default()
    };
    let mut command = command();
    command.idempotency_key = existing.idempotency_key.clone();

    let output = submit_application(&mut store, command).await.unwrap();

    assert_eq!(output.id, existing.id);
    assert!(store.created.is_none());
}

#[tokio::test]
async fn rejects_different_payload_for_same_idempotency_key() {
    let mut existing = application();
    existing.experience_summary = "Different".to_string();
    let mut store = FakeStore {
        can_submit: true,
        existing_by_key: Some(existing),
        ..Default::default()
    };
    let mut command = command();
    command.idempotency_key = Some("retry".to_string());

    let error = submit_application(&mut store, command).await.unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationSubmitError::InvalidInput(message)
            if message.contains("idempotency key is already used")
    ));
    assert!(store.created.is_none());
}

#[tokio::test]
async fn rejects_existing_non_rejected_application() {
    let mut store = FakeStore {
        can_submit: true,
        latest_application: Some(application()),
        ..Default::default()
    };

    let error = submit_application(&mut store, command()).await.unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationSubmitError::InvalidTransition(message)
            if message.contains("already exists")
    ));
    assert!(store.created.is_none());
}
