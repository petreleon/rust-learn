use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_fraud_dashboard::{
    FraudBlockDashboardFact, PlatformFraudDashboardError,
};
use crate::db::schema::reward_fraud_blocks;
use crate::infra::postgres::reporting::platform_fraud_dashboard_mappers::map_diesel_error;
use crate::models::reward_fraud_block::RewardFraudBlock;

pub(super) async fn active_fraud_block_facts(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<FraudBlockDashboardFact>, PlatformFraudDashboardError> {
    let now = chrono::Utc::now();
    let active_blocks = reward_fraud_blocks::table
        .filter(reward_fraud_blocks::revoked_at.is_null())
        .filter(
            reward_fraud_blocks::expires_at
                .is_null()
                .or(reward_fraud_blocks::expires_at.gt(now)),
        )
        .order(reward_fraud_blocks::created_at.desc())
        .limit(100)
        .load::<RewardFraudBlock>(conn)
        .await
        .map_err(map_diesel_error)?;

    Ok(active_blocks.into_iter().map(fraud_block_fact).collect())
}

fn fraud_block_fact(block: RewardFraudBlock) -> FraudBlockDashboardFact {
    FraudBlockDashboardFact {
        id: block.id,
        scope_type: block.scope_type,
        teacher_user_id: block.teacher_user_id,
        organization_id: block.organization_id,
        course_id: block.course_id,
        reward_policy_id: block.reward_policy_id,
        reason: block.reason,
        evidence_reference: block.evidence_reference,
        created_by_user_id: block.created_by_user_id,
        expires_at: block.expires_at,
        created_at: block.created_at,
        updated_at: block.updated_at,
    }
}
