use futures::executor::block_on;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, test_support::FakeRewardAuthorizationStore, RewardAuthorizationAction,
};
use crate::domain::access_control::permissions::Permissions;

#[test]
fn execute_reward_payout_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::EXECUTE_REWARD_PAYOUT],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ExecuteRewardPayout,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![Permissions::EXECUTE_REWARD_PAYOUT]
    );
}

#[test]
fn approve_reward_amount_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::APPROVE_REWARD_AMOUNT],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ApproveRewardAmount,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![Permissions::APPROVE_REWARD_AMOUNT]
    );
}

#[test]
fn manage_reward_policy_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::SET_REWARD_POLICY],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ManageRewardPolicy,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permissions::SET_REWARD_POLICY]);
}

#[test]
fn record_reward_compensation_accepts_any_wallet_reconciliation_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::MANAGE_WALLETS],
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
        vec![Permissions::RECONCILE_WALLETS, Permissions::MANAGE_WALLETS]
    );
}

#[test]
fn view_reward_audit_checks_exact_platform_permission() {
    let mut store = FakeRewardAuthorizationStore {
        platform_permissions: vec![Permissions::VIEW_REWARD_AUDIT],
        ..Default::default()
    };

    let allowed = block_on(authorize_reward_action(
        &mut store,
        7,
        RewardAuthorizationAction::ViewRewardAudit,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permissions::VIEW_REWARD_AUDIT]);
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
