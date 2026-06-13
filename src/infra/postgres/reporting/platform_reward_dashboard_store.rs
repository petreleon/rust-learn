use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_reward_dashboard::store::PlatformRewardDashboardStore;
use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardOutput,
};
use crate::infra::postgres::reporting::platform_reward_dashboard_reconciliation::reward_reconciliation_mismatches;
use crate::infra::postgres::reporting::platform_reward_dashboard_rows::{
    payout_failures, pending_amount_approvals,
};
use crate::infra::postgres::reporting::platform_reward_dashboard_summaries::{
    reward_candidate_dashboard_summary, teacher_application_dashboard_summary,
};

pub struct PostgresPlatformRewardDashboardStore<'a> {
    conn: &'a mut diesel_async::AsyncPgConnection,
}

impl<'a> PostgresPlatformRewardDashboardStore<'a> {
    pub fn new(conn: &'a mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformRewardDashboardStore for PostgresPlatformRewardDashboardStore<'_> {
    fn load_platform_reward_dashboard(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>> {
        async move {
            let teacher_applications = teacher_application_dashboard_summary(self.conn).await?;
            let reward_candidates = reward_candidate_dashboard_summary(self.conn).await?;
            let (pending_amount_approvals, pending_amount_approval_count) =
                pending_amount_approvals(self.conn).await?;
            let (payout_failures, payout_failure_count) = payout_failures(self.conn).await?;
            let reconciliation_mismatches = reward_reconciliation_mismatches(self.conn).await?;
            let reconciliation_mismatch_count = reconciliation_mismatches.len() as i64;

            Ok(PlatformRewardDashboardOutput {
                teacher_applications,
                reward_candidates,
                pending_amount_approval_count,
                pending_amount_approvals,
                payout_failure_count,
                payout_failures,
                reconciliation_mismatch_count,
                reconciliation_mismatches,
            })
        }
        .boxed()
    }
}
