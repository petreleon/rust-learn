use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::index_deposit::ObservedWalletDepositEvent;
use crate::db::schema::wallet_token_deposit_intents;
use crate::domain::wallet::deposit::{
    WalletDepositEventType, WALLET_DEPOSIT_STATUS_AMBIGUOUS, WALLET_DEPOSIT_STATUS_CREDITED,
    WALLET_DEPOSIT_STATUS_PENDING, WALLET_GAS_PAYER_PLATFORM, WALLET_GAS_PAYER_USER,
};
use crate::infra::postgres::models::wallet_token_deposit_intent::WalletTokenDepositIntent;
use crate::infra::postgres::wallet::wallet_deposit_index_ledger::normalize_address;

pub(super) async fn load_matching_pending_deposit_intents(
    conn: &mut AsyncPgConnection,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<Vec<WalletTokenDepositIntent>> {
    if let Some(intent) = find_deposit_intent_by_chain_event(conn, event, true).await? {
        if deposit_intent_matches_observed_event(&intent, event) {
            return Ok(vec![intent]);
        }
        return Ok(Vec::new());
    }

    let from_address = normalize_address(&event.from_address);
    let to_address = normalize_address(&event.to_address);

    let pending_intents = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_PENDING))
        .filter(wallet_token_deposit_intents::ethereum_address.eq(from_address))
        .filter(wallet_token_deposit_intents::platform_address.eq(to_address))
        .filter(wallet_token_deposit_intents::amount.eq(event.amount.clone()))
        .order(wallet_token_deposit_intents::created_at.asc())
        .for_update()
        .load::<WalletTokenDepositIntent>(conn)
        .await?;

    Ok(pending_intents
        .into_iter()
        .filter(|intent| deposit_intent_matches_observed_event(intent, event))
        .collect())
}

pub(super) fn deposit_intent_matches_observed_event(
    intent: &WalletTokenDepositIntent,
    event: &ObservedWalletDepositEvent,
) -> bool {
    let expected_event_type = match intent.gas_payer.as_str() {
        WALLET_GAS_PAYER_USER => WalletDepositEventType::Transfer,
        WALLET_GAS_PAYER_PLATFORM => WalletDepositEventType::Import,
        _ => return false,
    };
    let event_transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();
    let event_contract_address = normalize_address(&event.contract_address);

    intent.status == WALLET_DEPOSIT_STATUS_PENDING
        && event.event_type == expected_event_type
        && addresses_equal(&intent.ethereum_address, &event.from_address)
        && addresses_equal(&intent.platform_address, &event.to_address)
        && intent.amount == event.amount
        && intent
            .chain_id
            .map(|id| id == event.chain_id)
            .unwrap_or(true)
        && intent
            .contract_address
            .as_deref()
            .map(|address| addresses_equal(address, &event_contract_address))
            .unwrap_or(true)
        && intent
            .transaction_hash
            .as_deref()
            .map(|hash| hash.eq_ignore_ascii_case(&event_transaction_hash))
            .unwrap_or(true)
        && intent
            .log_index
            .map(|log_index| log_index == event.log_index)
            .unwrap_or(true)
        && intent
            .event_type
            .as_deref()
            .map(|event_type| event_type == event.event_type.as_str())
            .unwrap_or(true)
}

pub(super) async fn find_deposit_intent_by_chain_event(
    conn: &mut AsyncPgConnection,
    event: &ObservedWalletDepositEvent,
    for_update: bool,
) -> QueryResult<Option<WalletTokenDepositIntent>> {
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();
    let query = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)))
        .filter(wallet_token_deposit_intents::transaction_hash.eq(Some(transaction_hash)))
        .filter(wallet_token_deposit_intents::log_index.eq(Some(event.log_index)));

    if for_update {
        query.for_update().first(conn).await.optional()
    } else {
        query.first(conn).await.optional()
    }
}

pub(super) async fn mark_deposit_intents_ambiguous(
    conn: &mut AsyncPgConnection,
    intents: &[WalletTokenDepositIntent],
    event: &ObservedWalletDepositEvent,
) -> QueryResult<usize> {
    let ids = intents.iter().map(|intent| intent.id).collect::<Vec<_>>();
    diesel::update(
        wallet_token_deposit_intents::table.filter(
            wallet_token_deposit_intents::id
                .eq_any(ids)
                .and(wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_PENDING)),
        ),
    )
    .set((
        wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_AMBIGUOUS),
        wallet_token_deposit_intents::transaction_hash
            .eq(Some(event.transaction_hash.trim().to_ascii_lowercase())),
        wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)),
        wallet_token_deposit_intents::log_index.eq(Some(event.log_index)),
        wallet_token_deposit_intents::last_error.eq(Some(
            "multiple pending deposit intents matched the same chain event".to_string(),
        )),
        wallet_token_deposit_intents::updated_at.eq(chrono::Utc::now()),
    ))
    .execute(conn)
    .await
}

pub(super) async fn mark_deposit_intent_credited(
    conn: &mut AsyncPgConnection,
    intent_id: i64,
    transaction_id: i64,
    external_transaction_id: i64,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<WalletTokenDepositIntent> {
    diesel::update(wallet_token_deposit_intents::table.find(intent_id))
        .set((
            wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_CREDITED),
            wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)),
            wallet_token_deposit_intents::contract_address
                .eq(Some(normalize_address(&event.contract_address))),
            wallet_token_deposit_intents::transaction_hash
                .eq(Some(event.transaction_hash.trim().to_ascii_lowercase())),
            wallet_token_deposit_intents::log_index.eq(Some(event.log_index)),
            wallet_token_deposit_intents::event_type
                .eq(Some(event.event_type.as_str().to_string())),
            wallet_token_deposit_intents::external_transaction_id.eq(Some(external_transaction_id)),
            wallet_token_deposit_intents::transaction_id.eq(Some(transaction_id)),
            wallet_token_deposit_intents::last_error.eq(None::<String>),
            wallet_token_deposit_intents::updated_at.eq(chrono::Utc::now()),
            wallet_token_deposit_intents::credited_at.eq(Some(chrono::Utc::now())),
        ))
        .get_result(conn)
        .await
}

fn addresses_equal(left: &str, right: &str) -> bool {
    normalize_address(left) == normalize_address(right)
}
