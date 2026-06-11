async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    payment_strategy: &str,
) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: payment_strategy.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
) -> RewardCandidate {
    let candidate = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("reward_execution_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .get_result::<RewardCandidate>(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate.id))
        .set((
            reward_candidates::approved_amount.eq(approved_amount),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(Utc::now())),
            reward_candidates::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
        .expect("failed to update reward candidate amount")
}
