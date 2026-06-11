async fn link_course_to_org(conn: &mut AsyncPgConnection, course_id: i32, organization_id: i32) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}

async fn create_org_wallet(conn: &mut AsyncPgConnection, organization_id: i32) {
    create_org_wallet_with_value(conn, organization_id, BigDecimal::from(0)).await;
}

async fn create_org_wallet_with_value(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    value: BigDecimal,
) -> i32 {
    diesel::insert_into(wallets::table)
        .values(NewWallet {
            user_id: None,
            organization_id: Some(organization_id),
            value,
        })
        .returning(wallets::id)
        .get_result(conn)
        .await
        .expect("failed to create organization wallet")
}

async fn create_teacher_application(conn: &mut AsyncPgConnection, applicant_user_id: i32) -> i64 {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: None,
            requested_course_id: None,
            experience_summary: "reporting dashboard applicant".to_string(),
            organization_sponsor_id: None,
            portfolio_links: json!([]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
            idempotency_key: None,
        })
        .returning(teacher_applications::id)
        .get_result(conn)
        .await
        .expect("failed to create teacher application")
}

async fn create_sponsored_teacher_application(
    conn: &mut AsyncPgConnection,
    applicant_user_id: i32,
    organization_id: i32,
) -> i64 {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: Some(organization_id),
            requested_course_id: None,
            experience_summary: "organization reward dashboard applicant".to_string(),
            organization_sponsor_id: Some(organization_id),
            portfolio_links: json!([]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
            idempotency_key: None,
        })
        .returning(teacher_applications::id)
        .get_result(conn)
        .await
        .expect("failed to create sponsored teacher application")
}

async fn create_reward_candidate_with_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
) -> i64 {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("report_reward_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set(reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))))
        .execute(conn)
        .await
        .expect("failed to set reward candidate amount");

    candidate_id
}
