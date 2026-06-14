use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::link_wallet::{LinkedWalletView, WalletLinkError};
use crate::db::schema::wallets;
use crate::infra::postgres::wallet::wallet_mappers::wallet_view_from_model;
use crate::models::wallet::{NewWallet, Wallet};

pub(super) async fn link_user_wallet_record(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<LinkedWalletView, WalletLinkError> {
    if let Some(wallet) = find_user_wallet(conn, user_id).await? {
        return Ok(linked_wallet(wallet, false));
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
        Ok(wallet) => Ok(linked_wallet(wallet, true)),
        Err(error) if is_unique_violation(&error) => find_user_wallet(conn, user_id)
            .await?
            .map(|wallet| linked_wallet(wallet, false))
            .ok_or_else(|| WalletLinkError::WalletLoad("wallet not found after replay".into())),
        Err(error) => Err(WalletLinkError::WalletCreate(error.to_string())),
    }
}

pub(super) async fn link_organization_wallet_record(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<LinkedWalletView, WalletLinkError> {
    if let Some(wallet) = find_organization_wallet(conn, organization_id).await? {
        return Ok(linked_wallet(wallet, false));
    }

    let new_wallet = NewWallet {
        user_id: None,
        organization_id: Some(organization_id),
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(linked_wallet(wallet, true)),
        Err(error) if is_unique_violation(&error) => {
            find_organization_wallet(conn, organization_id)
                .await?
                .map(|wallet| linked_wallet(wallet, false))
                .ok_or_else(|| WalletLinkError::WalletLoad("wallet not found after replay".into()))
        }
        Err(error) => Err(WalletLinkError::WalletCreate(error.to_string())),
    }
}

async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<Wallet>, WalletLinkError> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first::<Wallet>(conn)
        .await
        .optional()
        .map_err(|error| WalletLinkError::WalletLoad(error.to_string()))
}

async fn find_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Option<Wallet>, WalletLinkError> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .filter(wallets::user_id.is_null())
        .first::<Wallet>(conn)
        .await
        .optional()
        .map_err(|error| WalletLinkError::WalletLoad(error.to_string()))
}

fn linked_wallet(wallet: Wallet, created: bool) -> LinkedWalletView {
    LinkedWalletView {
        wallet: wallet_view_from_model(wallet),
        created,
    }
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}
