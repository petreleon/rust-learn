use rust_learn::application::rewards::plan_payout::{
    RewardPayoutPlan, RewardPayoutPlanError, RewardPayoutPlanUseCase,
};
use rust_learn::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationError, RewardTokenConfirmationOutput as RewardTokenConfirmationResult,
    RewardTokenConfirmationUseCase,
};
use rust_learn::infra::postgres::rewards::reward_payout_plan_use_case::PostgresRewardPayoutPlanUseCase;
use rust_learn::infra::postgres::rewards::reward_token_confirmation_use_case::PostgresRewardTokenConfirmationUseCase;

async fn plan_reward_payout(
    _conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardExecutionError> {
    let pool = establish_connection();
    PostgresRewardPayoutPlanUseCase::new(pool)
        .plan_reward_payout(candidate_id)
        .await
        .map_err(map_reward_payout_plan_error)
}

fn map_reward_payout_plan_error(error: RewardPayoutPlanError) -> RewardExecutionError {
    match error {
        RewardPayoutPlanError::PermissionDenied(permission) => {
            RewardExecutionError::PermissionDenied(permission)
        }
        RewardPayoutPlanError::InvalidStatus(message) => {
            RewardExecutionError::InvalidStatus(message)
        }
        RewardPayoutPlanError::InvalidInput(message) => RewardExecutionError::InvalidInput(message),
        RewardPayoutPlanError::NoActivePolicy => RewardExecutionError::NoActivePolicy,
        RewardPayoutPlanError::Connection(message) | RewardPayoutPlanError::Database(message) => {
            RewardExecutionError::Database(message)
        }
    }
}

async fn record_reward_token_confirmation(
    _conn: &mut AsyncPgConnection,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    let pool = establish_connection();
    PostgresRewardTokenConfirmationUseCase::new(pool)
        .record_reward_token_confirmation(candidate_id, request)
        .await
        .map_err(map_reward_token_confirmation_error)
}

async fn record_reward_token_confirmation_for_actor(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardTokenConfirmationRequest,
) -> Result<RewardTokenConfirmationResult, RewardExecutionError> {
    let pool = establish_connection();
    PostgresRewardTokenConfirmationUseCase::new(pool)
        .record_reward_token_confirmation_for_actor(actor_user_id, candidate_id, request)
        .await
        .map_err(map_reward_token_confirmation_error)
}

fn map_reward_token_confirmation_error(
    error: RewardTokenConfirmationError,
) -> RewardExecutionError {
    match error {
        RewardTokenConfirmationError::PermissionDenied(permission) => {
            RewardExecutionError::PermissionDenied(permission)
        }
        RewardTokenConfirmationError::InvalidStatus(message) => {
            RewardExecutionError::InvalidStatus(message)
        }
        RewardTokenConfirmationError::InvalidInput(message) => {
            RewardExecutionError::InvalidInput(message)
        }
        RewardTokenConfirmationError::NotFound => RewardExecutionError::NoActivePolicy,
        RewardTokenConfirmationError::Connection(message)
        | RewardTokenConfirmationError::Database(message) => {
            RewardExecutionError::Database(message)
        }
    }
}
