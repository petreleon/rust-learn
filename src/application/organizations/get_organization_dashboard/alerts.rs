use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardAlertOutput, OrganizationDashboardCourseSummaryOutput,
    OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardRewardSummaryOutput,
    OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardWalletSummaryOutput,
};

pub fn organization_dashboard_alerts(
    organization_id: i32,
    courses: &OrganizationDashboardCourseSummaryOutput,
    teacher_applications: &OrganizationDashboardTeacherApplicationSummaryOutput,
    rewards: &OrganizationDashboardRewardSummaryOutput,
    wallet: &OrganizationDashboardWalletSummaryOutput,
    permissions: &OrganizationDashboardOperatorPermissionsOutput,
) -> Vec<OrganizationDashboardAlertOutput> {
    let mut alerts = Vec::new();
    push_teacher_application_alert(&mut alerts, organization_id, teacher_applications);
    push_course_attention_alert(&mut alerts, organization_id, courses);
    push_reward_reconciliation_alert(&mut alerts, organization_id, rewards);
    push_wallet_missing_alert(&mut alerts, wallet, permissions);

    if alerts.is_empty() {
        alerts.push(OrganizationDashboardAlertOutput {
            severity: "info".to_string(),
            kind: "no_attention_items".to_string(),
            message: "No dashboard attention items are visible for this session.".to_string(),
            action_label: None,
            action_href: None,
        });
    }

    alerts
}

fn push_teacher_application_alert(
    alerts: &mut Vec<OrganizationDashboardAlertOutput>,
    organization_id: i32,
    teacher_applications: &OrganizationDashboardTeacherApplicationSummaryOutput,
) {
    if !teacher_applications.available || teacher_applications.submitted == 0 {
        return;
    }

    alerts.push(OrganizationDashboardAlertOutput {
        severity: "warning".to_string(),
        kind: "teacher_applications_submitted".to_string(),
        message: format!(
            "{} sponsored teacher {} awaiting platform review.",
            teacher_applications.submitted,
            if teacher_applications.submitted == 1 {
                "application is"
            } else {
                "applications are"
            }
        ),
        action_label: Some("Open teacher nominations".to_string()),
        action_href: Some(format!(
            "/organizations/{organization_id}/teacher-applications"
        )),
    });
}

fn push_course_attention_alert(
    alerts: &mut Vec<OrganizationDashboardAlertOutput>,
    organization_id: i32,
    courses: &OrganizationDashboardCourseSummaryOutput,
) {
    if !courses.available || courses.needs_changes == 0 {
        return;
    }

    alerts.push(OrganizationDashboardAlertOutput {
        severity: "warning".to_string(),
        kind: "courses_need_changes".to_string(),
        message: format!(
            "{} sponsored course {} marked needs changes.",
            courses.needs_changes,
            if courses.needs_changes == 1 {
                "is"
            } else {
                "are"
            }
        ),
        action_label: Some("Open courses".to_string()),
        action_href: Some(format!("/organizations/{organization_id}/courses")),
    });
}

fn push_reward_reconciliation_alert(
    alerts: &mut Vec<OrganizationDashboardAlertOutput>,
    organization_id: i32,
    rewards: &OrganizationDashboardRewardSummaryOutput,
) {
    let reward_attention_count = rewards.failed_count + rewards.needs_reconciliation_count;
    if !rewards.available || reward_attention_count == 0 {
        return;
    }

    alerts.push(OrganizationDashboardAlertOutput {
        severity: "warning".to_string(),
        kind: "reward_reconciliation".to_string(),
        message: if reward_attention_count == 1 {
            "1 reward record has failed or needs reconciliation.".to_string()
        } else {
            format!("{reward_attention_count} reward records have failed or need reconciliation.")
        },
        action_label: Some("Open reports".to_string()),
        action_href: Some(format!("/organizations/{organization_id}/reports")),
    });
}

fn push_wallet_missing_alert(
    alerts: &mut Vec<OrganizationDashboardAlertOutput>,
    wallet: &OrganizationDashboardWalletSummaryOutput,
    permissions: &OrganizationDashboardOperatorPermissionsOutput,
) {
    if wallet.available
        && wallet.wallet_count == 0
        && (permissions.can_manage_wallets || permissions.can_manage_reward_budget)
    {
        alerts.push(OrganizationDashboardAlertOutput {
            severity: "warning".to_string(),
            kind: "wallet_missing".to_string(),
            message: "No organization wallet is configured for reward budget operations."
                .to_string(),
            action_label: None,
            action_href: None,
        });
    }
}
