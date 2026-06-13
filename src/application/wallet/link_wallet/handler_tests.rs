use futures::executor::block_on;

use super::handler::link_wallet;
use crate::application::wallet::link_wallet::{
    test_support::FakeWalletLinkStore, WalletLinkError, WalletLinkSubject,
};

#[test]
fn links_own_wallet_after_kyc_without_permission_check() {
    let mut store = FakeWalletLinkStore::default();

    let linked = block_on(link_wallet(&mut store, 7, WalletLinkSubject::OwnUser))
        .expect("own wallet should link");

    assert_eq!(linked.wallet.user_id, Some(7));
    assert!(linked.created);
    assert!(!store.checked_user_permission);
    assert!(store.checked_user_exists);
    assert!(store.checked_user_kyc);
    assert!(store.linked_user_wallet);
}

#[test]
fn denies_other_user_before_existence_check_without_permission() {
    let mut store = FakeWalletLinkStore {
        user_permission: false,
        ..Default::default()
    };

    let error = block_on(link_wallet(&mut store, 7, WalletLinkSubject::User(8)))
        .expect_err("other user link should be gated");

    assert_eq!(error, WalletLinkError::UserPermissionDenied);
    assert!(store.checked_user_permission);
    assert!(!store.checked_user_exists);
    assert!(!store.checked_user_kyc);
    assert!(!store.linked_user_wallet);
}

#[test]
fn requires_kyc_before_linking_user_wallet() {
    let mut store = FakeWalletLinkStore {
        user_kyc_verified: false,
        ..Default::default()
    };

    let error = block_on(link_wallet(&mut store, 7, WalletLinkSubject::OwnUser))
        .expect_err("unverified user should not link");

    assert_eq!(error, WalletLinkError::KycRequired);
    assert!(store.checked_user_exists);
    assert!(store.checked_user_kyc);
    assert!(!store.linked_user_wallet);
}

#[test]
fn checks_organization_exists_before_permission() {
    let mut store = FakeWalletLinkStore {
        organization_exists: false,
        ..Default::default()
    };

    let error = block_on(link_wallet(
        &mut store,
        7,
        WalletLinkSubject::Organization(9),
    ))
    .expect_err("missing organization should 404 first");

    assert_eq!(error, WalletLinkError::OrganizationNotFound);
    assert!(store.checked_organization_exists);
    assert!(!store.checked_organization_permission);
    assert!(!store.linked_organization_wallet);
}
