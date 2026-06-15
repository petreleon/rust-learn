use bigdecimal::BigDecimal;
use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};
use serde_json::json;

use crate::application::rewards::decide_amount::{
    decide_reward_amount, RewardAmountDecision, RewardAmountDecisionCommand,
    RewardAmountDecisionError, RewardAmountDecisionOutput, RewardAmountDecisionStore,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

struct FakeStore {
    can_approve: bool,
    decision: Option<RewardAmountDecision>,
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

impl RewardAmountDecisionStore for FakeStore {
    fn can_approve_reward_amount(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardAmountDecisionError>> {
        ready(Ok(self.can_approve)).boxed()
    }

    fn decide_reward_amount(
        &mut self,
        decision: RewardAmountDecision,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>> {
        self.decision = Some(decision.clone());
        ready(Ok(output(decision.target_status, decision.approved_amount))).boxed()
    }
}

#[test]
fn delegates_normalized_amount_approval() {
    let mut store = FakeStore::approving();
    let result = block_on(decide_reward_amount(
        &mut store,
        7,
        19,
        RewardAmountDecisionCommand {
            status: " approved ".to_string(),
            approved_amount: Some(BigDecimal::from(25)),
            decision_reason: Some("ok".to_string()),
        },
    ))
    .unwrap();

    assert_eq!(result.status, RewardCandidateStatus::AmountApproved);
    assert_eq!(result.approved_amount, Some(BigDecimal::from(25)));
    let decision = store.decision.unwrap();
    assert_eq!(decision.actor_user_id, 7);
    assert_eq!(decision.candidate_id, 19);
    assert_eq!(
        decision.target_status,
        RewardCandidateStatus::AmountApproved
    );
    assert_eq!(decision.approved_amount, Some(BigDecimal::from(25)));
}

#[test]
fn denies_without_platform_permission_before_amount_validation() {
    let mut store = FakeStore::denying();
    let error = block_on(decide_reward_amount(
        &mut store,
        7,
        19,
        RewardAmountDecisionCommand {
            status: "approved".to_string(),
            approved_amount: None,
            decision_reason: None,
        },
    ))
    .unwrap_err();

    assert_eq!(
        error,
        RewardAmountDecisionError::PermissionDenied("APPROVE_REWARD_AMOUNT".to_string())
    );
    assert!(store.decision.is_none());
}

#[test]
fn rejects_invalid_amount_before_store_mutation() {
    let mut store = FakeStore::approving();
    let error = block_on(decide_reward_amount(
        &mut store,
        7,
        19,
        RewardAmountDecisionCommand {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(-1)),
            decision_reason: None,
        },
    ))
    .unwrap_err();

    assert_eq!(
        error,
        RewardAmountDecisionError::InvalidInput("approved amount cannot be negative".to_string())
    );
    assert!(store.decision.is_none());
}

fn output(
    status: RewardCandidateStatus,
    approved_amount: Option<BigDecimal>,
) -> RewardAmountDecisionOutput {
    let now = Utc::now();
    RewardAmountDecisionOutput {
        id: 19,
        course_id: 11,
        student_user_id: 23,
        submitter_user_id: 7,
        source_scope: "course".to_string(),
        source_organization_id: None,
        event_type: "manual_completion".to_string(),
        idempotency_key: "manual:11:23".to_string(),
        evidence: json!({}),
        status,
        teacher_approver_user_id: Some(7),
        teacher_decision_reason: Some("complete".to_string()),
        teacher_decided_at: Some(now),
        amount_reviewer_user_id: Some(7),
        approved_amount,
        amount_decision_reason: Some("ok".to_string()),
        amount_decided_at: Some(now),
        created_at: now,
        updated_at: now,
    }
}
