use std::sync::Arc;

use actix_web::web;
use bigdecimal::BigDecimal;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::rewards::decide_amount::{
    RewardAmountDecisionCommand, RewardAmountDecisionError, RewardAmountDecisionOutput,
    RewardAmountDecisionUseCase,
};
use serde_json::json;

struct RouteOnlyRewardAmountDecisionUseCase;

pub fn reward_amount_decision_data() -> web::Data<Arc<dyn RewardAmountDecisionUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyRewardAmountDecisionUseCase) as Arc<dyn RewardAmountDecisionUseCase>
    )
}

impl RewardAmountDecisionUseCase for RouteOnlyRewardAmountDecisionUseCase {
    fn decide_reward_amount(
        &self,
        _actor_user_id: i32,
        candidate_id: i64,
        _command: RewardAmountDecisionCommand,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>> {
        ready(Ok(amount_decision_output(candidate_id))).boxed()
    }
}

fn amount_decision_output(candidate_id: i64) -> RewardAmountDecisionOutput {
    let now = Utc::now();
    RewardAmountDecisionOutput {
        id: candidate_id,
        course_id: 12,
        student_user_id: 23,
        submitter_user_id: 7,
        source_scope: "course".to_string(),
        source_organization_id: None,
        event_type: "manual_completion".to_string(),
        idempotency_key: "manual:12:23".to_string(),
        evidence: json!({}),
        status: "amount_approved".to_string(),
        teacher_approver_user_id: Some(7),
        teacher_decision_reason: Some("route smoke".to_string()),
        teacher_decided_at: Some(now),
        amount_reviewer_user_id: Some(9),
        approved_amount: Some(BigDecimal::from(25)),
        amount_decision_reason: Some("route smoke".to_string()),
        amount_decided_at: Some(now),
        created_at: now,
        updated_at: now,
    }
}
