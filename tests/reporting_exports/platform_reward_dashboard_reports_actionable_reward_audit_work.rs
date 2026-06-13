#[actix_web::test]
async fn platform_reward_dashboard_reports_actionable_reward_audit_work() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_reward_admin").await;
    let platform_moderator = create_test_user(&mut conn, "report_reward_moderator").await;
    let stranger = create_test_user(&mut conn, "report_reward_stranger").await;
    let applicant = create_test_user(&mut conn, "report_reward_applicant").await;
    let student = create_test_user(&mut conn, "report_reward_student").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, platform_moderator.id(), "MODERATOR").await;
    let course = create_course(&mut conn).await;
    create_teacher_application(&mut conn, applicant.id()).await;
    let pending_amount_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_TEACHER_APPROVED,
    )
    .await;
    let failed_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    create_failed_reward_execution_job(&mut conn, failed_candidate).await;
    let mismatch_candidate = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        platform_admin.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(platform_reward_dashboard_use_case(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(
        body["teacher_applications"]["submitted"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["reward_candidates"]["teacher_approved"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["pending_amount_approvals"]
        .as_array()
        .expect("pending amount rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == pending_amount_candidate));
    assert!(
        body["pending_amount_approval_count"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["payout_failure_count"].as_i64().unwrap_or_default() >= 1);
    assert!(body["payout_failures"]
        .as_array()
        .expect("payout failure rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == failed_candidate));
    assert!(
        body["reconciliation_mismatch_count"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["reconciliation_mismatches"]
        .as_array()
        .expect("reconciliation mismatch rows")
        .iter()
        .any(|row| row["reward_candidate_id"] == mismatch_candidate
            && row["mismatch_type"] == "needs_payout_record"));

    let denied_export_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_moderator.id())),
        ))
        .to_request();
    let denied_export_status = match app.call(denied_export_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(denied_export_status, StatusCode::FORBIDDEN);

    let export_req = test::TestRequest::get()
        .uri("/reports/platform/reward-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("teacher_applications,submitted,"));
    assert!(csv.contains("pending_amount_approvals,"));
    assert!(csv.contains("payout_failures,"));
    assert!(csv.contains("reconciliation_mismatches,"));
    assert!(csv.contains("needs_payout_record"));
}
