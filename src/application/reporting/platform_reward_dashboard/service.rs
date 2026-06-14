use futures::future::BoxFuture;

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardOutput,
};

pub trait PlatformRewardDashboardUseCase: Send + Sync {
    fn load_platform_reward_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>>;
}
