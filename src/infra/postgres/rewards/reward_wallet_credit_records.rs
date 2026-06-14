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
