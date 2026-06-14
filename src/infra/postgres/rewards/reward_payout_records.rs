use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_payout_records;
use crate::models::reward_payout_record::{NewRewardPayoutRecord, RewardPayoutRecord};

pub(super) async fn create_reward_payout_record(
    conn: &mut AsyncPgConnection,
    new_record: NewRewardPayoutRecord,
) -> QueryResult<RewardPayoutRecord> {
    diesel::insert_into(reward_payout_records::table)
        .values(&new_record)
        .get_result(conn)
        .await
}

pub(super) async fn find_reward_payout_record_by_candidate(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<Option<RewardPayoutRecord>> {
    reward_payout_records::table
        .filter(reward_payout_records::reward_candidate_id.eq(reward_candidate_id))
        .first(conn)
        .await
        .optional()
}
