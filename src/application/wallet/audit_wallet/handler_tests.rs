use futures::executor::block_on;

use super::{audit_wallet, audit_wallet_for_actor};
use crate::application::wallet::audit_wallet::{
    test_support::FakeWalletAuditStore, WalletAuditError, WalletAuditSubject, WalletAuditTarget,
};

#[test]
fn delegates_to_wallet_audit_store() {
    let mut store = FakeWalletAuditStore::default();
    let target = WalletAuditTarget {
        id: 10,
        user_id: Some(7),
        organization_id: None,
        value: "100".to_string(),
    };

    let audit = block_on(audit_wallet(&mut store, target)).expect("wallet audit should load");

    assert!(store.loaded);
    assert_eq!(audit.wallet.id, 10);
    assert_eq!(audit.wallet.owner_type, "user");
}

#[test]
fn loads_own_wallet_audit_without_permission_check() {
    let mut store = FakeWalletAuditStore::default();

    let audit = block_on(audit_wallet_for_actor(
        &mut store,
        7,
        WalletAuditSubject::OwnUser,
    ))
    .expect("own audit should load");

    assert_eq!(audit.wallet.user_id, Some(7));
    assert!(!store.checked_user_permission);
    assert!(store.checked_user_exists);
}

#[test]
fn denies_other_user_before_existence_check_without_permission() {
    let mut store = FakeWalletAuditStore {
        user_permission: false,
        ..Default::default()
    };

    let error = block_on(audit_wallet_for_actor(
        &mut store,
        7,
        WalletAuditSubject::User(8),
    ))
    .expect_err("other user audit should be gated");

    assert_eq!(error, WalletAuditError::UserPermissionDenied);
    assert!(store.checked_user_permission);
    assert!(!store.checked_user_exists);
}

#[test]
fn checks_organization_exists_before_permission() {
    let mut store = FakeWalletAuditStore {
        organization_exists: false,
        ..Default::default()
    };

    let error = block_on(audit_wallet_for_actor(
        &mut store,
        7,
        WalletAuditSubject::Organization(9),
    ))
    .expect_err("missing organization should 404 first");

    assert_eq!(error, WalletAuditError::OrganizationNotFound);
    assert!(store.checked_organization_exists);
    assert!(!store.checked_organization_permission);
}
