use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::json;

use crate::application::learning::assessment::{
    AssessmentRewardHandoff, AssessmentRewardHandoffOutput,
};
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;
use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmission, RewardCandidateSubmissionError, SubmitRewardCandidateCommand,
};
use crate::domain::rewards::candidate::event_type::{
    RewardEventType, REWARD_EVENT_ASSESSMENT_COMPLETION,
};
use crate::domain::rewards::candidate::lifecycle;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::rewards::reward_candidate_policy_lookup::active_reward_policy_ids_for_course_event;
use crate::infra::postgres::rewards::reward_candidate_records::find_candidate_by_idempotency_key;
use crate::infra::postgres::rewards::reward_candidate_submission_mutation::submit_reward_candidate;
use crate::infra::postgres::schema::reward_candidates;

pub async fn create_assessment_reward_handoff(
    conn: &mut AsyncPgConnection,
    handoff: AssessmentRewardHandoff,
) -> Result<AssessmentRewardHandoffOutput, AssessmentSubmissionError> {
    let Some(policy_id) = active_assessment_policy_id(conn, handoff.course_id).await? else {
        return Ok(AssessmentRewardHandoffOutput::missing_policy());
    };

    let idempotency_key = reward_idempotency_key(&handoff);
    if let Some(existing) = find_candidate_by_idempotency_key(conn, &idempotency_key)
        .await
        .map_err(map_handoff_load_error)?
    {
        return Ok(AssessmentRewardHandoffOutput::already_exists(
            Some(existing.id),
            Some(policy_id),
        ));
    }

    let submission = RewardCandidateSubmission {
        actor_user_id: handoff.user_id,
        course_id: handoff.course_id,
        source_scope: RewardCandidateSourceScope::Course,
        source_organization_id: None,
        command: SubmitRewardCandidateCommand {
            student_user_id: handoff.user_id,
            event_type: RewardEventType::AssessmentCompletion,
            idempotency_key: Some(idempotency_key),
            evidence: Some(reward_evidence(&handoff)),
        },
    };

    match submit_reward_candidate(conn, submission).await {
        Ok(candidate) => Ok(AssessmentRewardHandoffOutput::created(
            candidate.id,
            policy_id,
        )),
        Err(RewardCandidateSubmissionError::InvalidInput(message))
            if message.contains("active reward candidate already exists") =>
        {
            let existing = find_existing_assessment_candidate(conn, &handoff).await?;
            Ok(AssessmentRewardHandoffOutput::already_exists(
                existing.map(|candidate| candidate.id),
                Some(policy_id),
            ))
        }
        Err(error) => Ok(AssessmentRewardHandoffOutput::failed(format!(
            "Reward handoff failed: {}",
            reward_error_message(error)
        ))),
    }
}

async fn active_assessment_policy_id(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Option<i64>, AssessmentSubmissionError> {
    active_reward_policy_ids_for_course_event(conn, course_id, REWARD_EVENT_ASSESSMENT_COMPLETION)
        .await
        .map(|policy_ids| policy_ids.into_iter().next())
        .map_err(map_handoff_load_error)
}

async fn find_existing_assessment_candidate(
    conn: &mut AsyncPgConnection,
    handoff: &AssessmentRewardHandoff,
) -> Result<Option<RewardCandidate>, AssessmentSubmissionError> {
    let reusable = lifecycle::prior_candidate_statuses_allowing_new_submission();
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(handoff.course_id))
        .filter(reward_candidates::student_user_id.eq(handoff.user_id))
        .filter(reward_candidates::event_type.eq(REWARD_EVENT_ASSESSMENT_COMPLETION))
        .filter(reward_candidates::status.ne(reusable[0].as_str()))
        .filter(reward_candidates::status.ne(reusable[1].as_str()))
        .filter(reward_candidates::status.ne(reusable[2].as_str()))
        .order(reward_candidates::created_at.desc())
        .first::<RewardCandidate>(conn)
        .await
        .optional()
        .map_err(map_handoff_load_error)
}

fn reward_idempotency_key(handoff: &AssessmentRewardHandoff) -> String {
    format!(
        "assessment-completion:{}:{}:{}",
        handoff.course_id, handoff.assessment_id, handoff.user_id
    )
}

fn reward_evidence(handoff: &AssessmentRewardHandoff) -> serde_json::Value {
    json!({
        "assessment_id": handoff.assessment_id,
        "attempt_id": handoff.attempt_id,
        "passing_score": handoff.percentage,
        "required_passing_score": handoff.passing_score,
        "score": handoff.score,
        "total_points": handoff.total_points
    })
}

fn map_handoff_load_error(error: diesel::result::Error) -> AssessmentSubmissionError {
    AssessmentSubmissionError::LoadFailed(error.to_string())
}

fn reward_error_message(error: RewardCandidateSubmissionError) -> String {
    match error {
        RewardCandidateSubmissionError::PermissionDenied(message)
        | RewardCandidateSubmissionError::InvalidInput(message)
        | RewardCandidateSubmissionError::InvalidStatus(message)
        | RewardCandidateSubmissionError::Connection(message)
        | RewardCandidateSubmissionError::Database(message) => message,
        RewardCandidateSubmissionError::NotFound => {
            "reward candidate dependency was not found".to_string()
        }
    }
}
