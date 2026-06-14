use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_fraud_dashboard::store::PlatformFraudDashboardStore;
use crate::application::reporting::platform_fraud_dashboard::{
    FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput, PlatformFraudDashboardError,
    PlatformFraudDashboardOutput,
};
use crate::db::schema::reward_fraud_blocks;
use crate::domain::rewards::fraud_block::RewardFraudBlockScope;
use crate::models::reward_fraud_block::RewardFraudBlock;

pub struct PostgresPlatformFraudDashboardStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresPlatformFraudDashboardStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformFraudDashboardStore for PostgresPlatformFraudDashboardStore<'_> {
    fn load_platform_fraud_dashboard(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>> {
        async move {
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
                .load::<RewardFraudBlock>(self.conn)
                .await
                .map_err(map_diesel_error)?;

            let active_by_scope = scope_summary(&active_blocks);
            Ok(PlatformFraudDashboardOutput {
                active_total: active_blocks.len() as i64,
                active_by_scope,
                active_blocks: active_blocks.into_iter().map(map_fraud_block).collect(),
            })
        }
        .boxed()
    }
}

fn scope_summary(blocks: &[RewardFraudBlock]) -> FraudBlockScopeSummaryOutput {
    let mut summary = FraudBlockScopeSummaryOutput::default();
    for block in blocks {
        match RewardFraudBlockScope::parse(&block.scope_type) {
            Ok(RewardFraudBlockScope::Teacher) => summary.teacher += 1,
            Ok(RewardFraudBlockScope::Organization) => summary.organization += 1,
            Ok(RewardFraudBlockScope::Course) => summary.course += 1,
            Ok(RewardFraudBlockScope::RewardPolicy) => summary.reward_policy += 1,
            Err(_) => {}
        }
    }
    summary
}

fn map_fraud_block(block: RewardFraudBlock) -> FraudBlockDashboardRowOutput {
    FraudBlockDashboardRowOutput {
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

fn map_diesel_error(error: diesel::result::Error) -> PlatformFraudDashboardError {
    PlatformFraudDashboardError::Database(error.to_string())
}
