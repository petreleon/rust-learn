use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardAlertOutput, OrganizationDashboardCourseSummaryOutput,
    OrganizationDashboardHealthOutput, OrganizationDashboardMemberSummaryOutput,
    OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardOrganizationOutput,
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};

use super::dashboard_summary_dto::{
    OrganizationDashboardAlertResponse, OrganizationDashboardCourseSummaryResponse,
    OrganizationDashboardHealthResponse, OrganizationDashboardMemberSummaryResponse,
    OrganizationDashboardOperatorPermissionsResponse, OrganizationDashboardOrganizationResponse,
    OrganizationDashboardRewardSummaryResponse,
    OrganizationDashboardTeacherApplicationSummaryResponse,
    OrganizationDashboardWalletSummaryResponse,
};

impl From<OrganizationDashboardOrganizationOutput> for OrganizationDashboardOrganizationResponse {
    fn from(organization: OrganizationDashboardOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<OrganizationDashboardHealthOutput> for OrganizationDashboardHealthResponse {
    fn from(health: OrganizationDashboardHealthOutput) -> Self {
        Self {
            status: health.status,
            alert_count: health.alert_count,
        }
    }
}

impl From<OrganizationDashboardMemberSummaryOutput> for OrganizationDashboardMemberSummaryResponse {
    fn from(summary: OrganizationDashboardMemberSummaryOutput) -> Self {
        Self {
            available: summary.available,
            missing_permissions: summary.missing_permissions,
            total: summary.total,
            verified_email_count: summary.verified_email_count,
            kyc_ready_count: summary.kyc_ready_count,
            delegated_permission_count: summary.delegated_permission_count,
        }
    }
}

impl From<OrganizationDashboardCourseSummaryOutput> for OrganizationDashboardCourseSummaryResponse {
    fn from(summary: OrganizationDashboardCourseSummaryOutput) -> Self {
        Self {
            available: summary.available,
            missing_permissions: summary.missing_permissions,
            total: summary.total,
            draft: summary.draft,
            submitted: summary.submitted,
            needs_changes: summary.needs_changes,
            approved: summary.approved,
            published: summary.published,
            suspended: summary.suspended,
            archived: summary.archived,
        }
    }
}

impl From<OrganizationDashboardTeacherApplicationSummaryOutput>
    for OrganizationDashboardTeacherApplicationSummaryResponse
{
    fn from(summary: OrganizationDashboardTeacherApplicationSummaryOutput) -> Self {
        Self {
            available: summary.available,
            missing_permissions: summary.missing_permissions,
            total: summary.total,
            submitted: summary.submitted,
            needs_changes: summary.needs_changes,
            approved: summary.approved,
            rejected: summary.rejected,
        }
    }
}

impl From<OrganizationDashboardRewardSummaryOutput> for OrganizationDashboardRewardSummaryResponse {
    fn from(summary: OrganizationDashboardRewardSummaryOutput) -> Self {
        Self {
            available: summary.available,
            missing_permissions: summary.missing_permissions,
            reward_candidate_count: summary.reward_candidate_count,
            approved_reward_count: summary.approved_reward_count,
            approved_amount_total: summary.approved_amount_total,
            failed_count: summary.failed_count,
            needs_reconciliation_count: summary.needs_reconciliation_count,
        }
    }
}

impl From<OrganizationDashboardWalletSummaryOutput> for OrganizationDashboardWalletSummaryResponse {
    fn from(summary: OrganizationDashboardWalletSummaryOutput) -> Self {
        Self {
            available: summary.available,
            missing_permissions: summary.missing_permissions,
            wallet_count: summary.wallet_count,
            balance_total: summary.balance_total,
        }
    }
}

impl From<OrganizationDashboardOperatorPermissionsOutput>
    for OrganizationDashboardOperatorPermissionsResponse
{
    fn from(permissions: OrganizationDashboardOperatorPermissionsOutput) -> Self {
        Self {
            can_view_dashboard: permissions.can_view_dashboard,
            can_view_members: permissions.can_view_members,
            can_view_courses: permissions.can_view_courses,
            can_view_reports: permissions.can_view_reports,
            can_view_teacher_applications: permissions.can_view_teacher_applications,
            can_nominate_teachers: permissions.can_nominate_teachers,
            can_manage_wallets: permissions.can_manage_wallets,
            can_manage_reward_budget: permissions.can_manage_reward_budget,
        }
    }
}

impl From<OrganizationDashboardAlertOutput> for OrganizationDashboardAlertResponse {
    fn from(alert: OrganizationDashboardAlertOutput) -> Self {
        Self {
            severity: alert.severity,
            kind: alert.kind,
            message: alert.message,
            action_label: alert.action_label,
            action_href: alert.action_href,
        }
    }
}
