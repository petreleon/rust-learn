use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_wallet_credit_records;
use crate::models::reward_wallet_credit_record::{
    NewRewardWalletCreditRecord, RewardWalletCreditRecord,
};

pub(super) async fn create_reward_wallet_credit_record(
    conn: &mut AsyncPgConnection,
    new_record: NewRewardWalletCreditRecord,
) -> QueryResult<RewardWalletCreditRecord> {
    diesel::insert_into(reward_wallet_credit_records::table)
        .values(&new_record)
        .get_result(conn)
        .await
}

pub(crate) async fn find_reward_wallet_credit_record_by_candidate(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<Option<RewardWalletCreditRecord>> {
    reward_wallet_credit_records::table
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(reward_candidate_id))
        .first(conn)
        .await
        .optional()
}

pub(super) async fn mark_reward_wallet_credit_record_notified(
    conn: &mut AsyncPgConnection,
    reward_credit_record_id: i64,
    notification_id: i64,
) -> QueryResult<RewardWalletCreditRecord> {
    diesel::update(reward_wallet_credit_records::table.find(reward_credit_record_id))
        .set((
            reward_wallet_credit_records::notification_id.eq(Some(notification_id)),
            reward_wallet_credit_records::notified_at.eq(Some(chrono::Utc::now())),
        ))
        .get_result(conn)
        .await
}
