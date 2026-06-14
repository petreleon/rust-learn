use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::{
    platform_wallet_reconciliation_row, PlatformWalletReconciliationError,
    PlatformWalletReconciliationOutput, PlatformWalletReconciliationRowFact,
    PlatformWalletReconciliationRowOutput,
};
use crate::db::schema::wallets;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_counts::{
    wallet_reconciliation_counts, WalletReconciliationCounts,
};
use crate::infra::postgres::reporting::platform_wallet_reconciliation_mappers::map_diesel_error;
use crate::models::wallet::Wallet;

pub(super) async fn load_platform_wallet_reconciliation(
    conn: &mut AsyncPgConnection,
) -> Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError> {
    let wallet_rows = wallets::table
        .order(wallets::id.desc())
        .load::<Wallet>(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut rows = Vec::new();
    let mut total_internal_transactions = 0;
    let mut total_external_transactions = 0;
    let mut total_reward_records = 0;
    let mut total_needs_reconciliation = 0;

    for wallet in wallet_rows {
        let counts = wallet_reconciliation_counts(conn, &wallet).await?;
        total_internal_transactions += counts.internal_transaction_count;
        total_external_transactions += counts.external_transaction_count;
        total_reward_records += counts.reward_record_count;
        total_needs_reconciliation += counts.needs_reconciliation_count;
        rows.push(wallet_reconciliation_row(wallet, counts));
    }

    Ok(PlatformWalletReconciliationOutput {
        total_wallets: rows.len() as i64,
        total_internal_transactions,
        total_external_transactions,
        total_reward_records,
        total_needs_reconciliation,
        wallets: rows,
    })
}

fn wallet_reconciliation_row(
    wallet: Wallet,
    counts: WalletReconciliationCounts,
) -> PlatformWalletReconciliationRowOutput {
    platform_wallet_reconciliation_row(PlatformWalletReconciliationRowFact {
        wallet_id: wallet.id,
        user_id: wallet.user_id,
        organization_id: wallet.organization_id,
        balance: wallet.value,
        internal_transaction_count: counts.internal_transaction_count,
        external_transaction_count: counts.external_transaction_count,
        reward_record_count: counts.reward_record_count,
        needs_reconciliation_count: counts.needs_reconciliation_count,
        missing_credit_count: counts.missing_credit_count,
        missing_notification_count: counts.missing_notification_count,
        missing_payout_count: counts.missing_payout_count,
    })
}
