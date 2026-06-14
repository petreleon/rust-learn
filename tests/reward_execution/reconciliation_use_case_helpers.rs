use rust_learn::application::rewards::reconcile_candidate::{
    RewardReconciliationError, RewardReconciliationOutput as RewardReconciliationResult,
    RewardReconciliationUseCase,
};
use rust_learn::infra::postgres::rewards::reward_reconciliation_use_case::PostgresRewardReconciliationUseCase;

async fn reconcile_reward_candidate(
    _conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<RewardReconciliationResult, RewardExecutionError> {
    let pool = establish_connection();
    PostgresRewardReconciliationUseCase::new(pool)
        .reconcile_reward_candidate(candidate_id)
        .await
        .map_err(map_reward_reconciliation_error)
}

fn map_reward_reconciliation_error(error: RewardReconciliationError) -> RewardExecutionError {
    match error {
        RewardReconciliationError::PermissionDenied(permission) => {
            RewardExecutionError::PermissionDenied(permission)
        }
        RewardReconciliationError::InvalidStatus(message) => {
            RewardExecutionError::InvalidStatus(message)
        }
        RewardReconciliationError::InvalidInput(message) => {
            RewardExecutionError::InvalidInput(message)
        }
        RewardReconciliationError::NoActivePolicy | RewardReconciliationError::NotFound => {
            RewardExecutionError::NoActivePolicy
        }
        RewardReconciliationError::Connection(message)
        | RewardReconciliationError::Database(message) => RewardExecutionError::Database(message),
    }
}
