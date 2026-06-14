use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_fraud_dashboard::store::PlatformFraudDashboardStore;
use crate::application::reporting::platform_fraud_dashboard::{
    platform_fraud_dashboard_from_facts, FraudBlockDashboardFact, PlatformFraudDashboardError,
    PlatformFraudDashboardOutput,
};
use crate::db::schema::reward_fraud_blocks;
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

            Ok(platform_fraud_dashboard_from_facts(
                active_blocks.into_iter().map(fraud_block_fact).collect(),
            ))
        }
        .boxed()
    }
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

fn map_diesel_error(error: diesel::result::Error) -> PlatformFraudDashboardError {
    PlatformFraudDashboardError::Database(error.to_string())
}
