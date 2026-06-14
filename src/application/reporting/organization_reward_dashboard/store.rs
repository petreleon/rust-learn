use chrono::NaiveDate;
use futures::future::BoxFuture;

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
};

pub trait OrganizationRewardDashboardStore {
    fn load_organization_reward_dashboard(
        &mut self,
        organization_id: i32,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> BoxFuture<'_, Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>>;
}
