use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardMemberSummaryOutput,
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};
use crate::domain::access_control::permissions::Permissions;

pub fn gated_member_summary() -> OrganizationDashboardMemberSummaryOutput {
    OrganizationDashboardMemberSummaryOutput {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.into()],
        total: 0,
        verified_email_count: 0,
        kyc_ready_count: 0,
        delegated_permission_count: 0,
    }
}

pub fn gated_course_summary() -> OrganizationDashboardCourseSummaryOutput {
    OrganizationDashboardCourseSummaryOutput {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.into()],
        total: 0,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}

pub fn gated_teacher_application_summary() -> OrganizationDashboardTeacherApplicationSummaryOutput {
    OrganizationDashboardTeacherApplicationSummaryOutput {
        available: false,
        missing_permissions: vec![
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS.into(),
            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.into(),
        ],
        total: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        rejected: 0,
    }
}

pub fn gated_reward_summary() -> OrganizationDashboardRewardSummaryOutput {
    OrganizationDashboardRewardSummaryOutput {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORG_REWARD_REPORTS.into()],
        reward_candidate_count: 0,
        approved_reward_count: 0,
        approved_amount_total: "0".to_string(),
        failed_count: 0,
        needs_reconciliation_count: 0,
    }
}

pub fn gated_wallet_summary() -> OrganizationDashboardWalletSummaryOutput {
    OrganizationDashboardWalletSummaryOutput {
        available: false,
        missing_permissions: vec![
            Permissions::MANAGE_ORG_WALLETS.into(),
            Permissions::MANAGE_ORG_REWARD_BUDGET.into(),
            Permissions::VIEW_ORG_REWARD_REPORTS.into(),
        ],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}
