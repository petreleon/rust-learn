use crate::application::rewards::list_reward_history::validation::validated_filter;
use crate::application::rewards::list_reward_history::{
    StudentRewardCandidateRecord, StudentRewardHistoryEntry, StudentRewardHistoryError,
    StudentRewardHistoryQuery,
};
use crate::application::rewards::ports::StudentRewardHistoryStore;

pub async fn list_student_reward_history(
    store: &mut impl StudentRewardHistoryStore,
    actor_user_id: i32,
    query: StudentRewardHistoryQuery,
) -> Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError> {
    let filter = validated_filter(actor_user_id, query)?;
    let rows = store.list_student_reward_candidates(filter).await?;
    let mut history = Vec::new();

    for row in rows {
        if !store
            .can_view_course_reward_status(actor_user_id, row.course_id)
            .await?
        {
            continue;
        }
        history.push(history_entry(store, row).await?);
    }

    Ok(history)
}

async fn history_entry(
    store: &mut impl StudentRewardHistoryStore,
    row: StudentRewardCandidateRecord,
) -> Result<StudentRewardHistoryEntry, StudentRewardHistoryError> {
    let wallet_credit = store.load_wallet_credit(row.reward_candidate_id).await?;
    let token_transaction = store
        .load_token_transaction(row.reward_candidate_id)
        .await?;

    Ok(StudentRewardHistoryEntry {
        reward_candidate_id: row.reward_candidate_id,
        course_id: row.course_id,
        course_title: row.course_title,
        event_type: row.event_type,
        status: row.status,
        approved_amount: row.approved_amount,
        wallet_credit,
        token_transaction,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
