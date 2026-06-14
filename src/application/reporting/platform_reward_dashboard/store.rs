use futures::future::BoxFuture;

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardOutput,
};

pub trait PlatformRewardDashboardStore {
    fn load_platform_reward_dashboard(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>>;
}
