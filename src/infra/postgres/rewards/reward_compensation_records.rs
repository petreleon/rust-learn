use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_compensation_records;
use crate::infra::postgres::models::reward_compensation_record::{
    NewRewardCompensationRecord, RewardCompensationRecord,
};

pub(super) async fn create_reward_compensation_record(
    conn: &mut AsyncPgConnection,
    new_record: NewRewardCompensationRecord,
) -> QueryResult<RewardCompensationRecord> {
    diesel::insert_into(reward_compensation_records::table)
        .values(&new_record)
        .get_result(conn)
        .await
}

pub(super) async fn find_reward_compensation_record_by_idempotency_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> QueryResult<Option<RewardCompensationRecord>> {
    reward_compensation_records::table
        .filter(reward_compensation_records::idempotency_key.eq(idempotency_key))
        .first(conn)
        .await
        .optional()
}
