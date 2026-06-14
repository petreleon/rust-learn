use super::*;

#[test]
fn platform_notifications_target_reward_audit_or_fraud_managers() {
    assert_eq!(
        platform_reward_fraud_block_notification_permissions(),
        [
            Permission::ViewRewardAudit,
            Permission::ManageRewardFraudBlocks
        ]
    );
}

#[test]
fn organization_notifications_target_org_reward_report_or_budget_managers() {
    assert_eq!(
        organization_reward_fraud_block_notification_permissions(),
        [
            Permission::ViewOrgRewardReports,
            Permission::ManageOrgRewardBudget
        ]
    );
}
