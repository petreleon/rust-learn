use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};
use serde_json::json;

use crate::application::rewards::decide_amount::{
    RewardAmountDecision, RewardAmountDecisionError, RewardAmountDecisionOutput,
    RewardAmountDecisionStore,
};
use crate::domain::rewards::audit::RewardAuditEventType;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_amount_decision_mappers::{
    map_reward_amount_decision_candidate, map_reward_amount_decision_error,
};
use crate::infra::postgres::rewards::reward_amount_decision_transition::{
    candidate_teacher_user_ids, ensure_amount_transition,
};
use crate::infra::postgres::rewards::reward_audit_records::create_reward_audit_event;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::ensure_no_active_reward_fraud_block;
use crate::infra::postgres::rewards::reward_candidate_records::{
    find_candidate, update_amount_decision,
};
use crate::infra::postgres::rewards::reward_execution_job_records::enqueue_reward_execution_job;
use crate::models::reward_audit_event::NewRewardAuditEvent;

pub struct PostgresRewardAmountDecisionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardAmountDecisionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardAmountDecisionStore for PostgresRewardAmountDecisionStore<'_> {
    fn can_approve_reward_amount(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardAmountDecisionError>> {
        async move {
            reward_authorization_access::can_approve_reward_amount(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardAmountDecisionError::Database(error.to_string()))
        }
        .boxed()
    }

    fn decide_reward_amount(
        &mut self,
        decision: RewardAmountDecision,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>> {
        async move {
            let actor_user_id = decision.actor_user_id;
            let updated = self
                .conn
                .transaction::<_, RewardAmountTransactionError, _>(|conn| {
                    Box::pin(async move {
                        apply_amount_decision(conn, decision)
                            .await
                            .map_err(RewardAmountTransactionError::Application)
                    })
                })
                .await
                .map_err(RewardAmountDecisionError::from)?;

            let approved_amount = updated
                .approved_amount
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "none".to_string());
            log::info!(
                "event=reward_candidate_amount_decision candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} approved_amount={} event_type={} source_scope={} source_organization_id={:?}",
                updated.id,
                actor_user_id,
                updated.student_user_id,
                updated.course_id,
                updated.status,
                approved_amount,
                updated.event_type,
                updated.source_scope,
                updated.source_organization_id
            );

            Ok(updated)
        }
        .boxed()
    }
}

enum RewardAmountTransactionError {
    Application(RewardAmountDecisionError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardAmountTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardAmountTransactionError> for RewardAmountDecisionError {
    fn from(error: RewardAmountTransactionError) -> Self {
        match error {
            RewardAmountTransactionError::Application(error) => error,
            RewardAmountTransactionError::Diesel(error) => map_reward_amount_decision_error(error),
        }
    }
}

async fn apply_amount_decision(
    conn: &mut AsyncPgConnection,
    decision: RewardAmountDecision,
) -> Result<RewardAmountDecisionOutput, RewardAmountDecisionError> {
    let existing = find_candidate(conn, decision.candidate_id)
        .await
        .map_err(map_reward_amount_decision_error)?;
    if existing.status == decision.target_status.as_str() {
        return map_reward_amount_decision_candidate(existing);
    }

    ensure_amount_transition(&existing.status, decision.target_status)?;
    let from_status = existing.status.clone();
    let teacher_user_ids = candidate_teacher_user_ids(&existing);
    ensure_no_active_reward_fraud_block(
        conn,
        teacher_user_ids.as_slice(),
        existing.course_id,
        &existing.event_type,
        existing.source_organization_id,
    )
    .await?;

    let now = Utc::now();
    let updated = update_amount_decision(
        conn,
        decision.candidate_id,
        decision.actor_user_id,
        decision.target_status.as_str(),
        decision.approved_amount.clone(),
        decision.decision_reason.as_deref(),
        now,
    )
    .await
    .map_err(map_reward_amount_decision_error)?;

    create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id: Some(decision.actor_user_id),
            event_type: RewardAuditEventType::AmountDecision.as_str().to_string(),
            from_status: Some(from_status),
            to_status: updated.status.clone(),
            reason: decision.decision_reason,
            metadata: json!({
                "approved_amount": updated.approved_amount.as_ref().map(ToString::to_string),
            }),
        },
    )
    .await
    .map_err(map_reward_amount_decision_error)?;

    if decision.target_status == RewardCandidateStatus::AmountApproved {
        enqueue_reward_execution_job(conn, decision.candidate_id)
            .await
            .map_err(map_reward_amount_decision_error)?;
    }

    map_reward_amount_decision_candidate(updated)
}
