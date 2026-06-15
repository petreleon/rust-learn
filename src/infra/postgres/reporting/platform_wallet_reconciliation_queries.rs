use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::{
    platform_wallet_reconciliation_output, PlatformWalletReconciliationError,
    PlatformWalletReconciliationOutput, PlatformWalletReconciliationRowFact,
};
use crate::infra::postgres::models::wallet::Wallet;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_counts::{
    wallet_reconciliation_counts, WalletReconciliationCounts,
};
use crate::infra::postgres::reporting::platform_wallet_reconciliation_mappers::map_diesel_error;
use crate::infra::postgres::schema::wallets;

pub(super) async fn load_platform_wallet_reconciliation(
    conn: &mut AsyncPgConnection,
) -> Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError> {
    let wallet_rows = wallets::table
        .order(wallets::id.desc())
        .load::<Wallet>(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut wallet_facts = Vec::new();

    for wallet in wallet_rows {
        let counts = wallet_reconciliation_counts(conn, &wallet).await?;
        wallet_facts.push(wallet_reconciliation_fact(wallet, counts));
    }

    Ok(platform_wallet_reconciliation_output(wallet_facts))
}

fn wallet_reconciliation_fact(
    wallet: Wallet,
    counts: WalletReconciliationCounts,
) -> PlatformWalletReconciliationRowFact {
    PlatformWalletReconciliationRowFact {
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
    }
}
