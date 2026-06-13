use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::rewards::list_reward_history::{
    StudentRewardHistoryEntry, StudentRewardHistoryError, StudentRewardHistoryQuery,
    StudentRewardHistoryUseCase,
};
use rust_learn::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, ListRewardFraudBlocksOutput, ListRewardFraudBlocksQuery,
    RewardFraudBlockAuditEventOutput, RewardFraudBlockError, RewardFraudBlockOutput,
    RewardFraudBlockUseCase,
};

struct RouteOnlyRewardFraudBlockUseCase;
struct RouteOnlyStudentRewardHistoryUseCase;

pub fn reward_fraud_block_data() -> web::Data<Arc<dyn RewardFraudBlockUseCase>> {
    web::Data::new(Arc::new(RouteOnlyRewardFraudBlockUseCase) as Arc<dyn RewardFraudBlockUseCase>)
}

pub fn student_reward_history_data() -> web::Data<Arc<dyn StudentRewardHistoryUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyStudentRewardHistoryUseCase) as Arc<dyn StudentRewardHistoryUseCase>
    )
}

impl RewardFraudBlockUseCase for RouteOnlyRewardFraudBlockUseCase {
    fn create_reward_fraud_block(
        &self,
        _actor_user_id: i32,
        _command: CreateRewardFraudBlockCommand,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        reward_fraud_block_error()
    }

    fn list_reward_fraud_blocks(
        &self,
        _actor_user_id: i32,
        _query: ListRewardFraudBlocksQuery,
    ) -> BoxFuture<'_, Result<ListRewardFraudBlocksOutput, RewardFraudBlockError>> {
        reward_fraud_block_error()
    }

    fn revoke_reward_fraud_block(
        &self,
        _actor_user_id: i32,
        _block_id: i64,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        reward_fraud_block_error()
    }

    fn reward_fraud_block_audit_history(
        &self,
        _actor_user_id: i32,
        _block_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardFraudBlockAuditEventOutput>, RewardFraudBlockError>> {
        reward_fraud_block_error()
    }
}

impl StudentRewardHistoryUseCase for RouteOnlyStudentRewardHistoryUseCase {
    fn list_student_reward_history(
        &self,
        _actor_user_id: i32,
        _query: StudentRewardHistoryQuery,
    ) -> BoxFuture<'_, Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError>> {
        ready(Err(StudentRewardHistoryError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}

fn reward_fraud_block_error<T: Send + 'static>(
) -> BoxFuture<'static, Result<T, RewardFraudBlockError>> {
    ready(Err(RewardFraudBlockError::Database(
        "route-only use case".to_string(),
    )))
    .boxed()
}
