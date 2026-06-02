use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rust_learn::db::schema::{external_transactions, transactions};
use rust_learn::services::token_reconciliation::{
    record_token_event, ObservedTokenEvent, TokenEventKind,
};

fn sync_connection() -> PgConnection {
    let _ = dotenvy::dotenv();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("failed to connect to database")
}

fn unique_hash(prefix: &str) -> String {
    format!(
        "0x{}{:x}{:x}",
        prefix,
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    )
}

fn observed_event() -> ObservedTokenEvent {
    ObservedTokenEvent {
        chain_id: 31337,
        contract_address: "0x00000000000000000000000000000000000000aa".to_string(),
        transaction_hash: unique_hash("abc"),
        log_index: 7,
        event_type: TokenEventKind::Transfer,
        from_address: Some("0x00000000000000000000000000000000000000bb".to_string()),
        to_address: "0x00000000000000000000000000000000000000cc".to_string(),
        amount: BigDecimal::from(42),
    }
}

#[test]
fn record_token_event_inserts_external_transaction_and_is_idempotent() {
    let mut conn = sync_connection();
    let event = observed_event();

    let first = record_token_event(&mut conn, &event).expect("first token event should record");
    assert!(first.inserted);

    let second =
        record_token_event(&mut conn, &event).expect("duplicate token event should reconcile");
    assert!(!second.inserted);
    assert_eq!(first.transaction_id, second.transaction_id);
    assert_eq!(
        first.external_transaction_id,
        second.external_transaction_id
    );

    let external = external_transactions::table
        .find(first.external_transaction_id)
        .first::<rust_learn::models::transaction::ExternalTransaction>(&mut conn)
        .expect("external transaction should exist");
    assert_eq!(external.chain_id, Some(event.chain_id));
    assert_eq!(
        external.contract_address.as_deref(),
        Some(event.contract_address.as_str())
    );
    assert_eq!(
        external.transaction_hash.as_deref(),
        Some(event.transaction_hash.as_str())
    );
    assert_eq!(external.log_index, Some(event.log_index));
    assert_eq!(external.event_type.as_deref(), Some("transfer"));
    assert_eq!(
        external.from_address.as_deref(),
        event.from_address.as_deref()
    );
    assert_eq!(
        external.to_address.as_deref(),
        Some(event.to_address.as_str())
    );
    assert_eq!(external.amount, event.amount);

    let transaction_type = transactions::table
        .find(first.transaction_id)
        .select(transactions::type_)
        .first::<String>(&mut conn)
        .expect("generic transaction should exist");
    assert_eq!(transaction_type, "token_transfer");
}
