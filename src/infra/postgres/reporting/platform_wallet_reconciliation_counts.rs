use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationError;
use crate::application::reporting::platform_wallet_reconciliation::{
    missing_credit_record_candidate_statuses, needs_reconciliation_candidate_statuses,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::models::wallet::Wallet;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_candidate_ids::reward_candidate_ids;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_mappers::map_diesel_error;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_missing::{
    count_missing_notifications, count_missing_payouts,
};
use crate::infra::postgres::schema::{
    internal_transactions, reward_candidates, reward_payout_records, reward_wallet_credit_records,
};

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
        needs_reconciliation_count: count_statuses(
            conn,
            &candidate_ids,
            needs_reconciliation_candidate_statuses(),
        )
        .await?,
        missing_credit_count: count_missing_credits(conn, &candidate_ids).await?,
        missing_notification_count: count_missing_notifications(conn, &candidate_ids).await?,
        missing_payout_count: count_missing_payouts(conn, &candidate_ids).await?,
    })
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

async fn count_statuses<const N: usize>(
    conn: &mut AsyncPgConnection,
    candidate_ids: &[i64],
    statuses: [RewardCandidateStatus; N],
) -> Result<i64, PlatformWalletReconciliationError> {
    if candidate_ids.is_empty() {
        return Ok(0);
    }
    reward_candidates::table
        .filter(reward_candidates::id.eq_any(candidate_ids))
        .filter(reward_candidates::status.eq_any(status_keys(statuses)))
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
                .eq_any(status_keys(missing_credit_record_candidate_statuses())),
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

pub(super) fn status_keys<const N: usize>(
    statuses: [RewardCandidateStatus; N],
) -> Vec<&'static str> {
    statuses
        .into_iter()
        .map(RewardCandidateStatus::as_str)
        .collect()
}
