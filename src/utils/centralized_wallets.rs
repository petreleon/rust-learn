use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;


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
            use crate::db::schema::wallets::dsl as W;
            if let Some(existing_id) = W::wallets
                .select(W::id)
                .filter(W::user_id.eq(owner_id).and(W::organization_id.is_null()))
                .first::<i32>(conn)
                .optional()? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_id: i32 = diesel::insert_into(W::wallets)
                .values((W::user_id.eq(owner_id), W::organization_id.eq::<Option<i32>>(None), W::value.eq(zero)))
                .returning(W::id)
                .get_result(conn)?;
            Ok(new_id)
        }
        OwnerType::Organization => {
            use crate::db::schema::wallets::dsl as W;
            if let Some(existing_id) = W::wallets
                .select(W::id)
                .filter(W::organization_id.eq(owner_id).and(W::user_id.is_null()))
                .first::<i32>(conn)
                .optional()? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_id: i32 = diesel::insert_into(W::wallets)
                .values((W::user_id.eq::<Option<i32>>(None), W::organization_id.eq(owner_id), W::value.eq(zero)))
                .returning(W::id)
                .get_result(conn)?;
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
    use crate::db::schema::internal_transactions::dsl as IT;
    use crate::db::schema::wallets::dsl as W;

    // Perform the guarded atomic update first: ensure balance doesn't go negative.
    // Use RETURNING id to check that the row was updated. If no rows were affected,
    // the guard failed (would go negative) and we return an error.
    let zero = BigDecimal::from(0);
    let updated_rows = diesel::update(
        W::wallets.filter(
            W::id
                .eq(wallet_id)
                .and((W::value + amount.clone()).ge(zero.clone())),
        ),
    )
    .set(W::value.eq(W::value + amount.clone()))
    .execute(conn)?;

    if updated_rows == 0 {
        return Err(anyhow!("insufficient funds or wallet not found"));
    }

    // Now insert the internal transaction row (we already adjusted the balance)
    let internal_id: i64 = diesel::insert_into(IT::internal_transactions)
        .values((IT::wallet_id.eq(wallet_id), IT::amount.eq(amount.clone())))
        .returning(IT::id)
        .get_result(conn)?;

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
            use crate::db::schema::transactions::dsl as TX;
            use crate::db::schema::transactions_internal_transactions::dsl as LINK;
            use crate::db::schema::wallets::dsl as W;

            if from_wallet_id == to_wallet_id {
                return Err(anyhow!("cannot transfer to the same wallet"));
            }

            // Lock both wallet rows in ascending id order to avoid cycles
            let mut ids = vec![from_wallet_id, to_wallet_id];
            ids.sort_unstable();
            let _locked: Vec<(i32, BigDecimal)> = W::wallets
                .select((W::id, W::value))
                .filter(W::id.eq_any(&ids))
                .order(W::id.asc())
                .for_update()
                .load(txn)?;

            // Perform debit and credit using the helper (these will do guarded updates)
            let debit_id = pay(txn, from_wallet_id, amount.clone())?;
            let credit_id = receive(txn, to_wallet_id, amount.clone())?;

            // Create generic transaction
            let tx_id: i64 = diesel::insert_into(TX::transactions)
                .values(TX::type_.eq("internal_transfer"))
                .returning(TX::id)
                .get_result(txn)?;

            // Link generic transaction to internal entries
            diesel::insert_into(LINK::transactions_internal_transactions)
                .values((LINK::transaction_id.eq(tx_id), LINK::internal_transaction_id.eq(debit_id)))
                .execute(txn)?;

            diesel::insert_into(LINK::transactions_internal_transactions)
                .values((LINK::transaction_id.eq(tx_id), LINK::internal_transaction_id.eq(credit_id)))
                .execute(txn)?;

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
