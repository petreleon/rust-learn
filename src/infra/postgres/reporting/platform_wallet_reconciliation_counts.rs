use std::collections::HashSet;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationError;
use crate::db::schema::{
    internal_transactions, reward_candidates, reward_payout_records, reward_wallet_credit_records,
};
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_COMPLETED, REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::infra::postgres::reporting::platform_wallet_reconciliation_missing::{
    count_missing_notifications, count_missing_payouts,
};
use crate::models::wallet::Wallet;

pub(super) struct WalletReconciliationCounts {
    pub internal_transaction_count: i64,
    pub external_transaction_count: i64,
    pub reward_record_count: i64,
    pub needs_reconciliation_count: i64,
    pub missing_credit_count: i64,
    pub missing_notification_count: i64,
    pub missing_payout_count: i64,
}

pub(super) async fn wallet_reconciliation_counts(
    conn: &mut AsyncPgConnection,
    wallet: &Wallet,
) -> Result<WalletReconciliationCounts, PlatformWalletReconciliationError> {
    let internal_transaction_count = internal_transactions::table
        .filter(internal_transactions::wallet_id.eq(wallet.id))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)?;
    let candidate_ids = reward_candidate_ids(conn, wallet).await?;
    let reward_record_count = candidate_ids.len() as i64;

    Ok(WalletReconciliationCounts {
        internal_transaction_count,
        external_transaction_count: count_payout_records(conn, &candidate_ids).await?,
        reward_record_count,
        needs_reconciliation_count: count_status(
            conn,
            &candidate_ids,
            REWARD_STATUS_NEEDS_RECONCILIATION,
        )
        .await?,
        missing_credit_count: count_missing_credits(conn, &candidate_ids).await?,
        missing_notification_count: count_missing_notifications(conn, &candidate_ids).await?,
        missing_payout_count: count_missing_payouts(conn, &candidate_ids).await?,
    })
}

async fn reward_candidate_ids(
    conn: &mut AsyncPgConnection,
    wallet: &Wallet,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    let credit_candidate_ids: Vec<i64> = reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::wallet_id.eq(wallet.id))
        .select(reward_wallet_credit_records::reward_candidate_id)
        .load(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut candidate_ids: HashSet<i64> = credit_candidate_ids.into_iter().collect();
    if let Some(user_id) = wallet.user_id {
        candidate_ids.extend(candidate_ids_for_user(conn, user_id).await?);
    }
    if let Some(organization_id) = wallet.organization_id {
        candidate_ids.extend(candidate_ids_for_organization(conn, organization_id).await?);
    }
    Ok(candidate_ids.into_iter().collect())
}

async fn candidate_ids_for_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    reward_candidates::table
        .filter(reward_candidates::student_user_id.eq(user_id))
        .select(reward_candidates::id)
        .load::<i64>(conn)
        .await
        .map_err(map_diesel_error)
}

async fn candidate_ids_for_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i64>, PlatformWalletReconciliationError> {
    reward_candidates::table
        .filter(reward_candidates::source_organization_id.eq(organization_id))
        .select(reward_candidates::id)
        .load::<i64>(conn)
        .await
        .map_err(map_diesel_error)
}

async fn count_payout_records(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> Result<i64, PlatformWalletReconciliationError> {
    if candidate_ids.is_empty() {
        return Ok(0);
    }
    reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq_any(candidate_ids))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

async fn count_status(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    status: &str,
) -> Result<i64, PlatformWalletReconciliationError> {
    if candidate_ids.is_empty() {
        return Ok(0);
    }
    reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .filter(reward_candidates::status.eq(status))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

async fn count_missing_credits(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
) -> Result<i64, PlatformWalletReconciliationError> {
    if candidate_ids.is_empty() {
        return Ok(0);
    }
    reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .filter(
            reward_candidates::status
                .eq(REWARD_STATUS_WALLET_CREDITED)
                .or(reward_candidates::status.eq(REWARD_STATUS_NOTIFIED))
                .or(reward_candidates::status.eq(REWARD_STATUS_COMPLETED)),
        )
        .left_join(
            reward_wallet_credit_records::table
                .on(reward_candidates::id.eq(reward_wallet_credit_records::reward_candidate_id)),
        )
        .filter(reward_wallet_credit_records::id.is_null())
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformWalletReconciliationError {
    PlatformWalletReconciliationError::Database(error.to_string())
}
