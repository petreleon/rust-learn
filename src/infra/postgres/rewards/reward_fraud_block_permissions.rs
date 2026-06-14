use diesel_async::AsyncPgConnection;

use crate::application::rewards::manage_fraud_block::RewardFraudBlockError;
use crate::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::infra::postgres::rewards::reward_authorization_access;

pub(super) async fn can_manage_fraud_block_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    scope_type: &str,
) -> Result<bool, RewardFraudBlockError> {
    match authorization_for_scope(scope_type)? {
        FraudBlockAuthorization::Teacher => {
            reward_authorization_access::can_manage_teacher_reward_fraud_block(conn, actor_user_id)
                .await
        }
        FraudBlockAuthorization::Organization => {
            reward_authorization_access::can_manage_organization_reward_fraud_block(
                conn,
                actor_user_id,
            )
            .await
        }
        FraudBlockAuthorization::General => {
            reward_authorization_access::can_manage_reward_fraud_block(conn, actor_user_id).await
        }
    }
    .map_err(|error| RewardFraudBlockError::Database(error.to_string()))
}

pub(super) async fn can_view_fraud_blocks(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardFraudBlockError> {
    reward_authorization_access::can_view_reward_fraud_blocks(conn, actor_user_id)
        .await
        .map_err(|error| RewardFraudBlockError::Database(error.to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FraudBlockAuthorization {
    Teacher,
    Organization,
    General,
}

fn authorization_for_scope(
    scope_type: &str,
) -> Result<FraudBlockAuthorization, RewardFraudBlockError> {
    match scope_type {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(FraudBlockAuthorization::Teacher),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => Ok(FraudBlockAuthorization::Organization),
        REWARD_FRAUD_BLOCK_SCOPE_COURSE | REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(FraudBlockAuthorization::General)
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
