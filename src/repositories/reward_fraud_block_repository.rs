use crate::db::schema::reward_fraud_blocks;
use crate::models::reward_fraud_block::{NewRewardFraudBlock, RewardFraudBlock};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug, Default)]
pub struct RewardFraudBlockFilter {
    pub scope_type: Option<String>,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

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

pub async fn list_reward_fraud_blocks(
    conn: &mut AsyncPgConnection,
    filter: RewardFraudBlockFilter,
) -> QueryResult<(Vec<RewardFraudBlock>, i64)> {
    let now = Utc::now();
    let limit = filter.limit.unwrap_or(100).clamp(1, 500);
    let offset = filter.offset.unwrap_or(0).max(0);

    let mut count_query = reward_fraud_blocks::table.into_boxed();
    if let Some(scope_type) = &filter.scope_type {
        count_query = count_query.filter(reward_fraud_blocks::scope_type.eq(scope_type));
    }
    if let Some(teacher_user_id) = filter.teacher_user_id {
        count_query = count_query.filter(reward_fraud_blocks::teacher_user_id.eq(Some(teacher_user_id)));
    }
    if let Some(organization_id) = filter.organization_id {
        count_query = count_query.filter(reward_fraud_blocks::organization_id.eq(Some(organization_id)));
    }
    if let Some(course_id) = filter.course_id {
        count_query = count_query.filter(reward_fraud_blocks::course_id.eq(Some(course_id)));
    }
    if let Some(reward_policy_id) = filter.reward_policy_id {
        count_query = count_query.filter(reward_fraud_blocks::reward_policy_id.eq(Some(reward_policy_id)));
    }
    if let Some(active) = filter.active {
        if active {
            count_query = count_query
                .filter(reward_fraud_blocks::revoked_at.is_null())
                .filter(
                    reward_fraud_blocks::expires_at
                        .is_null()
                        .or(reward_fraud_blocks::expires_at.gt(now)),
                );
        } else {
            count_query = count_query.filter(
                reward_fraud_blocks::revoked_at
                    .is_not_null()
                    .or(reward_fraud_blocks::expires_at.le(now)),
            );
        }
    }
    let total = count_query.count().get_result::<i64>(conn).await?;

    let mut query = reward_fraud_blocks::table.into_boxed();
    if let Some(scope_type) = &filter.scope_type {
        query = query.filter(reward_fraud_blocks::scope_type.eq(scope_type));
    }
    if let Some(teacher_user_id) = filter.teacher_user_id {
        query = query.filter(reward_fraud_blocks::teacher_user_id.eq(Some(teacher_user_id)));
    }
    if let Some(organization_id) = filter.organization_id {
        query = query.filter(reward_fraud_blocks::organization_id.eq(Some(organization_id)));
    }
    if let Some(course_id) = filter.course_id {
        query = query.filter(reward_fraud_blocks::course_id.eq(Some(course_id)));
    }
    if let Some(reward_policy_id) = filter.reward_policy_id {
        query = query.filter(reward_fraud_blocks::reward_policy_id.eq(Some(reward_policy_id)));
    }
    if let Some(active) = filter.active {
        if active {
            query = query
                .filter(reward_fraud_blocks::revoked_at.is_null())
                .filter(
                    reward_fraud_blocks::expires_at
                        .is_null()
                        .or(reward_fraud_blocks::expires_at.gt(now)),
                );
        } else {
            query = query.filter(
                reward_fraud_blocks::revoked_at
                    .is_not_null()
                    .or(reward_fraud_blocks::expires_at.le(now)),
            );
        }
    }

    let blocks = query
        .order(reward_fraud_blocks::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load(conn)
        .await?;
    Ok((blocks, total))
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
