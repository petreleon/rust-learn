use crate::application::reporting::platform_reward_dashboard::store::PlatformRewardDashboardStore;
use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardOutput,
};

pub async fn load_platform_reward_dashboard(
    store: &mut impl PlatformRewardDashboardStore,
) -> Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError> {
    store.load_platform_reward_dashboard().await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_platform_reward_dashboard;
    use crate::application::reporting::platform_reward_dashboard::store::PlatformRewardDashboardStore;
    use crate::application::reporting::platform_reward_dashboard::{
        PlatformRewardDashboardError, PlatformRewardDashboardOutput,
        RewardCandidateDashboardSummaryOutput, TeacherApplicationDashboardSummaryOutput,
    };

    #[test]
    fn loads_platform_reward_dashboard_through_store_port() {
        let mut store = FakePlatformRewardDashboardStore { called: false };

        let output = block_on(load_platform_reward_dashboard(&mut store))
            .expect("platform reward dashboard should load");

        assert!(store.called);
        assert_eq!(output.pending_amount_approval_count, 2);
        assert_eq!(output.teacher_applications.submitted, 3);
    }

    struct FakePlatformRewardDashboardStore {
        called: bool,
    }

    impl PlatformRewardDashboardStore for FakePlatformRewardDashboardStore {
        fn load_platform_reward_dashboard(
            &mut self,
        ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>>
        {
            self.called = true;
            ready(Ok(PlatformRewardDashboardOutput {
                teacher_applications: TeacherApplicationDashboardSummaryOutput {
                    submitted: 3,
                    ..Default::default()
                },
                reward_candidates: RewardCandidateDashboardSummaryOutput::default(),
                pending_amount_approval_count: 2,
                pending_amount_approvals: vec![],
                payout_failure_count: 0,
                payout_failures: vec![],
                reconciliation_mismatch_count: 0,
                reconciliation_mismatches: vec![],
            }))
            .boxed()
        }
    }
}
