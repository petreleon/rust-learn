use serde::Serialize;

use crate::application::organizations::get_organization_dashboard::OrganizationDashboardOutput;

use super::dashboard_summary_dto::{
    OrganizationDashboardAlertResponse, OrganizationDashboardCourseSummaryResponse,
    OrganizationDashboardHealthResponse, OrganizationDashboardMemberSummaryResponse,
    OrganizationDashboardOperatorPermissionsResponse, OrganizationDashboardOrganizationResponse,
    OrganizationDashboardRewardSummaryResponse,
    OrganizationDashboardTeacherApplicationSummaryResponse,
    OrganizationDashboardWalletSummaryResponse,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardResponse {
    pub organization: OrganizationDashboardOrganizationResponse,
    pub health: OrganizationDashboardHealthResponse,
    pub members: OrganizationDashboardMemberSummaryResponse,
    pub courses: OrganizationDashboardCourseSummaryResponse,
    pub teacher_applications: OrganizationDashboardTeacherApplicationSummaryResponse,
    pub rewards: OrganizationDashboardRewardSummaryResponse,
    pub wallet: OrganizationDashboardWalletSummaryResponse,
    pub operator_permissions: OrganizationDashboardOperatorPermissionsResponse,
    pub alerts: Vec<OrganizationDashboardAlertResponse>,
}

impl From<OrganizationDashboardOutput> for OrganizationDashboardResponse {
    fn from(output: OrganizationDashboardOutput) -> Self {
        Self {
            organization: output.organization.into(),
            health: output.health.into(),
            members: output.members.into(),
            courses: output.courses.into(),
            teacher_applications: output.teacher_applications.into(),
            rewards: output.rewards.into(),
            wallet: output.wallet.into(),
            operator_permissions: output.operator_permissions.into(),
            alerts: output.alerts.into_iter().map(Into::into).collect(),
        }
    }
}
