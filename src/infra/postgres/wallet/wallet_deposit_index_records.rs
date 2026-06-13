use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
};
use crate::domain::wallet::deposit::{
    WALLET_DEPOSIT_STATUS_AMBIGUOUS, WALLET_DEPOSIT_STATUS_CREDITED, WALLET_DEPOSIT_STATUS_PENDING,
};
use crate::infra::postgres::wallet::wallet_deposit_index_ledger::{
    apply_deposit_ledger_entries, create_deposit_external_transaction, create_deposit_transaction,
};
use crate::infra::postgres::wallet::wallet_deposit_index_matching::{
    deposit_intent_matches_observed_event, find_deposit_intent_by_chain_event,
    load_matching_pending_deposit_intents, mark_deposit_intent_credited,
    mark_deposit_intents_ambiguous,
};

impl From<DieselError> for WalletDepositIndexError {
    fn from(error: DieselError) -> Self {
        WalletDepositIndexError::Database(error.to_string())
    }
}

pub(super) async fn index_observed_wallet_deposit(
    conn: &mut AsyncPgConnection,
    event: ObservedWalletDepositEvent,
) -> Result<WalletDepositIndexOutput, WalletDepositIndexError> {
    conn.transaction::<_, WalletDepositIndexError, _>(|conn| {
        Box::pin(async move {
            if let Some(existing_intent) =
                find_deposit_intent_by_chain_event(conn, &event, true).await?
            {
                if existing_intent.status == WALLET_DEPOSIT_STATUS_CREDITED
                    || existing_intent.status != WALLET_DEPOSIT_STATUS_PENDING
                {
                    return Ok(WalletDepositIndexOutput {
                        intent_id: Some(existing_intent.id),
                        wallet_id: Some(existing_intent.wallet_id),
                        transaction_id: existing_intent.transaction_id,
                        external_transaction_id: existing_intent.external_transaction_id,
                        internal_transaction_ids: Vec::new(),
                        credited: false,
                        status: existing_intent.status,
                    });
                }

                if !deposit_intent_matches_observed_event(&existing_intent, &event) {
                    log::warn!(
                        "event=wallet_token_deposit_event_mismatch intent_id={} chain_id={} tx_hash={} log_index={} event_type={}",
                        existing_intent.id,
                        event.chain_id,
                        event.transaction_hash,
                        event.log_index,
                        event.event_type
                    );
                    return Ok(uncredited_output(
                        Some(existing_intent.id),
                        Some(existing_intent.wallet_id),
                        "mismatched",
                    ));
                }
            }

            let candidates = load_matching_pending_deposit_intents(conn, &event).await?;
            if candidates.is_empty() {
                log::info!(
                    "event=wallet_token_deposit_event_unmatched chain_id={} tx_hash={} log_index={} from_address={} to_address={} amount={}",
                    event.chain_id,
                    event.transaction_hash,
                    event.log_index,
                    event.from_address,
                    event.to_address,
                    event.amount
                );
                return Ok(uncredited_output(None, None, "unmatched"));
            }

            if candidates.len() > 1 {
                mark_deposit_intents_ambiguous(conn, candidates.as_slice(), &event).await?;
                log::warn!(
                    "event=wallet_token_deposit_event_ambiguous chain_id={} tx_hash={} log_index={} candidate_count={}",
                    event.chain_id,
                    event.transaction_hash,
                    event.log_index,
                    candidates.len()
                );
                return Ok(uncredited_output(
                    None,
                    None,
                    WALLET_DEPOSIT_STATUS_AMBIGUOUS,
                ));
            }

            let Some(intent) = candidates.into_iter().next() else {
                return Err(WalletDepositIndexError::Database(
                    "matched deposit candidate disappeared before crediting".to_string(),
                ));
            };
            let transaction_id = create_deposit_transaction(conn).await?;
            let external_transaction_id =
                create_deposit_external_transaction(conn, transaction_id, &event).await?;
            let internal_transaction_ids = apply_deposit_ledger_entries(
                conn,
                intent.wallet_id,
                transaction_id,
                intent.amount.clone(),
                intent.tax_amount.clone(),
            )
            .await?;
            let credited_intent =
                mark_deposit_intent_credited(conn, intent.id, transaction_id, external_transaction_id, &event)
                    .await?;

            log::info!(
                "event=wallet_token_deposit_credited intent_id={} user_id={} wallet_id={} transaction_id={} external_transaction_id={} amount={} tax_amount={}",
                credited_intent.id,
                credited_intent.user_id,
                credited_intent.wallet_id,
                transaction_id,
                external_transaction_id,
                credited_intent.amount,
                credited_intent.tax_amount
            );

            Ok(WalletDepositIndexOutput {
                intent_id: Some(credited_intent.id),
                wallet_id: Some(credited_intent.wallet_id),
                transaction_id: Some(transaction_id),
                external_transaction_id: Some(external_transaction_id),
                internal_transaction_ids,
                credited: true,
                status: credited_intent.status,
            })
        })
    })
    .await
}

fn uncredited_output(
    intent_id: Option<i64>,
    wallet_id: Option<i32>,
    status: &str,
) -> WalletDepositIndexOutput {
    WalletDepositIndexOutput {
        intent_id,
        wallet_id,
        transaction_id: None,
        external_transaction_id: None,
        internal_transaction_ids: Vec::new(),
        credited: false,
        status: status.to_string(),
    }
}
