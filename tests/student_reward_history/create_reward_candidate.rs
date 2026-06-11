async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    amount: BigDecimal,
) -> i64 {
    let candidate_id = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("student_reward_history_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(amount)),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(chrono::Utc::now())),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .execute(conn)
        .await
        .expect("failed to set approved amount");

    candidate_id
}

struct RewardFinancialFixture {
    payout_transaction_id: i64,
    external_transaction_id: i64,
    wallet_transaction_id: i64,
    internal_transaction_id: i64,
    wallet_credit_record_id: i64,
    transaction_hash: String,
}
