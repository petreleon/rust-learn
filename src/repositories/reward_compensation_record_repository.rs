use crate::db::schema::reward_compensation_records;
use crate::models::reward_compensation_record::{
    NewRewardCompensationRecord, RewardCompensationRecord,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_reward_compensation_record(
    conn: &mut AsyncPgConnection,
    new_record: NewRewardCompensationRecord,
) -> QueryResult<RewardCompensationRecord> {
    diesel::insert_into(reward_compensation_records::table)
        .values(&new_record)
        .get_result(conn)
        .await
}

pub async fn find_reward_compensation_record_by_idempotency_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> QueryResult<Option<RewardCompensationRecord>> {
    reward_compensation_records::table
        .filter(reward_compensation_records::idempotency_key.eq(idempotency_key))
        .first(conn)
        .await
        .optional()
}

pub async fn list_reward_compensation_records_by_candidate(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> QueryResult<Vec<RewardCompensationRecord>> {
    reward_compensation_records::table
        .filter(reward_compensation_records::reward_candidate_id.eq(reward_candidate_id))
        .order(reward_compensation_records::created_at.desc())
        .load(conn)
        .await
}

pub async fn list_reward_compensation_records_by_wallet(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
) -> QueryResult<Vec<RewardCompensationRecord>> {
    reward_compensation_records::table
        .filter(reward_compensation_records::wallet_id.eq(wallet_id))
        .order(reward_compensation_records::created_at.desc())
        .load(conn)
        .await
}
