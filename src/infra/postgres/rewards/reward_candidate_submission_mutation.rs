use diesel_async::AsyncPgConnection;
use serde_json::json;

use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmission, RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
};
use crate::domain::rewards::candidate::evidence::ensure_reward_evidence_is_eligible;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::ensure_no_active_reward_fraud_block;
use crate::infra::postgres::rewards::reward_candidate_records::find_candidate_by_idempotency_key;
use crate::infra::postgres::rewards::reward_candidate_submission_audit_insert::create_candidate_with_audit;
use crate::infra::postgres::rewards::reward_candidate_submission_eligibility::{
    ensure_no_prior_active_reward_candidate, ensure_reward_target_eligible,
};
use crate::infra::postgres::rewards::reward_candidate_submission_mappers::map_reward_candidate_submission_error;
use crate::infra::postgres::rewards::reward_candidate_submission_validation::{
    normalize_idempotency_key, normalize_reward_event_type,
};
use crate::models::reward_candidate::{NewRewardCandidate, RewardCandidate};

pub(super) async fn submit_reward_candidate(
    conn: &mut AsyncPgConnection,
    submission: RewardCandidateSubmission,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    let event_type = normalize_reward_event_type(&submission.command.event_type)?;
    let idempotency_key = normalize_idempotency_key(
        submission.command.idempotency_key,
        submission.course_id,
        submission.command.student_user_id,
        event_type,
    )?;
    let event_type_string = event_type.as_str().to_string();

    if let Some(existing) = find_candidate_by_idempotency_key(conn, &idempotency_key)
        .await
        .map_err(map_reward_candidate_submission_error)?
    {
        return idempotent_replay_or_conflict(
            existing,
            submission.actor_user_id,
            submission.course_id,
            submission.command.student_user_id,
            &event_type_string,
        );
    }

    let evidence = submission.command.evidence.unwrap_or_else(|| json!({}));
    ensure_no_active_reward_fraud_block(
        conn,
        &[submission.actor_user_id],
        submission.course_id,
        &event_type_string,
        submission.source_organization_id,
    )
    .await?;
    ensure_reward_target_eligible(
        conn,
        submission.command.student_user_id,
        submission.course_id,
        &event_type_string,
    )
    .await?;
    ensure_reward_evidence_is_eligible(event_type, &evidence)?;
    ensure_no_prior_active_reward_candidate(
        conn,
        submission.command.student_user_id,
        submission.course_id,
        &event_type_string,
    )
    .await?;

    let actor_user_id = submission.actor_user_id;
    let created = create_candidate_with_audit(
        conn,
        NewRewardCandidate {
            course_id: submission.course_id,
            student_user_id: submission.command.student_user_id,
            submitter_user_id: actor_user_id,
            source_scope: submission.source_scope,
            source_organization_id: submission.source_organization_id,
            event_type: event_type_string,
            idempotency_key,
            evidence,
            status: RewardCandidateStatus::PendingTeacherApproval
                .as_str()
                .to_string(),
        },
        actor_user_id,
    )
    .await?;

    log::info!(
        "event=reward_candidate_submitted candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?} idempotency_key={}",
        created.id,
        actor_user_id,
        created.student_user_id,
        created.course_id,
        created.status,
        created.event_type,
        created.source_scope,
        created.source_organization_id,
        created.idempotency_key
    );

    Ok(created)
}

fn idempotent_replay_or_conflict(
    existing: RewardCandidate,
    actor_user_id: i32,
    course_id: i32,
    student_user_id: i32,
    event_type: &str,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    if existing.course_id == course_id
        && existing.student_user_id == student_user_id
        && existing.event_type == event_type
    {
        log::info!(
            "event=reward_candidate_idempotent_replay candidate_id={} actor_user_id={} student_user_id={} course_id={} status={} event_type={} source_scope={} source_organization_id={:?} idempotency_key={}",
            existing.id,
            actor_user_id,
            existing.student_user_id,
            existing.course_id,
            existing.status,
            existing.event_type,
            existing.source_scope,
            existing.source_organization_id,
            existing.idempotency_key
        );
        return Ok(existing.into());
    }

    Err(RewardCandidateSubmissionError::InvalidInput(
        "idempotency key is already used by another reward candidate".to_string(),
    ))
}
