use futures::executor::block_on;

use super::handler::read_wallet;
use crate::application::wallet::read_wallet::{
    test_support::FakeWalletReadStore, WalletReadError, WalletReadSubject,
};

#[test]
fn reads_own_wallet_without_permission_check() {
    let mut store = FakeWalletReadStore::default();

    let wallet = block_on(read_wallet(&mut store, 7, WalletReadSubject::OwnUser))
        .expect("own wallet should load");

    assert_eq!(wallet.user_id, Some(7));
    assert!(!store.checked_user_permission);
    assert!(store.checked_user_exists);
}

#[test]
fn denies_other_user_before_existence_check_without_permission() {
    let mut store = FakeWalletReadStore {
        user_permission: false,
        ..Default::default()
    };

    let error = block_on(read_wallet(&mut store, 7, WalletReadSubject::User(8)))
        .expect_err("other user read should be gated");

    assert_eq!(error, WalletReadError::UserPermissionDenied);
    assert!(store.checked_user_permission);
    assert!(!store.checked_user_exists);
}

#[test]
fn checks_organization_exists_before_permission() {
    let mut store = FakeWalletReadStore {
        organization_exists: false,
        ..Default::default()
    };

    let error = block_on(read_wallet(
        &mut store,
        7,
        WalletReadSubject::Organization(9),
    ))
    .expect_err("missing organization should 404 first");

    assert_eq!(error, WalletReadError::OrganizationNotFound);
    assert!(store.checked_organization_exists);
    assert!(!store.checked_organization_permission);
}
