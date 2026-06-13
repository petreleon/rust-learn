use rust_learn::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
    RewardCandidateSubmissionUseCase,
    SubmitRewardCandidateCommand as SubmitRewardCandidateRequest,
};
use rust_learn::infra::postgres::rewards::reward_candidate_submission_use_case::PostgresRewardCandidateSubmissionUseCase;

async fn submit_course_reward_candidate(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateError> {
    let pool = establish_connection();
    PostgresRewardCandidateSubmissionUseCase::new(pool)
        .submit_course_reward_candidate(actor_user_id, course_id, request)
        .await
        .map_err(map_reward_candidate_submission_error)
}

async fn submit_organization_reward_candidate(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    course_id: i32,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateError> {
    let pool = establish_connection();
    PostgresRewardCandidateSubmissionUseCase::new(pool)
        .submit_organization_reward_candidate(actor_user_id, organization_id, course_id, request)
        .await
        .map_err(map_reward_candidate_submission_error)
}

fn map_reward_candidate_submission_error(
    error: RewardCandidateSubmissionError,
) -> RewardCandidateError {
    match error {
        RewardCandidateSubmissionError::PermissionDenied(permission) => {
            RewardCandidateError::PermissionDenied(permission)
        }
        RewardCandidateSubmissionError::InvalidInput(message) => {
            RewardCandidateError::InvalidInput(message)
        }
        RewardCandidateSubmissionError::InvalidStatus(message) => {
            RewardCandidateError::InvalidStatus(message)
        }
        RewardCandidateSubmissionError::NotFound => RewardCandidateError::NotFound,
        RewardCandidateSubmissionError::Connection(message)
        | RewardCandidateSubmissionError::Database(message) => {
            RewardCandidateError::Database(message)
        }
    }
}
