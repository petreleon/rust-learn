async fn link_course_to_org(conn: &mut AsyncPgConnection, course_id: i32, organization_id: i32) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course and organization");
}

async fn create_chapter(conn: &mut AsyncPgConnection, course_id: i32, title: &str) -> i32 {
    diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id,
            title: title.to_string(),
            order: 0,
        })
        .returning(chapters::id)
        .get_result(conn)
        .await
        .expect("failed to create chapter")
}

async fn create_content(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_type: &str,
    data: Option<&str>,
) {
    diesel::insert_into(contents::table)
        .values(NewContent {
            chapter_id,
            content_type: content_type.to_string(),
            data: data.map(ToString::to_string),
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to create content");
}

async fn create_join_request(
    conn: &mut AsyncPgConnection,
    requester_user_id: i32,
    course_id: i32,
    status: &str,
) {
    diesel::insert_into(course_join_requests::table)
        .values(NewCourseJoinRequest {
            course_id,
            requester_user_id,
            status: status.to_string(),
        })
        .execute(conn)
        .await
        .expect("failed to create join request");
}

async fn create_reward_policy(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            active: true,
            cooldown_seconds: 0,
            course_id: Some(course_id),
            created_by_user_id: None,
            event_type: "course_completion".to_string(),
            max_payout: None,
            multiplier: BigDecimal::from(1),
            organization_id: None,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            token_amount: BigDecimal::from(25),
            version: 1,
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
) {
    diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            event_type: "course_completion".to_string(),
            evidence: json!({ "source": "teacher dashboard test" }),
            idempotency_key: unique_string("teacher_dashboard_reward"),
            source_organization_id: None,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            status: status.to_string(),
            student_user_id,
            submitter_user_id,
        })
        .execute(conn)
        .await
        .expect("failed to create reward candidate");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}
