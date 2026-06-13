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
