async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role not found");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_active_course_reward_policy(conn: &mut AsyncPgConnection, course_id: i32) {
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
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

fn reward_request(student_user_id: i32, idempotency_key: &str) -> SubmitRewardCandidateRequest {
    SubmitRewardCandidateRequest {
        student_user_id,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: Some(idempotency_key.to_string()),
        evidence: Some(json!({ "completion_percentage": 100 })),
    }
}
