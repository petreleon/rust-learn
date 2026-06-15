use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::audit_wallet::{WalletAuditError, WalletAuditTarget};
use crate::db::schema::{organizations, users, wallets};
use crate::infra::postgres::models::wallet::Wallet;
use crate::infra::postgres::wallet::wallet_audit_mappers::wallet_audit_target_from_model;

pub(super) async fn user_exists(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<bool, WalletAuditError> {
    users::table
        .find(user_id)
        .select(users::id)
        .first::<i32>(conn)
        .await
        .optional()
        .map(|row| row.is_some())
        .map_err(|error| WalletAuditError::UserLoad(error.to_string()))
}

pub(super) async fn organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<bool, WalletAuditError> {
    organizations::table
        .find(organization_id)
        .select(organizations::id)
        .first::<i32>(conn)
        .await
        .optional()
        .map(|row| row.is_some())
        .map_err(|error| WalletAuditError::OrganizationLoad(error.to_string()))
}

pub(super) async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<WalletAuditTarget>, WalletAuditError> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first::<Wallet>(conn)
        .await
        .optional()
        .map(|wallet| wallet.as_ref().map(wallet_audit_target_from_model))
        .map_err(|error| WalletAuditError::WalletLoad(error.to_string()))
}

pub(super) async fn find_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Option<WalletAuditTarget>, WalletAuditError> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .filter(wallets::user_id.is_null())
        .first::<Wallet>(conn)
        .await
        .optional()
        .map(|wallet| wallet.as_ref().map(wallet_audit_target_from_model))
        .map_err(|error| WalletAuditError::WalletLoad(error.to_string()))
}
