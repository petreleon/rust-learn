use chrono::NaiveDate;
use futures::future::BoxFuture;

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
};

pub trait OrganizationRewardDashboardUseCase: Send + Sync {
    fn load_organization_reward_dashboard(
        &self,
        organization_id: i32,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> BoxFuture<'_, Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>>;
}
