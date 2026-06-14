use futures::executor::block_on;

use crate::application::access_control::authorize_wallet::{
    authorize_wallet_action, test_support::FakeWalletAuthorizationStore, WalletAuthorizationAction,
};
use crate::domain::access_control::permission::Permission;

#[test]
fn view_user_wallet_accepts_any_platform_wallet_view_permission() {
    let mut store = FakeWalletAuthorizationStore {
        platform_permissions: vec![Permission::ReconcileWallets],
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
            Permission::ViewWallet,
            Permission::ViewTransactions,
            Permission::ReconcileWallets
        ]
    );
}

#[test]
fn view_organization_wallet_falls_back_to_organization_permissions() {
    let mut store = FakeWalletAuthorizationStore {
        organization_permissions: vec![(42, Permission::ViewOrgRewardReports)],
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
            (42, Permission::ManageOrgWallets),
            (42, Permission::ViewOrgRewardReports)
        ]
    );
}

#[test]
fn link_organization_wallet_accepts_platform_override() {
    let mut store = FakeWalletAuthorizationStore {
        platform_permissions: vec![Permission::CreateWallet],
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
        platform_permissions: vec![Permission::SetRetireTax],
        ..Default::default()
    };

    let allowed = block_on(authorize_wallet_action(
        &mut store,
        7,
        WalletAuthorizationAction::SetRetireTax,
    ))
    .unwrap();

    assert!(allowed);
    assert_eq!(store.platform_checks, vec![Permission::SetRetireTax]);
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
