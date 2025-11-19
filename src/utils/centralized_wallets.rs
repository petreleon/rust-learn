use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::wallet::{Wallet, NewWallet};
use crate::models::transaction::{Transaction, InternalTransaction, TransactionLink};


/// Owner type for locating a wallet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerType {
    User,
    Organization,
}

impl OwnerType {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "user" | "users" => Ok(OwnerType::User),
            "organization" | "org" | "organizations" => Ok(OwnerType::Organization),
            other => Err(anyhow!("unknown owner type: {} (expected 'user' or 'organization')", other)),
        }
    }
}

// No raw SQL structs needed with DSL

/// Locate a wallet for a given owner (type, id). If none exists, create one with value=0 and return it.
pub fn wallet_locator(conn: &mut PgConnection, owner_type: &str, owner_id: i32) -> Result<i32> {
    let owner = OwnerType::parse(owner_type)?;

    match owner {
        OwnerType::User => {
            if let Some(existing_id) = Wallet::find_by_user_id(owner_id, conn)? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_wallet = NewWallet {
                user_id: Some(owner_id),
                organization_id: None,
                value: zero,
            };
            let new_id = Wallet::create(new_wallet, conn)?;
            Ok(new_id)
        }
        OwnerType::Organization => {
            if let Some(existing_id) = Wallet::find_by_organization_id(owner_id, conn)? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_wallet = NewWallet {
                user_id: None,
                organization_id: Some(owner_id),
                value: zero,
            };
            let new_id = Wallet::create(new_wallet, conn)?;
            Ok(new_id)
        }
    }

}

#[derive(Debug)]
pub struct TransferResult {
    pub transaction_id: i64,
    pub debit_internal_id: i64,
    pub credit_internal_id: i64,
}

/// Core helper: applies an internal transaction effect to a single wallet.
/// - Inserts into internal_transactions (amount may be positive or negative)
/// - Updates the wallet balance atomically (SELECT ... FOR UPDATE, then UPDATE)
/// Returns the created internal_transactions.id
pub fn transact(conn: &mut PgConnection, wallet_id: i32, amount: BigDecimal) -> Result<i64> {
    // Perform the guarded atomic update first: ensure balance doesn't go negative.
    // Use RETURNING id to check that the row was updated. If no rows were affected,
    // the guard failed (would go negative) and we return an error.
    let updated_rows = Wallet::update_balance_guarded(wallet_id, amount.clone(), conn)?;

    if updated_rows == 0 {
        return Err(anyhow!("insufficient funds or wallet not found"));
    }

    // Now insert the internal transaction row (we already adjusted the balance)
    let internal_id = InternalTransaction::create(wallet_id, amount, conn)?;

    Ok(internal_id)
}

/// Debits a wallet by `amount` (amount must be positive). Internally calls `transact` with negative amount.
pub fn pay(conn: &mut PgConnection, wallet_id: i32, amount: BigDecimal) -> Result<i64> {
    if amount <= BigDecimal::from(0) {
        return Err(anyhow!("amount must be positive for pay"));
    }
    transact(conn, wallet_id, -amount)
}

/// Credits a wallet by `amount` (amount must be positive). Internally calls `transact` with positive amount.
pub fn receive(conn: &mut PgConnection, wallet_id: i32, amount: BigDecimal) -> Result<i64> {
    if amount <= BigDecimal::from(0) {
        return Err(anyhow!("amount must be positive for receive"));
    }
    transact(conn, wallet_id, amount)
}

/// Perform an internal transfer between two wallets.
/// Creates:
/// - two internal_transactions rows (debit negative, credit positive)
/// - one generic transactions row of type 'internal_transfer'
/// - two links in transactions_internal_transactions
/// Also updates wallet balances.
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
            let _locked = Wallet::lock_wallets(ids, txn)?;

            // Perform debit and credit using the helper (these will do guarded updates)
            let debit_id = pay(txn, from_wallet_id, amount.clone())?;
            let credit_id = receive(txn, to_wallet_id, amount.clone())?;

            // Create generic transaction
            let tx_id = Transaction::create("internal_transfer", txn)?;

            // Link generic transaction to internal entries
            TransactionLink::create(tx_id, debit_id, txn)?;
            TransactionLink::create(tx_id, credit_id, txn)?;

            Ok(TransferResult { transaction_id: tx_id, debit_internal_id: debit_id, credit_internal_id: credit_id })
        });

        match result {
            Ok(r) => return Ok(r),
            Err(e) => {
                // If Diesel returned a DB error indicating a deadlock or serialization failure,
                // retry the whole transaction a few times with backoff.
                let should_retry = match e.downcast_ref::<DieselError>() {
                    Some(DieselError::DatabaseError(kind, info)) => {
                        matches!(kind, DatabaseErrorKind::SerializationFailure) || info.message().contains("deadlock")
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
