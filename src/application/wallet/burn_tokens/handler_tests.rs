use bigdecimal::BigDecimal;
use futures::executor::block_on;

use crate::application::wallet::burn_tokens::{
    load_organization_token_burn_permissions, request_token_burn, test_support::FakeTokenBurnStore,
    TokenBurnCommand, TokenBurnError, TokenBurnSubject,
};
use crate::domain::wallet::burn::{
    TokenBurnFeePath, TokenBurnSource, TOKEN_BURN_STATUS_DEPOSIT_PENDING,
};

#[test]
fn centralized_user_burn_records_ledger_ready_draft() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        ..Default::default()
    };

    block_on(request_token_burn(
        &mut store,
        7,
        TokenBurnSubject::OwnUser,
        burn_command("centralized_wallet", "none"),
    ))
    .unwrap();

    let draft = store.created_draft.expect("draft");
    assert_eq!(draft.user_id, Some(7));
    assert_eq!(draft.source, TokenBurnSource::CentralizedWallet);
    assert_eq!(draft.fee_path, TokenBurnFeePath::None);
    assert!(!draft.metamask_required);
}

#[test]
fn rejects_mismatched_fee_path() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        ..Default::default()
    };

    let error = block_on(request_token_burn(
        &mut store,
        7,
        TokenBurnSubject::OwnUser,
        burn_command("decentralized_direct", "platform_deposit_fee"),
    ))
    .expect_err("fee path should fail");

    assert!(matches!(error, TokenBurnError::InvalidInput(_)));
    assert!(store.created_draft.is_none());
}

#[test]
fn organization_burn_requires_permission_and_keeps_actor_evidence() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        organization_exists: true,
        can_burn_organization: true,
        ..Default::default()
    };

    block_on(request_token_burn(
        &mut store,
        7,
        TokenBurnSubject::Organization(3),
        burn_command("centralized_wallet", "none"),
    ))
    .unwrap();

    let draft = store.created_draft.expect("draft");
    assert_eq!(draft.organization_id, Some(3));
    assert_eq!(draft.actor_user_id, 7);
    assert_eq!(
        draft.permission_evidence.as_deref(),
        Some("BURN_ORGANIZATION_TOKENS")
    );
}

#[test]
fn platform_mediated_without_burn_tx_stays_deposit_pending() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        deposit_tax: BigDecimal::from(2),
        ..Default::default()
    };
    let mut command = burn_command("decentralized_platform_mediated", "platform_deposit_fee");
    command.deposit_intent_id = Some(99);
    command.transaction_hash = None;
    command.log_index = None;

    block_on(request_token_burn(
        &mut store,
        7,
        TokenBurnSubject::OwnUser,
        command,
    ))
    .unwrap();

    let draft = store.created_draft.expect("draft");
    assert_eq!(draft.status.as_str(), TOKEN_BURN_STATUS_DEPOSIT_PENDING);
    assert_eq!(draft.fee_amount, BigDecimal::from(2));
}

#[test]
fn organization_burn_permissions_include_kyc_and_permission_state() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        organization_exists: true,
        can_burn_organization: true,
        ..Default::default()
    };

    let permissions =
        block_on(load_organization_token_burn_permissions(&mut store, 7, 3)).expect("permissions");

    assert_eq!(permissions.organization_id, 3);
    assert_eq!(permissions.required_permission, "BURN_ORGANIZATION_TOKENS");
    assert!(permissions.can_burn);
    assert!(permissions.kyc_verified);
    assert!(permissions.can_request_burn);
}

#[test]
fn organization_burn_permissions_require_existing_organization() {
    let mut store = FakeTokenBurnStore {
        kyc_verified: true,
        organization_exists: false,
        can_burn_organization: true,
        ..Default::default()
    };

    let error = block_on(load_organization_token_burn_permissions(&mut store, 7, 3))
        .expect_err("missing organization should fail");

    assert_eq!(error, TokenBurnError::OrganizationNotFound);
}

fn burn_command(source: &str, fee_path: &str) -> TokenBurnCommand {
    TokenBurnCommand {
        amount: BigDecimal::from(10),
        source: source.to_string(),
        fee_path: fee_path.to_string(),
        idempotency_key: "burn-key".to_string(),
        ethereum_address: Some("0xabc".to_string()),
        platform_address: None,
        chain_id: Some(31337),
        contract_address: Some("0xcontract".to_string()),
        transaction_hash: Some("0xtx".to_string()),
        log_index: Some(0),
        deposit_intent_id: None,
        leaderboard_visible: Some(true),
    }
}
