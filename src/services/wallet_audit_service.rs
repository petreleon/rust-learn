use diesel_async::AsyncPgConnection;

use crate::application::wallet::audit_wallet::{self, WalletAudit, WalletAuditError};
use crate::infra::postgres::wallet::wallet_audit_mappers::wallet_audit_target_from_model;
use crate::infra::postgres::wallet::wallet_audit_store::PostgresWalletAuditStore;
use crate::models::wallet::Wallet;

// TODO(level-2-wallet): delete this compatibility wrapper after wallet audit
// routes move from `api/wallets` into `http/wallet`.
pub async fn build_wallet_audit(
    conn: &mut AsyncPgConnection,
    wallet: Wallet,
) -> Result<WalletAudit, WalletAuditError> {
    let target = wallet_audit_target_from_model(&wallet);
    let mut store = PostgresWalletAuditStore::new(conn);
    audit_wallet::audit_wallet(&mut store, target).await
}
