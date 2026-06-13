use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardMemberSummaryOutput,
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};

pub fn gated_member_summary() -> OrganizationDashboardMemberSummaryOutput {
    OrganizationDashboardMemberSummaryOutput {
        available: false,
        missing_permissions: vec!["VIEW_ORGANIZATION".to_string()],
        total: 0,
        verified_email_count: 0,
        kyc_ready_count: 0,
        delegated_permission_count: 0,
    }
}

pub fn gated_course_summary() -> OrganizationDashboardCourseSummaryOutput {
    OrganizationDashboardCourseSummaryOutput {
        available: false,
        missing_permissions: vec!["VIEW_ORGANIZATION".to_string()],
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
            "VIEW_ORG_TEACHER_APPLICATIONS".to_string(),
            "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW".to_string(),
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
        missing_permissions: vec!["VIEW_ORG_REWARD_REPORTS".to_string()],
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
            "MANAGE_ORG_WALLETS".to_string(),
            "MANAGE_ORG_REWARD_BUDGET".to_string(),
            "VIEW_ORG_REWARD_REPORTS".to_string(),
        ],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}
