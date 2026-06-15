use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::record_compensation::RewardCompensationError;
use crate::infra::postgres::models::wallet::{NewWallet, Wallet};
use crate::infra::postgres::rewards::reward_compensation_mappers::map_reward_compensation_error;
use crate::infra::postgres::schema::wallets;

pub(super) async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Wallet, RewardCompensationError> {
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
            .ok_or(RewardCompensationError::NotFound),
        Err(error) => Err(map_reward_compensation_error(error)),
    }
}

pub(super) async fn apply_wallet_adjustment(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<Wallet, RewardCompensationError> {
    let updated_wallet = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount))
    .get_result::<Wallet>(conn)
    .await;

    match updated_wallet {
        Ok(wallet) => Ok(wallet),
        Err(diesel::result::Error::NotFound) => Err(RewardCompensationError::InsufficientFunds),
        Err(error) => Err(map_reward_compensation_error(error)),
    }
}

async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<Wallet>, RewardCompensationError> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first(conn)
        .await
        .optional()
        .map_err(map_reward_compensation_error)
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}
