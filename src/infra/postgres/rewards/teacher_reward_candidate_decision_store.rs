use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};
use serde_json::json;

use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecision, TeacherRewardCandidateDecisionError,
    TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionStore,
};
use crate::config::constants::permissions::Permissions;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::candidate::transition;
use crate::infra::postgres::rewards::teacher_reward_candidate_decision_fraud_blocks::ensure_no_active_reward_fraud_block;
use crate::infra::postgres::rewards::teacher_reward_candidate_decision_mappers::map_teacher_decision_error;
use crate::models::reward_audit_event::{NewRewardAuditEvent, REWARD_AUDIT_EVENT_TEACHER_DECISION};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::{reward_audit_event_repository, reward_candidate_repository};

pub struct PostgresTeacherRewardCandidateDecisionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherRewardCandidateDecisionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherRewardCandidateDecisionStore for PostgresTeacherRewardCandidateDecisionStore<'_> {
    fn can_approve_student_reward_candidate(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherRewardCandidateDecisionError>> {
        async move {
            user_permission_course_request(
                self.conn,
                actor_user_id,
                course_id,
                &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
            )
            .await
            .map_err(map_teacher_decision_error)
        }
        .boxed()
    }

    fn decide_teacher_reward_candidate(
        &mut self,
        decision: TeacherRewardCandidateDecision,
    ) -> BoxFuture<
        '_,
        Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError>,
    > {
        async move {
            let actor_user_id = decision.actor_user_id;
            let updated = self
                .conn
                .transaction::<_, TeacherDecisionTransactionError, _>(|conn| {
                    Box::pin(async move {
                        apply_teacher_decision(conn, decision)
                            .await
                            .map_err(TeacherDecisionTransactionError::Application)
                    })
                })
                .await
                .map_err(TeacherRewardCandidateDecisionError::from)?;

            log::info!(
                "event=reward_candidate_teacher_decision candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?}",
                updated.id,
                actor_user_id,
                updated.student_user_id,
                updated.course_id,
                updated.status,
                updated.event_type,
                updated.source_scope,
                updated.source_organization_id
            );

            Ok(updated)
        }
        .boxed()
    }
}

enum TeacherDecisionTransactionError {
    Application(TeacherRewardCandidateDecisionError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for TeacherDecisionTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<TeacherDecisionTransactionError> for TeacherRewardCandidateDecisionError {
    fn from(error: TeacherDecisionTransactionError) -> Self {
        match error {
            TeacherDecisionTransactionError::Application(error) => error,
            TeacherDecisionTransactionError::Diesel(error) => map_teacher_decision_error(error),
        }
    }
}

async fn apply_teacher_decision(
    conn: &mut AsyncPgConnection,
    decision: TeacherRewardCandidateDecision,
) -> Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError> {
    let existing = reward_candidate_repository::find_candidate(conn, decision.candidate_id)
        .await
        .map_err(map_teacher_decision_error)?;
    if existing.course_id != decision.course_id {
        return Err(TeacherRewardCandidateDecisionError::NotFound);
    }

    if existing.status == decision.target_status.as_str() {
        return Ok(existing.into());
    }

    ensure_teacher_transition(&existing.status, decision.target_status)?;
    ensure_no_active_reward_fraud_block(
        conn,
        &[decision.actor_user_id],
        existing.course_id,
        &existing.event_type,
        existing.source_organization_id,
    )
    .await?;

    let from_status = existing.status;
    let now = Utc::now();
    let updated = reward_candidate_repository::update_teacher_decision(
        conn,
        decision.candidate_id,
        decision.actor_user_id,
        decision.target_status.as_str(),
        decision.decision_reason.as_deref(),
        now,
    )
    .await
    .map_err(map_teacher_decision_error)?;

    reward_audit_event_repository::create_reward_audit_event(
        conn,
        NewRewardAuditEvent {
            reward_candidate_id: updated.id,
            actor_user_id: Some(decision.actor_user_id),
            event_type: REWARD_AUDIT_EVENT_TEACHER_DECISION.to_string(),
            from_status: Some(from_status),
            to_status: updated.status.clone(),
            reason: decision.decision_reason,
            metadata: json!({}),
        },
    )
    .await
    .map_err(map_teacher_decision_error)?;

    Ok(updated.into())
}

fn ensure_teacher_transition(
    current_status: &str,
    target_status: RewardCandidateStatus,
) -> Result<(), TeacherRewardCandidateDecisionError> {
    let current_status = RewardCandidateStatus::parse(current_status).map_err(|_| {
        TeacherRewardCandidateDecisionError::InvalidStatus(
            "reward candidate has already left teacher approval".to_string(),
        )
    })?;
    let approved = target_status == RewardCandidateStatus::TeacherApproved;
    transition::teacher_decision(current_status, approved).map_err(|_| {
        TeacherRewardCandidateDecisionError::InvalidStatus(
            "reward candidate has already left teacher approval".to_string(),
        )
    })?;
    Ok(())
}
