use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};
use serde_json::json;

use crate::application::rewards::submit_candidate::{
    submit_course_reward_candidate, submit_organization_reward_candidate,
    RewardCandidateSubmission, RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionStore, SubmitRewardCandidateCommand,
};
use crate::domain::rewards::candidate::source::{REWARD_SOURCE_COURSE, REWARD_SOURCE_ORGANIZATION};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

struct FakeStore {
    course_exists: bool,
    attached: bool,
    course_permission: bool,
    organization_permission: bool,
    submission: Option<RewardCandidateSubmission>,
}

impl FakeStore {
    fn allowing() -> Self {
        Self {
            course_exists: true,
            attached: true,
            course_permission: true,
            organization_permission: true,
            submission: None,
        }
    }
}

impl RewardCandidateSubmissionStore for FakeStore {
    fn course_exists(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardCandidateSubmissionError>> {
        if self.course_exists {
            ready(Ok(())).boxed()
        } else {
            ready(Err(RewardCandidateSubmissionError::NotFound)).boxed()
        }
    }

    fn course_attached_to_organization(
        &mut self,
        _course_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        ready(Ok(self.attached)).boxed()
    }

    fn can_submit_course_reward_event(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        ready(Ok(self.course_permission)).boxed()
    }

    fn can_submit_organization_course_reward_event(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateSubmissionError>> {
        ready(Ok(self.organization_permission)).boxed()
    }

    fn submit_reward_candidate(
        &mut self,
        submission: RewardCandidateSubmission,
    ) -> BoxFuture<'_, Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError>>
    {
        self.submission = Some(submission);
        ready(Ok(output())).boxed()
    }
}

#[test]
fn course_submission_uses_course_source_after_permission() {
    let mut store = FakeStore::allowing();
    block_on(submit_course_reward_candidate(&mut store, 7, 11, command())).unwrap();

    let submission = store.submission.unwrap();
    assert_eq!(submission.actor_user_id, 7);
    assert_eq!(submission.course_id, 11);
    assert_eq!(submission.source_scope, REWARD_SOURCE_COURSE);
    assert_eq!(submission.source_organization_id, None);
}

#[test]
fn organization_submission_uses_organization_source_after_attachment_and_permission() {
    let mut store = FakeStore::allowing();
    block_on(submit_organization_reward_candidate(
        &mut store,
        7,
        13,
        11,
        command(),
    ))
    .unwrap();

    let submission = store.submission.unwrap();
    assert_eq!(submission.course_id, 11);
    assert_eq!(submission.source_scope, REWARD_SOURCE_ORGANIZATION);
    assert_eq!(submission.source_organization_id, Some(13));
}

#[test]
fn denies_course_submission_before_store_mutation() {
    let mut store = FakeStore {
        course_permission: false,
        ..FakeStore::allowing()
    };
    let error = block_on(submit_course_reward_candidate(&mut store, 7, 11, command())).unwrap_err();

    assert_eq!(
        error,
        RewardCandidateSubmissionError::PermissionDenied("SUBMIT_COURSE_REWARD_EVENT".to_string())
    );
    assert!(store.submission.is_none());
}

#[test]
fn rejects_unattached_organization_course_before_store_mutation() {
    let mut store = FakeStore {
        attached: false,
        ..FakeStore::allowing()
    };
    let error = block_on(submit_organization_reward_candidate(
        &mut store,
        7,
        13,
        11,
        command(),
    ))
    .unwrap_err();

    assert_eq!(
        error,
        RewardCandidateSubmissionError::InvalidInput(
            "course is not attached to the organization".to_string()
        )
    );
    assert!(store.submission.is_none());
}

fn command() -> SubmitRewardCandidateCommand {
    SubmitRewardCandidateCommand {
        student_user_id: 23,
        event_type: "manual_completion".to_string(),
        idempotency_key: Some("manual:11:23".to_string()),
        evidence: Some(json!({})),
    }
}

fn output() -> RewardCandidateSubmissionOutput {
    let now = Utc::now();
    RewardCandidateSubmissionOutput {
        id: 19,
        course_id: 11,
        student_user_id: 23,
        submitter_user_id: 7,
        source_scope: REWARD_SOURCE_COURSE.to_string(),
        source_organization_id: None,
        event_type: "manual_completion".to_string(),
        idempotency_key: "manual:11:23".to_string(),
        evidence: json!({}),
        status: RewardCandidateStatus::PendingTeacherApproval,
        teacher_approver_user_id: None,
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: now,
        updated_at: now,
    }
}
