use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::read_wallet::{WalletReadError, WalletReadStore, WalletView};
use crate::db::schema::{organizations, users, wallets};
use crate::infra::postgres::wallet::wallet_access::{
    can_view_organization_wallet, can_view_user_wallet,
};
use crate::infra::postgres::wallet::wallet_read_mappers::wallet_view_from_model;
use crate::models::wallet::Wallet;

pub struct PostgresWalletReadStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletReadStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletReadStore for PostgresWalletReadStore<'_> {
    fn can_view_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        async move {
            can_view_user_wallet(self.conn, actor_user_id)
                .await
                .map_err(|error| WalletReadError::UserAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn can_view_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        async move {
            can_view_organization_wallet(self.conn, actor_user_id, organization_id)
                .await
                .map_err(|error| WalletReadError::OrganizationAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|row| row.is_some())
                .map_err(|error| WalletReadError::UserLoad(error.to_string()))
        }
        .boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletReadError>> {
        async move {
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|row| row.is_some())
                .map_err(|error| WalletReadError::OrganizationLoad(error.to_string()))
        }
        .boxed()
    }

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>> {
        async move {
            wallets::table
                .filter(wallets::user_id.eq(user_id))
                .filter(wallets::organization_id.is_null())
                .first::<Wallet>(self.conn)
                .await
                .optional()
                .map(|wallet| wallet.map(wallet_view_from_model))
                .map_err(|error| WalletReadError::WalletLoad(error.to_string()))
        }
        .boxed()
    }

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletView>, WalletReadError>> {
        async move {
            wallets::table
                .filter(wallets::organization_id.eq(organization_id))
                .filter(wallets::user_id.is_null())
                .first::<Wallet>(self.conn)
                .await
                .optional()
                .map(|wallet| wallet.map(wallet_view_from_model))
                .map_err(|error| WalletReadError::WalletLoad(error.to_string()))
        }
        .boxed()
    }
}
