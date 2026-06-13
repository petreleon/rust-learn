use bigdecimal::BigDecimal;
use futures::executor::block_on;

use super::handler::index_observed_deposit;
use crate::application::wallet::index_deposit::{
    test_support::FakeWalletDepositIndexStore, ObservedWalletDepositEvent, WalletDepositIndexError,
};

#[test]
fn delegates_valid_observed_deposit_to_store() {
    let mut store = FakeWalletDepositIndexStore::default();
    let event = valid_event();

    let output =
        block_on(index_observed_deposit(&mut store, event)).expect("valid event should index");

    assert!(output.credited);
    assert_eq!(output.status, "credited");
    assert_eq!(
        store
            .event
            .expect("event should be captured")
            .transaction_hash,
        "0xhash"
    );
}

#[test]
fn rejects_invalid_chain_id_before_store_call() {
    let mut store = FakeWalletDepositIndexStore::default();
    let mut event = valid_event();
    event.chain_id = 0;

    let error =
        block_on(index_observed_deposit(&mut store, event)).expect_err("invalid chain should fail");

    assert_eq!(
        error,
        WalletDepositIndexError::InvalidInput("observed chain_id must be positive".to_string())
    );
    assert!(store.event.is_none());
}

#[test]
fn rejects_unsupported_event_type() {
    let mut store = FakeWalletDepositIndexStore::default();
    let mut event = valid_event();
    event.event_type = "mint".to_string();

    let error = block_on(index_observed_deposit(&mut store, event))
        .expect_err("unsupported event type should fail");

    assert_eq!(
        error,
        WalletDepositIndexError::InvalidInput(
            "observed event_type must be 'import' or 'transfer'".to_string()
        )
    );
    assert!(store.event.is_none());
}

#[test]
fn rejects_empty_required_addresses() {
    let mut store = FakeWalletDepositIndexStore::default();
    let mut event = valid_event();
    event.from_address = " ".to_string();

    let error = block_on(index_observed_deposit(&mut store, event))
        .expect_err("empty from address should fail");

    assert_eq!(
        error,
        WalletDepositIndexError::InvalidInput("observed from_address is required".to_string())
    );
}

#[test]
fn rejects_non_positive_amount() {
    let mut store = FakeWalletDepositIndexStore::default();
    let mut event = valid_event();
    event.amount = BigDecimal::from(0);

    let error =
        block_on(index_observed_deposit(&mut store, event)).expect_err("zero amount should fail");

    assert_eq!(
        error,
        WalletDepositIndexError::InvalidInput("observed amount must be positive".to_string())
    );
}

fn valid_event() -> ObservedWalletDepositEvent {
    ObservedWalletDepositEvent {
        chain_id: 1,
        contract_address: "0xcontract".to_string(),
        transaction_hash: "0xhash".to_string(),
        log_index: 0,
        event_type: "transfer".to_string(),
        from_address: "0xuser".to_string(),
        to_address: "0xplatform".to_string(),
        amount: BigDecimal::from(100),
    }
}
