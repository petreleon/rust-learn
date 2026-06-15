use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;

use crate::infra::postgres::models::wallet::NewWallet;
use crate::infra::postgres::wallet::wallet_ledger_records::{
    create_internal_transaction, create_wallet, find_organization_wallet_id, find_user_wallet_id,
    update_wallet_balance_guarded,
};

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
            other => Err(anyhow!(
                "unknown owner type: {} (expected 'user' or 'organization')",
                other
            )),
        }
    }
}

// No raw SQL structs needed with DSL

/// Locate a wallet for a given owner (type, id). If none exists, create one with value=0 and return it.
pub fn wallet_locator(conn: &mut PgConnection, owner_type: &str, owner_id: i32) -> Result<i32> {
    let owner = OwnerType::parse(owner_type)?;

    match owner {
        OwnerType::User => {
            if let Some(existing_id) = find_user_wallet_id(owner_id, conn)? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_wallet = NewWallet {
                user_id: Some(owner_id),
                organization_id: None,
                value: zero,
            };
            let new_id = create_wallet(new_wallet, conn)?;
            Ok(new_id)
        }
        OwnerType::Organization => {
            if let Some(existing_id) = find_organization_wallet_id(owner_id, conn)? {
                return Ok(existing_id);
            }
            let zero = BigDecimal::from(0);
            let new_wallet = NewWallet {
                user_id: None,
                organization_id: Some(owner_id),
                value: zero,
            };
            let new_id = create_wallet(new_wallet, conn)?;
            Ok(new_id)
        }
    }
}

/// Core helper: applies an internal transaction effect to a single wallet.
/// - Inserts into internal_transactions (amount may be positive or negative)
/// - Updates the wallet balance atomically (SELECT ... FOR UPDATE, then UPDATE)
/// - Returns the created internal_transactions.id
pub fn transact(conn: &mut PgConnection, wallet_id: i32, amount: BigDecimal) -> Result<i64> {
    // Perform the guarded atomic update first: ensure balance doesn't go negative.
    // Use RETURNING id to check that the row was updated. If no rows were affected,
    // the guard failed (would go negative) and we return an error.
    let updated_rows = update_wallet_balance_guarded(wallet_id, amount.clone(), conn)?;

    if updated_rows == 0 {
        return Err(anyhow!("insufficient funds or wallet not found"));
    }

    // Now insert the internal transaction row (we already adjusted the balance)
    let internal_id = create_internal_transaction(wallet_id, amount, conn)?;

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
