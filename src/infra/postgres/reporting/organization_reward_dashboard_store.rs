use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_reward_dashboard::store::OrganizationRewardDashboardStore;
use crate::application::reporting::organization_reward_dashboard::{
    organization_reward_dashboard_from_facts, OrganizationRewardDashboardError,
    OrganizationRewardDashboardFacts, OrganizationRewardDashboardOutput,
};
use crate::infra::postgres::reporting::organization_reward_dashboard_courses::course_reward_rows;
use crate::infra::postgres::reporting::organization_reward_dashboard_organizations::organization_name;
use crate::infra::postgres::reporting::organization_reward_dashboard_teacher_applications::sponsored_teacher_application_summary;
use crate::infra::postgres::reporting::organization_reward_dashboard_wallets::wallet_balance_rows;

pub struct PostgresOrganizationRewardDashboardStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresOrganizationRewardDashboardStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationRewardDashboardStore for PostgresOrganizationRewardDashboardStore<'_> {
    fn load_organization_reward_dashboard(
        &mut self,
        organization_id: i32,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> BoxFuture<'_, Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>>
    {
        async move {
            let organization_name = organization_name(self.conn, organization_id).await?;
            let sponsored_teacher_applications =
                sponsored_teacher_application_summary(self.conn, organization_id).await?;
            let course_data = course_reward_rows(self.conn, organization_id, from, to).await?;
            let wallet_data = wallet_balance_rows(self.conn, organization_id).await?;

            Ok(organization_reward_dashboard_from_facts(
                OrganizationRewardDashboardFacts {
                    organization_id,
                    organization_name,
                    sponsored_teacher_applications,
                    courses: course_data,
                    wallets: wallet_data,
                },
            ))
        }
        .boxed()
    }
}
