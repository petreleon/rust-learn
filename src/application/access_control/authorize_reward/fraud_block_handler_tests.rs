use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permissions::Permissions;

#[test]
fn manage_teacher_fraud_block_accepts_teacher_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS],
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
            Permissions::BLOCK_REWARD_TEACHER,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS
        ]
    );
}

#[test]
fn manage_organization_fraud_block_accepts_org_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::BLOCK_REWARD_ORGANIZATION],
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
        vec![Permissions::BLOCK_REWARD_ORGANIZATION]
    );
}

#[test]
fn manage_general_fraud_block_checks_exact_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS],
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
        vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS]
    );
}

#[test]
fn view_fraud_blocks_accepts_audit_or_general_fraud_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS],
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
            Permissions::VIEW_REWARD_AUDIT,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS
        ]
    );
}
