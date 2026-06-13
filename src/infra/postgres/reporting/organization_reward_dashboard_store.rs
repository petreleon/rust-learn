use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_reward_dashboard::store::OrganizationRewardDashboardStore;
use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
};
use crate::db::schema::organizations;
use crate::infra::postgres::reporting::organization_reward_dashboard_queries::{
    course_reward_rows, map_diesel_error, sponsored_teacher_application_summary,
    wallet_balance_rows,
};

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
            let organization_name = organizations::table
                .find(organization_id)
                .select(organizations::name)
                .first::<String>(self.conn)
                .await
                .map_err(map_diesel_error)?;

            let sponsored_teacher_applications =
                sponsored_teacher_application_summary(self.conn, organization_id).await?;
            let course_data = course_reward_rows(self.conn, organization_id, from, to).await?;
            let wallet_data = wallet_balance_rows(self.conn, organization_id).await?;

            let course_reward_count = course_data
                .iter()
                .map(|data| data.row.reward_candidate_count)
                .sum::<i64>();
            let approved_reward_count = course_data
                .iter()
                .map(|data| data.row.approved_reward_count)
                .sum::<i64>();
            let approved_amount_total =
                course_data.iter().fold(BigDecimal::from(0), |sum, data| {
                    sum + data.approved_amount_total.clone()
                });
            let wallet_balance_total = wallet_data
                .iter()
                .fold(BigDecimal::from(0), |sum, data| sum + data.balance.clone());

            Ok(OrganizationRewardDashboardOutput {
                organization_id,
                organization_name,
                sponsored_teacher_applications,
                course_reward_count,
                approved_reward_count,
                approved_amount_total: approved_amount_total.to_string(),
                courses: course_data.into_iter().map(|data| data.row).collect(),
                wallets: wallet_data.into_iter().map(|data| data.row).collect(),
                wallet_balance_total: wallet_balance_total.to_string(),
            })
        }
        .boxed()
    }
}
