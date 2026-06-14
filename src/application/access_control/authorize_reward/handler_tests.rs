use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permission::Permission;

#[test]
fn execute_reward_payout_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ExecuteRewardPayout],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ExecuteRewardPayout,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permission::ExecuteRewardPayout]);
}

#[test]
fn approve_reward_amount_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ApproveRewardAmount],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ApproveRewardAmount,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permission::ApproveRewardAmount]);
}

#[test]
fn manage_reward_policy_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::SetRewardPolicy],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageRewardPolicy,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permission::SetRewardPolicy]);
}

#[test]
fn record_reward_compensation_accepts_any_wallet_reconciliation_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ManageWallets],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::RecordRewardCompensation,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![Permission::ReconcileWallets, Permission::ManageWallets]
    );
}

#[test]
fn view_reward_audit_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permission::ViewRewardAudit],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ViewRewardAudit,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permission::ViewRewardAudit]);
}

#[test]
fn execute_reward_payout_denies_without_permission() {
    let mut store = FakeRewardAuthorizationStore::default();

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ExecuteRewardPayout,
    ))
    .unwrap();

    assert!(!allowed);
}
