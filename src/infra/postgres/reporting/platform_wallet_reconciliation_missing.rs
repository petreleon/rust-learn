use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_wallet_reconciliation::{
    missing_notification_record_candidate_statuses, missing_payout_record_candidate_statuses,
    PlatformWalletReconciliationError,
};
use crate::db::schema::{reward_candidates, reward_payout_records, reward_wallet_credit_records};
use crate::infra::postgres::reporting::platform_wallet_reconciliation_counts::map_diesel_error;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_counts::status_keys;

pub(super) async fn count_missing_notifications(
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
                .eq_any(status_keys(missing_notification_record_candidate_statuses())),
        )
        .left_join(
            reward_wallet_credit_records::table
                .on(reward_candidates::id.eq(reward_wallet_credit_records::reward_candidate_id)),
        )
        .filter(reward_wallet_credit_records::notification_id.is_null())
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

pub(super) async fn count_missing_payouts(
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
                .eq_any(status_keys(missing_payout_record_candidate_statuses())),
        )
        .left_join(
            reward_payout_records::table
                .on(reward_candidates::id.eq(reward_payout_records::reward_candidate_id)),
        )
        .filter(reward_payout_records::id.is_null())
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
