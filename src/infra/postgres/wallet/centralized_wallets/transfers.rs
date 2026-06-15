use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infra::postgres::wallet::wallet_ledger_records::{
    create_transaction, link_internal_transaction, lock_wallets_for_update,
};

use super::records::{pay, receive, wallet_locator};

#[derive(Debug)]
pub struct TransferResult {
    pub transaction_id: i64,
    pub debit_internal_id: i64,
    pub credit_internal_id: i64,
}

/// Perform an internal transfer between two wallets.
/// Creates:
/// - two internal_transactions rows (debit negative, credit positive)
/// - one generic transactions row of type 'internal_transfer'
/// - two links in transactions_internal_transactions
/// - Also updates wallet balances.
pub fn transfers_between_wallets(
    conn: &mut PgConnection,
    from_wallet_id: i32,
    to_wallet_id: i32,
    amount: BigDecimal,
) -> Result<TransferResult> {
    if amount <= BigDecimal::from(0) {
        return Err(anyhow!("amount must be positive"));
    }
    // Prevent deadlocks by always locking wallet rows in a deterministic order
    // and retrying on transient serialization/deadlock errors.
    use diesel::result::DatabaseErrorKind;
    use diesel::result::Error as DieselError;
    use std::time::Duration;
    const MAX_RETRIES: usize = 4;

    for attempt in 0..MAX_RETRIES {
        let result = conn.transaction::<TransferResult, anyhow::Error, _>(|txn| {
            if from_wallet_id == to_wallet_id {
                return Err(anyhow!("cannot transfer to the same wallet"));
            }

            // Lock both wallet rows in ascending id order to avoid cycles
            let mut ids = vec![from_wallet_id, to_wallet_id];
            ids.sort_unstable();
            let _locked = lock_wallets_for_update(ids, txn)?;

            // Perform debit and credit using the helper (these will do guarded updates)
            let debit_id = pay(txn, from_wallet_id, amount.clone())?;
            let credit_id = receive(txn, to_wallet_id, amount.clone())?;

            // Create generic transaction
            let tx_id = create_transaction("internal_transfer", txn)?;

            // Link generic transaction to internal entries
            link_internal_transaction(tx_id, debit_id, txn)?;
            link_internal_transaction(tx_id, credit_id, txn)?;

            Ok(TransferResult {
                transaction_id: tx_id,
                debit_internal_id: debit_id,
                credit_internal_id: credit_id,
            })
        });

        match result {
            Ok(r) => return Ok(r),
            Err(e) => {
                // If Diesel returned a DB error indicating a deadlock or serialization failure,
                // retry the whole transaction a few times with backoff.
                let should_retry = match e.downcast_ref::<DieselError>() {
                    Some(DieselError::DatabaseError(kind, info)) => {
                        matches!(kind, DatabaseErrorKind::SerializationFailure)
                            || info.message().contains("deadlock")
                    }
                    _ => false,
                };

                if should_retry && attempt + 1 < MAX_RETRIES {
                    // exponential backoff
                    let backoff = Duration::from_millis(50 * (1 << attempt) as u64);
                    std::thread::sleep(backoff);
                    continue;
                }

                return Err(e);
            }
        }
    }

    Err(anyhow!("transfer failed after retries"))
}

/// Send money from one owner (type, id) to another.
/// Types: "user" or "organization". Creates wallets if missing.
pub fn send_money(
    conn: &mut PgConnection,
    from_type: &str,
    from_id: i32,
    to_type: &str,
    to_id: i32,
    amount: BigDecimal,
) -> Result<TransferResult> {
    let from_wallet = wallet_locator(conn, from_type, from_id)?;
    let to_wallet = wallet_locator(conn, to_type, to_id)?;
    transfers_between_wallets(conn, from_wallet, to_wallet, amount)
}
