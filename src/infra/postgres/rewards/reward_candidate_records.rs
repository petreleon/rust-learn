use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::reward_candidates;
use crate::models::reward_candidate::RewardCandidate;

pub(super) async fn find_candidate(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    reward_candidates::table
        .find(candidate_id)
        .first::<RewardCandidate>(conn)
        .await
}
