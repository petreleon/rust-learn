use rust_learn::application::rewards::decide_amount::{
    RewardAmountDecisionCommand as RewardAmountDecisionRequest, RewardAmountDecisionError,
    RewardAmountDecisionOutput, RewardAmountDecisionUseCase,
};
use rust_learn::infra::postgres::rewards::reward_amount_decision_use_case::PostgresRewardAmountDecisionUseCase;

async fn decide_reward_amount(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardAmountDecisionRequest,
) -> Result<RewardAmountDecisionOutput, RewardCandidateError> {
    let pool = establish_connection();
    PostgresRewardAmountDecisionUseCase::new(pool)
        .decide_reward_amount(actor_user_id, candidate_id, request)
        .await
        .map_err(map_reward_amount_decision_error)
}

fn map_reward_amount_decision_error(error: RewardAmountDecisionError) -> RewardCandidateError {
    match error {
        RewardAmountDecisionError::PermissionDenied(permission) => {
            RewardCandidateError::PermissionDenied(permission)
        }
        RewardAmountDecisionError::InvalidInput(message) => {
            RewardCandidateError::InvalidInput(message)
        }
        RewardAmountDecisionError::InvalidStatus(message) => {
            RewardCandidateError::InvalidStatus(message)
        }
        RewardAmountDecisionError::NotFound => RewardCandidateError::NotFound,
        RewardAmountDecisionError::Connection(message)
        | RewardAmountDecisionError::Database(message) => RewardCandidateError::Database(message),
    }
}
