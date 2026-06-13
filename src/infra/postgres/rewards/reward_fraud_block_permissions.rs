use diesel_async::AsyncPgConnection;

use crate::application::rewards::manage_fraud_block::RewardFraudBlockError;
use crate::config::constants::permissions::Permissions;
use crate::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::infra::postgres::rewards::reward_fraud_block_mappers::map_reward_fraud_block_error;
use crate::repositories::platform_repository::user_permission_platform_request;

pub(super) async fn can_manage_fraud_block_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    scope_type: &str,
) -> Result<bool, RewardFraudBlockError> {
    let permissions = required_permissions_for_scope(scope_type)?;
    for permission in permissions.iter() {
        let allowed =
            user_permission_platform_request(conn, actor_user_id, &permission.to_string())
                .await
                .map_err(map_reward_fraud_block_error)?;
        if allowed {
            return Ok(true);
        }
    }

    Ok(false)
}

pub(super) async fn can_view_fraud_blocks(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardFraudBlockError> {
    for permission in [
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
    ] {
        let allowed =
            user_permission_platform_request(conn, actor_user_id, &permission.to_string())
                .await
                .map_err(map_reward_fraud_block_error)?;
        if allowed {
            return Ok(true);
        }
    }

    Ok(false)
}

fn required_permissions_for_scope(
    scope_type: &str,
) -> Result<Vec<Permissions>, RewardFraudBlockError> {
    match scope_type {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(vec![
            Permissions::BLOCK_REWARD_TEACHER,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => Ok(vec![
            Permissions::BLOCK_REWARD_ORGANIZATION,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_COURSE | REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS])
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
