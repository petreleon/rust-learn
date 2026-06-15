use futures::executor::block_on;

use crate::application::access_control::authorize_wallet::{
    authorize_wallet_action, test_support::FakeWalletAuthorizationStore, WalletAuthorizationAction,
};
use crate::domain::access_control::permissions::Permissions;

#[test]
fn view_user_wallet_accepts_any_platform_wallet_view_permission() {
    let mut store = FakeWalletAuthorizationStore {
        platform_permissions: vec![Permissions::RECONCILE_WALLETS],
        ..Default::default()
    };

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::ViewUserWallet,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.platform_checks,
        vec![
            Permissions::VIEW_WALLET,
            Permissions::VIEW_TRANSACTIONS,
            Permissions::RECONCILE_WALLETS
        ]
    );
}

#[test]
fn view_organization_wallet_falls_back_to_organization_permissions() {
    let mut store = FakeWalletAuthorizationStore {
        organization_permissions: vec![(42, Permissions::VIEW_ORG_REWARD_REPORTS)],
        ..Default::default()
    };

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::ViewOrganizationWallet {
            organization_id: 42,
        },
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(
        store.organization_checks,
        vec![
            (42, Permissions::MANAGE_ORG_WALLETS),
            (42, Permissions::VIEW_ORG_REWARD_REPORTS)
        ]
    );
}

#[test]
fn link_organization_wallet_accepts_platform_override() {
    let mut store = FakeWalletAuthorizationStore {
        platform_permissions: vec![Permissions::CREATE_WALLET],
        ..Default::default()
    };

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::LinkOrganizationWallet {
            organization_id: 42,
        },
    ))
    .unwrap();

    assert!(allowed);
    assert!(store.organization_checks.is_empty());
}

#[test]
fn set_retire_tax_checks_exact_platform_permission() {
    let mut store = FakeWalletAuthorizationStore {
        platform_permissions: vec![Permissions::SET_RETIRE_TAX],
        ..Default::default()
    };

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::SetRetireTax,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permissions::SET_RETIRE_TAX]);
}

#[test]
fn link_user_wallet_denies_when_no_permission_matches() {
    let mut store = FakeWalletAuthorizationStore::default();

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::LinkUserWallet,
    ))
    .unwrap();

    assert!(!allowed);
}
