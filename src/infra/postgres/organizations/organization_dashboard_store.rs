use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError,
    OrganizationDashboardMemberSummaryOutput, OrganizationDashboardOperatorPermissionsOutput,
    OrganizationDashboardOrganizationOutput, OrganizationDashboardRewardSummaryOutput,
    OrganizationDashboardStore, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};
use crate::db::schema::organizations;
use crate::infra::postgres::organizations::{
    organization_dashboard_course_summary, organization_dashboard_mappers,
    organization_dashboard_member_summary, organization_dashboard_permissions,
    organization_dashboard_reward_summary, organization_dashboard_teacher_summary,
};

pub struct PostgresOrganizationDashboardStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationDashboardStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationDashboardStore for PostgresOrganizationDashboardStore<'_> {
    fn load_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOrganizationOutput, OrganizationDashboardError>>
    {
        async move {
            organizations::table
                .find(organization_id)
                .select((organizations::id, organizations::name))
                .first::<(i32, String)>(self.conn)
                .await
                .map(|(id, name)| OrganizationDashboardOrganizationOutput { id, name })
                .map_err(organization_dashboard_mappers::map_dashboard_error)
        }
        .boxed()
    }

    fn can_view_dashboard(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationDashboardError>> {
        async move {
            organization_dashboard_permissions::can_view_dashboard(
                self.conn,
                actor_user_id,
                organization_id,
            )
            .await
        }
        .boxed()
    }

    fn load_operator_permissions(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardError>,
    > {
        async move {
            organization_dashboard_permissions::load_operator_permissions(
                self.conn,
                actor_user_id,
                organization_id,
            )
            .await
        }
        .boxed()
    }

    fn load_member_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardMemberSummaryOutput, OrganizationDashboardError>>
    {
        async move {
            organization_dashboard_member_summary::load_member_summary(self.conn, organization_id)
                .await
        }
        .boxed()
    }

    fn load_course_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError>>
    {
        async move {
            organization_dashboard_course_summary::load_course_summary(self.conn, organization_id)
                .await
        }
        .boxed()
    }

    fn load_teacher_application_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardError>,
    > {
        async move {
            organization_dashboard_teacher_summary::load_teacher_application_summary(
                self.conn,
                organization_id,
            )
            .await
        }
        .boxed()
    }

    fn load_reward_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardRewardSummaryOutput, OrganizationDashboardError>>
    {
        async move {
            organization_dashboard_reward_summary::load_reward_summary(self.conn, organization_id)
                .await
        }
        .boxed()
    }

    fn load_wallet_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardWalletSummaryOutput, OrganizationDashboardError>>
    {
        async move {
            organization_dashboard_reward_summary::load_wallet_summary(self.conn, organization_id)
                .await
        }
        .boxed()
    }
}
