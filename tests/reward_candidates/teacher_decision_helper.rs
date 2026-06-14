use rust_learn::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionCommand as TeacherRewardCandidateDecisionRequest,
    TeacherRewardCandidateDecisionError, TeacherRewardCandidateDecisionOutput,
    TeacherRewardCandidateDecisionUseCase,
};
use rust_learn::infra::postgres::rewards::teacher_reward_candidate_decision_use_case::PostgresTeacherRewardCandidateDecisionUseCase;

async fn decide_reward_candidate_by_teacher(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    candidate_id: i64,
    request: TeacherRewardCandidateDecisionRequest,
) -> Result<TeacherRewardCandidateDecisionOutput, RewardCandidateError> {
    let pool = establish_connection();
    PostgresTeacherRewardCandidateDecisionUseCase::new(pool)
        .decide_teacher_reward_candidate(actor_user_id, course_id, candidate_id, request)
        .await
        .map_err(map_teacher_reward_candidate_decision_error)
}

fn map_teacher_reward_candidate_decision_error(
    error: TeacherRewardCandidateDecisionError,
) -> RewardCandidateError {
    match error {
        TeacherRewardCandidateDecisionError::PermissionDenied(permission) => {
            RewardCandidateError::PermissionDenied(permission)
        }
        TeacherRewardCandidateDecisionError::InvalidStatus(message) => {
            RewardCandidateError::InvalidStatus(message)
        }
        TeacherRewardCandidateDecisionError::NotFound => RewardCandidateError::NotFound,
        TeacherRewardCandidateDecisionError::Connection(message)
        | TeacherRewardCandidateDecisionError::Database(message) => {
            RewardCandidateError::Database(message)
        }
    }
}
