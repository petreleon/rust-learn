use futures::future::BoxFuture;

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError,
    OrganizationDashboardMemberSummaryOutput, OrganizationDashboardOperatorPermissionsOutput,
    OrganizationDashboardOrganizationOutput, OrganizationDashboardRewardSummaryOutput,
    OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardWalletSummaryOutput,
};

pub trait OrganizationDashboardStore {
    fn load_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOrganizationOutput, OrganizationDashboardError>>;

    fn can_view_dashboard(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationDashboardError>>;

    fn load_operator_permissions(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardError>,
    >;

    fn load_member_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardMemberSummaryOutput, OrganizationDashboardError>>;

    fn load_course_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError>>;

    fn load_teacher_application_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardError>,
    >;

    fn load_reward_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardRewardSummaryOutput, OrganizationDashboardError>>;

    fn load_wallet_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardWalletSummaryOutput, OrganizationDashboardError>>;
}
