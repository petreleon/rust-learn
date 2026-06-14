use crate::config::constants::permissions::Permissions;

use super::super::dashboard_types_and_reads::{
    OrganizationDashboardOperatorPermissions, OrganizationDashboardRewardSummary,
    OrganizationDashboardWalletSummary,
};
use super::super::organization_dashboard_reward_summary::{
    gated_course_summary, gated_member_summary,
};
use super::super::support::{
    OrganizationDashboardCourseSummary, OrganizationDashboardTeacherApplicationSummary,
};

// ── gated summaries ──

#[test]
fn gated_member_summary_has_correct_permission() {
    let summary = gated_member_summary();
    assert!(!summary.available);
    assert_eq!(
        summary.missing_permissions,
        vec![Permissions::VIEW_ORGANIZATION.to_string()]
    );
    assert_eq!(summary.total, 0);
}

#[test]
fn gated_course_summary_has_correct_permission() {
    let summary = gated_course_summary();
    assert!(!summary.available);
    assert_eq!(
        summary.missing_permissions,
        vec![Permissions::VIEW_ORGANIZATION.to_string()]
    );
}

// ── organization_dashboard_alerts ──

pub(super) fn course_summary(
    available: bool,
    needs_changes: i64,
) -> OrganizationDashboardCourseSummary {
    OrganizationDashboardCourseSummary {
        available,
        missing_permissions: if available {
            vec![]
        } else {
            vec!["test".into()]
        },
        total: 10,
        draft: 0,
        submitted: 0,
        needs_changes,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}
pub(super) fn ta_summary(
    available: bool,
    submitted: i64,
) -> OrganizationDashboardTeacherApplicationSummary {
    OrganizationDashboardTeacherApplicationSummary {
        available,
        missing_permissions: if available {
            vec![]
        } else {
            vec!["test".into()]
        },
        submitted,
        approved: 0,
        rejected: 0,
        needs_changes: 0,
        total: 0,
    }
}

pub(super) fn reward_summary(
    available: bool,
    failed: i64,
    needs_reconciliation: i64,
) -> OrganizationDashboardRewardSummary {
    OrganizationDashboardRewardSummary {
        available,
        missing_permissions: if available {
            vec![]
        } else {
            vec!["test".into()]
        },
        reward_candidate_count: 0,
        approved_reward_count: 0,
        approved_amount_total: "0".into(),
        failed_count: failed,
        needs_reconciliation_count: needs_reconciliation,
    }
}

pub(super) fn wallet_summary(available: bool) -> OrganizationDashboardWalletSummary {
    OrganizationDashboardWalletSummary {
        available,
        missing_permissions: if available {
            vec![]
        } else {
            vec!["test".into()]
        },
        wallet_count: if available { 1 } else { 0 },
        balance_total: "0".into(),
    }
}

pub(super) fn operator_permissions() -> OrganizationDashboardOperatorPermissions {
    OrganizationDashboardOperatorPermissions {
        can_view_dashboard: true,
        can_view_members: true,
        can_view_courses: true,
        can_view_reports: true,
        can_view_teacher_applications: true,
        can_nominate_teachers: true,
        can_manage_wallets: true,
        can_manage_reward_budget: true,
    }
}
