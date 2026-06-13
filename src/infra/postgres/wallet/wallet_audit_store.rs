use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditTarget, WalletAuditWallet,
};
use crate::infra::postgres::wallet::wallet_audit_candidate_ids::load_wallet_reward_candidate_ids;
use crate::infra::postgres::wallet::wallet_audit_compensation_records::load_compensation_records;
use crate::infra::postgres::wallet::wallet_audit_external_transactions::load_external_transactions;
use crate::infra::postgres::wallet::wallet_audit_internal_transactions::load_internal_transactions;
use crate::infra::postgres::wallet::wallet_audit_reward_records::load_reward_records;

pub struct PostgresWalletAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletAuditStore for PostgresWalletAuditStore<'_> {
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
