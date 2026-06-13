use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};
use serde_json::json;

use crate::application::rewards::decide_teacher_candidate::{
    decide_teacher_reward_candidate, TeacherRewardCandidateDecision,
    TeacherRewardCandidateDecisionCommand, TeacherRewardCandidateDecisionError,
    TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionStore,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

struct FakeStore {
    can_approve: bool,
    decision: Option<TeacherRewardCandidateDecision>,
}

impl FakeStore {
    fn approving() -> Self {
        Self {
            can_approve: true,
            decision: None,
        }
    }

    fn denying() -> Self {
        Self {
            can_approve: false,
            decision: None,
        }
    }
}

impl TeacherRewardCandidateDecisionStore for FakeStore {
    fn can_approve_student_reward_candidate(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherRewardCandidateDecisionError>> {
        ready(Ok(self.can_approve)).boxed()
    }

    fn decide_teacher_reward_candidate(
        &mut self,
        decision: TeacherRewardCandidateDecision,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    > {
        self.decision = Some(decision.clone());
        ready(Ok(output(decision.target_status.as_str()))).boxed()
    }
}

#[test]
fn delegates_normalized_teacher_approval() {
    let mut store = FakeStore::approving();
    let result = block_on(decide_teacher_reward_candidate(
        &mut store,
        7,
        11,
        19,
        TeacherRewardCandidateDecisionCommand {
            status: " approved ".to_string(),
            decision_reason: Some("complete".to_string()),
        },
    ))
    .unwrap();

    assert_eq!(result.status, "teacher_approved");
    let decision = store.decision.unwrap();
    assert_eq!(decision.actor_user_id, 7);
    assert_eq!(decision.course_id, 11);
    assert_eq!(decision.candidate_id, 19);
    assert_eq!(
        decision.target_status,
        RewardCandidateStatus::TeacherApproved
    );
    assert_eq!(decision.decision_reason, Some("complete".to_string()));
}

#[test]
fn rejects_unsupported_status_before_store_mutation() {
    let mut store = FakeStore::approving();
    let error = block_on(decide_teacher_reward_candidate(
        &mut store,
        7,
        11,
        19,
        TeacherRewardCandidateDecisionCommand {
            status: "pending_teacher_approval".to_string(),
            decision_reason: None,
        },
    ))
    .unwrap_err();

    assert_eq!(
        error,
        TeacherRewardCandidateDecisionError::InvalidStatus(
            "unsupported teacher reward decision status".to_string()
        )
    );
    assert!(store.decision.is_none());
}

#[test]
fn denies_without_course_permission_before_store_mutation() {
    let mut store = FakeStore::denying();
    let error = block_on(decide_teacher_reward_candidate(
        &mut store,
        7,
        11,
        19,
        TeacherRewardCandidateDecisionCommand {
            status: "rejected".to_string(),
            decision_reason: None,
        },
    ))
    .unwrap_err();

    assert_eq!(
        error,
        TeacherRewardCandidateDecisionError::PermissionDenied(
            "APPROVE_STUDENT_REWARD_CANDIDATE".to_string()
        )
    );
    assert!(store.decision.is_none());
}

fn output(status: &str) -> TeacherRewardCandidateDecisionOutput {
    let now = Utc::now();
    TeacherRewardCandidateDecisionOutput {
        id: 19,
        course_id: 11,
        student_user_id: 23,
        submitter_user_id: 7,
        source_scope: "course".to_string(),
        source_organization_id: None,
        event_type: "manual_completion".to_string(),
        idempotency_key: "manual:11:23".to_string(),
        evidence: json!({}),
        status: status.to_string(),
        teacher_approver_user_id: Some(7),
        teacher_decision_reason: Some("complete".to_string()),
        teacher_decided_at: Some(now),
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: now,
        updated_at: now,
    }
}
