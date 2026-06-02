use crate::db::schema::reward_fraud_blocks;
use crate::models::reward_fraud_block::{NewRewardFraudBlock, RewardFraudBlock};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    new_block: NewRewardFraudBlock,
) -> QueryResult<RewardFraudBlock> {
    diesel::insert_into(reward_fraud_blocks::table)
        .values(&new_block)
        .get_result(conn)
        .await
}

pub async fn find_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    block_id: i64,
) -> QueryResult<RewardFraudBlock> {
    reward_fraud_blocks::table.find(block_id).first(conn).await
}

pub async fn revoke_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    block_id: i64,
    revoked_by_user_id: i32,
    revoked_at: DateTime<Utc>,
) -> QueryResult<RewardFraudBlock> {
    diesel::update(reward_fraud_blocks::table.find(block_id))
        .set((
            reward_fraud_blocks::revoked_by_user_id.eq(Some(revoked_by_user_id)),
            reward_fraud_blocks::revoked_at.eq(Some(revoked_at)),
            reward_fraud_blocks::updated_at.eq(revoked_at),
        ))
        .get_result(conn)
        .await
}
