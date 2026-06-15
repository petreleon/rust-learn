use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::link_wallet::{LinkedWalletView, WalletLinkError, WalletLinkStore};
use crate::infra::postgres::schema::{organizations, users};
use crate::infra::postgres::wallet::wallet_access::{
    can_link_organization_wallet, can_link_user_wallet,
};
use crate::infra::postgres::wallet::wallet_link_records::{
    link_organization_wallet_record, link_user_wallet_record,
};

pub struct PostgresWalletLinkStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletLinkStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletLinkStore for PostgresWalletLinkStore<'_> {
    fn can_link_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        async move {
            can_link_user_wallet(self.conn, actor_user_id)
                .await
                .map_err(|error| WalletLinkError::UserAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn can_link_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        async move {
            can_link_organization_wallet(self.conn, actor_user_id, organization_id)
                .await
                .map_err(|error| WalletLinkError::OrganizationAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|row| row.is_some())
                .map_err(|error| WalletLinkError::UserLoad(error.to_string()))
        }
        .boxed()
    }

    fn user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::kyc_verified)
                .first::<bool>(self.conn)
                .await
                .map_err(|error| WalletLinkError::KycLoad(error.to_string()))
        }
        .boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletLinkError>> {
        async move {
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|row| row.is_some())
                .map_err(|error| WalletLinkError::OrganizationLoad(error.to_string()))
        }
        .boxed()
    }

    fn link_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>> {
        async move { link_user_wallet_record(self.conn, user_id).await }.boxed()
    }

    fn link_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<LinkedWalletView, WalletLinkError>> {
        async move { link_organization_wallet_record(self.conn, organization_id).await }.boxed()
    }
}
