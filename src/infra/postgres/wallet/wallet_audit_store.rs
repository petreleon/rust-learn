use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditTarget, WalletAuditWallet,
};
use crate::infra::postgres::wallet::wallet_access::{
    can_view_organization_wallet, can_view_user_wallet,
};
use crate::infra::postgres::wallet::wallet_audit_candidate_ids::load_wallet_reward_candidate_ids;
use crate::infra::postgres::wallet::wallet_audit_compensation_records::load_compensation_records;
use crate::infra::postgres::wallet::wallet_audit_external_transactions::load_external_transactions;
use crate::infra::postgres::wallet::wallet_audit_internal_transactions::load_internal_transactions;
use crate::infra::postgres::wallet::wallet_audit_reward_records::load_reward_records;
use crate::infra::postgres::wallet::wallet_audit_wallet_lookup::{
    find_organization_wallet, find_user_wallet, organization_exists, user_exists,
};

pub struct PostgresWalletAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletAuditStore for PostgresWalletAuditStore<'_> {
    fn can_view_user_wallet(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        async move {
            can_view_user_wallet(self.conn, actor_user_id)
                .await
                .map_err(|error| WalletAuditError::UserAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn can_view_organization_wallet(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        async move {
            can_view_organization_wallet(self.conn, actor_user_id, organization_id)
                .await
                .map_err(|error| WalletAuditError::OrganizationAccessCheck(error.to_string()))
        }
        .boxed()
    }

    fn user_exists(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        async move { user_exists(self.conn, user_id).await }.boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletAuditError>> {
        async move { organization_exists(self.conn, organization_id).await }.boxed()
    }

    fn find_user_wallet(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>> {
        async move { find_user_wallet(self.conn, user_id).await }.boxed()
    }

    fn find_organization_wallet(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<WalletAuditTarget>, WalletAuditError>> {
        async move { find_organization_wallet(self.conn, organization_id).await }.boxed()
    }

    fn load_wallet_audit(
        &mut self,
        target: WalletAuditTarget,
    ) -> BoxFuture<'_, Result<WalletAudit, WalletAuditError>> {
        async move { load_wallet_audit(self.conn, target).await }.boxed()
    }
}

async fn load_wallet_audit(
    conn: &mut AsyncPgConnection,
    target: WalletAuditTarget,
) -> Result<WalletAudit, WalletAuditError> {
    let wallet_id = target.id;
    let internal_transactions = load_internal_transactions(conn, wallet_id).await?;
    let wallet_transaction_ids = internal_transactions
        .iter()
        .map(|row| row.transaction_id)
        .collect::<Vec<_>>();
    let candidate_ids = load_wallet_reward_candidate_ids(conn, &target).await?;
    let reward_records = load_reward_records(conn, candidate_ids.as_slice()).await?;
    let external_transactions = load_external_transactions(
        conn,
        candidate_ids.as_slice(),
        wallet_transaction_ids.as_slice(),
    )
    .await?;
    let compensation_records = load_compensation_records(conn, wallet_id).await?;

    Ok(WalletAudit {
        wallet: WalletAuditWallet {
            id: target.id,
            owner_type: target.owner_type().to_string(),
            user_id: target.user_id,
            organization_id: target.organization_id,
            value: target.value,
        },
        internal_transactions,
        external_transactions,
        reward_records,
        compensation_records,
    })
}
