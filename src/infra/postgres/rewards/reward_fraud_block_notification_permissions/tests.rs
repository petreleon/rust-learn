use super::*;
use crate::config::constants::permissions::Permissions;

#[test]
fn platform_fraud_permissions_are_correct() {
    let permissions = platform_fraud_notification_permissions();
    assert_eq!(permissions.len(), 2);
    assert!(permissions.contains(&Permissions::VIEW_REWARD_AUDIT.to_string()));
}

#[test]
fn organization_fraud_permissions_are_correct() {
    let permissions = organization_fraud_notification_permissions();
    assert_eq!(permissions.len(), 2);
    assert!(permissions.contains(&Permissions::VIEW_ORG_REWARD_REPORTS.to_string()));
}
