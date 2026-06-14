use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};

pub fn teacher_application_summary() -> OrganizationDashboardTeacherApplicationSummaryOutput {
    OrganizationDashboardTeacherApplicationSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: 1,
        submitted: 1,
        needs_changes: 0,
        approved: 0,
        rejected: 0,
    }
}

pub fn reward_summary() -> OrganizationDashboardRewardSummaryOutput {
    OrganizationDashboardRewardSummaryOutput {
        available: true,
        missing_permissions: vec![],
        reward_candidate_count: 2,
        approved_reward_count: 1,
        approved_amount_total: "10".to_string(),
        failed_count: 1,
        needs_reconciliation_count: 0,
    }
}

pub fn wallet_summary() -> OrganizationDashboardWalletSummaryOutput {
    OrganizationDashboardWalletSummaryOutput {
        available: true,
        missing_permissions: vec![],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}
