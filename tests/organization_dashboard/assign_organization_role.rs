async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn create_sponsored_teacher_application(
    conn: &mut AsyncPgConnection,
    applicant_user_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(teacher_applications::table)
        .values(NewTeacherApplication {
            applicant_user_id,
            requested_scope: TEACHER_APPLICATION_SCOPE_PLATFORM.to_string(),
            requested_organization_id: None,
            requested_course_id: None,
            experience_summary: "Organization-sponsored Rust teaching candidate.".to_string(),
            organization_sponsor_id: Some(organization_id),
            portfolio_links: json!(["https://example.test/portfolio"]),
            status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
            idempotency_key: Some(unique_string("dashboard_teacher_application")),
        })
        .execute(conn)
        .await
        .expect("failed to create sponsored teacher application");
}

async fn create_org_wallet(conn: &mut AsyncPgConnection, organization_id: i32, value: BigDecimal) {
    diesel::insert_into(wallets::table)
        .values(NewWallet {
            user_id: None,
            organization_id: Some(organization_id),
            value,
        })
        .execute(conn)
        .await
        .expect("failed to create organization wallet");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
    status: &str,
    approved_amount: Option<BigDecimal>,
) {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: Some(organization_id),
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("dashboard_reward_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: status.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    if let Some(amount) = approved_amount {
        diesel::update(reward_candidates::table.find(candidate_id))
            .set(reward_candidates::approved_amount.eq(Some(amount)))
            .execute(conn)
            .await
            .expect("failed to set approved amount");
    }
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn alert_kind_exists(body: &Value, expected: &str) -> bool {
    body["alerts"]
        .as_array()
        .map(|alerts| {
            alerts
                .iter()
                .any(|alert| alert["kind"].as_str() == Some(expected))
        })
        .unwrap_or(false)
}

fn missing_permission_exists(body: &Value, section: &str, expected: &str) -> bool {
    body[section]["missing_permissions"]
        .as_array()
        .map(|permissions| {
            permissions
                .iter()
                .any(|permission| permission.as_str() == Some(expected))
        })
        .unwrap_or(false)
}
