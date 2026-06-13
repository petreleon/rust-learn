use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permission::Permission;

#[test]
fn manage_teacher_fraud_block_accepts_teacher_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ManageRewardFraudBlocks],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageTeacherRewardFraudBlock,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![
            Permission::BlockRewardTeacher,
            Permission::ManageRewardFraudBlocks
        ]
    );
}

#[test]
fn manage_organization_fraud_block_accepts_org_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::BlockRewardOrganization],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageOrganizationRewardFraudBlock,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![Permission::BlockRewardOrganization]
    );
}

#[test]
fn manage_general_fraud_block_checks_exact_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ManageRewardFraudBlocks],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageRewardFraudBlock,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![Permission::ManageRewardFraudBlocks]
    );
}

#[test]
fn view_fraud_blocks_accepts_audit_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ManageRewardFraudBlocks],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ViewRewardFraudBlocks,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![
            Permission::ViewRewardAudit,
            Permission::ManageRewardFraudBlocks
        ]
    );
}
