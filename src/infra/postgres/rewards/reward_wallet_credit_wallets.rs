use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::credit_wallet::RewardWalletCreditError;
use crate::infra::postgres::models::wallet::{NewWallet, Wallet};
use crate::infra::postgres::rewards::reward_wallet_credit_mappers::map_diesel_error;
use crate::infra::postgres::schema::wallets;

pub(super) async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Wallet, RewardWalletCreditError> {
    if let Some(wallet) = find_user_wallet(conn, user_id).await? {
        return Ok(wallet);
    }

    let new_wallet = NewWallet {
        user_id: Some(user_id),
        organization_id: None,
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(wallet),
        Err(error) if is_unique_violation(&error) => find_user_wallet(conn, user_id)
            .await?
            .ok_or(RewardWalletCreditError::NotFound),
        Err(error) => Err(map_diesel_error(error)),
    }
}

pub(super) async fn credit_wallet_balance(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<Wallet, RewardWalletCreditError> {
    diesel::update(wallets::table.find(wallet_id))
        .set(wallets::value.eq(wallets::value + amount))
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<Wallet>, RewardWalletCreditError> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first(conn)
        .await
        .optional()
        .map_err(map_diesel_error)
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}
