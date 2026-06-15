use super::*;

#[test]
fn platform_notifications_target_reward_audit_or_fraud_managers() {
    assert_eq!(
        platform_reward_fraud_block_notification_permissions(),
        [
            Permissions::VIEW_REWARD_AUDIT,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS
        ]
    );
}

#[test]
fn organization_notifications_target_org_reward_report_or_budget_managers() {
    assert_eq!(
        organization_reward_fraud_block_notification_permissions(),
        [
            Permissions::VIEW_ORG_REWARD_REPORTS,
            Permissions::MANAGE_ORG_REWARD_BUDGET
        ]
    );
}
