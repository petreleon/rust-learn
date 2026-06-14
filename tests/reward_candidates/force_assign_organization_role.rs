async fn force_assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role not found");
    organization_role_records::assign_organization_role_to_user(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn create_active_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> i64 {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: event_type.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .returning(reward_policies::id)
        .get_result(conn)
        .await
        .expect("failed to create reward policy")
}

fn reward_request(student_user_id: i32, idempotency_key: &str) -> SubmitRewardCandidateRequest {
    SubmitRewardCandidateRequest {
        student_user_id,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: Some(idempotency_key.to_string()),
        evidence: Some(json!({ "completion_percentage": 100 })),
    }
}

fn teacher_fraud_block_request(teacher_user_id: i32) -> RewardFraudBlockRequest {
    RewardFraudBlockRequest {
        scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
        teacher_user_id: Some(teacher_user_id),
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "teacher reward activity paused".to_string(),
        evidence_reference: Some("case://reward-candidate-flow".to_string()),
        expires_at: None,
    }
}

#[derive(Clone, Copy)]
enum FraudBlockScopeUnderTest {
    Organization,
    Course,
    RewardPolicy,
}

fn scoped_fraud_block_request(
    scope: FraudBlockScopeUnderTest,
    organization_id: i32,
    course_id: i32,
    reward_policy_id: i64,
) -> RewardFraudBlockRequest {
    match scope {
        FraudBlockScopeUnderTest::Organization => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string(),
            teacher_user_id: None,
            organization_id: Some(organization_id),
            course_id: None,
            reward_policy_id: None,
            reason: "organization reward activity paused".to_string(),
            evidence_reference: Some("case://organization-reward-block".to_string()),
            expires_at: None,
        },
        FraudBlockScopeUnderTest::Course => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: Some(course_id),
            reward_policy_id: None,
            reason: "course reward activity paused".to_string(),
            evidence_reference: Some("case://course-reward-block".to_string()),
            expires_at: None,
        },
        FraudBlockScopeUnderTest::RewardPolicy => RewardFraudBlockRequest {
            scope_type: REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY.to_string(),
            teacher_user_id: None,
            organization_id: None,
            course_id: None,
            reward_policy_id: Some(reward_policy_id),
            reason: "reward policy activity paused".to_string(),
            evidence_reference: Some("case://policy-reward-block".to_string()),
            expires_at: None,
        },
    }
}

fn expected_block_message(scope: FraudBlockScopeUnderTest) -> &'static str {
    match scope {
        FraudBlockScopeUnderTest::Organization => "organization reward activity is blocked",
        FraudBlockScopeUnderTest::Course => "course reward activity is blocked",
        FraudBlockScopeUnderTest::RewardPolicy => "reward policy activity is blocked",
    }
}
