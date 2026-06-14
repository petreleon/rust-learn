use super::super::dashboard_types_and_reads::OrganizationDashboardWalletSummary;
use super::super::organization_dashboard_alerts::organization_dashboard_alerts;
use super::gated_member_summary_has_correct_permission::{
    course_summary, operator_permissions, reward_summary, ta_summary, wallet_summary,
};

#[test]
fn no_alerts_when_everything_clean() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(true, 0),
        &ta_summary(true, 0),
        &reward_summary(true, 0, 0),
        &wallet_summary(true),
        &operator_permissions(),
    );
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].kind, "no_attention_items");
}

#[test]
fn alert_when_submitted_teacher_applications() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(true, 0),
        &ta_summary(true, 3),
        &reward_summary(true, 0, 0),
        &wallet_summary(true),
        &operator_permissions(),
    );
    assert!(alerts
        .iter()
        .any(|a| a.kind == "teacher_applications_submitted"));
}

#[test]
fn alert_when_courses_need_changes() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(true, 2),
        &ta_summary(true, 0),
        &reward_summary(true, 0, 0),
        &wallet_summary(true),
        &operator_permissions(),
    );
    assert!(alerts.iter().any(|a| a.kind == "courses_need_changes"));
}

#[test]
fn alert_when_reward_reconciliation_needed() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(true, 0),
        &ta_summary(true, 0),
        &reward_summary(true, 1, 2),
        &wallet_summary(true),
        &operator_permissions(),
    );
    assert!(alerts.iter().any(|a| a.kind == "reward_reconciliation"));
}

#[test]
fn no_alerts_when_sections_are_gated() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(false, 5),
        &ta_summary(false, 3),
        &reward_summary(false, 2, 1),
        &wallet_summary(false),
        &operator_permissions(),
    );
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].kind, "no_attention_items");
}

#[test]
fn alert_when_wallet_missing() {
    let alerts = organization_dashboard_alerts(
        1,
        &course_summary(true, 0),
        &ta_summary(true, 0),
        &reward_summary(true, 0, 0),
        &OrganizationDashboardWalletSummary {
            available: true,
            missing_permissions: vec![],
            wallet_count: 0,
            balance_total: "0".into(),
        },
        &operator_permissions(),
    );
    assert!(alerts.iter().any(|a| a.kind == "wallet_missing"));
}
