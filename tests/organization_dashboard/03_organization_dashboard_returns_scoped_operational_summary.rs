#[actix_web::test]
async fn organization_dashboard_returns_scoped_operational_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardOrg")).await;
    let operator = create_test_user(&mut conn, "dashboard_operator").await;
    let learner = create_test_user(&mut conn, "dashboard_learner").await;
    let teacher = create_test_user(&mut conn, "dashboard_teacher").await;
    let applicant = create_test_user(&mut conn, "dashboard_applicant").await;
    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, learner.id(), org.id, "STUDENT").await;
    assign_organization_role(&mut conn, teacher.id(), org.id, "TEACHER").await;

    let published_course = create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardPublishedCourse"),
        COURSE_STATUS_PUBLISHED,
    )
    .await;
    create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardNeedsChangesCourse"),
        COURSE_STATUS_NEEDS_CHANGES,
    )
    .await;
    create_sponsored_teacher_application(&mut conn, applicant.id(), org.id).await;
    create_org_wallet(&mut conn, org.id, BigDecimal::from(125)).await;
    create_reward_candidate(
        &mut conn,
        published_course.id,
        org.id,
        learner.id(),
        teacher.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(40)),
    )
    .await;
    create_reward_candidate(
        &mut conn,
        published_course.id,
        org.id,
        learner.id(),
        teacher.id(),
        REWARD_STATUS_FAILED,
        None,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(operator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["organization"]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(body["health"]["status"].as_str(), Some("attention"));
    assert_eq!(body["members"]["available"].as_bool(), Some(true));
    assert_eq!(body["members"]["total"].as_i64(), Some(3));
    assert_eq!(body["courses"]["total"].as_i64(), Some(2));
    assert_eq!(body["courses"]["published"].as_i64(), Some(1));
    assert_eq!(body["courses"]["needs_changes"].as_i64(), Some(1));
    assert_eq!(body["teacher_applications"]["submitted"].as_i64(), Some(1));
    assert_eq!(body["rewards"]["available"].as_bool(), Some(true));
    assert_eq!(body["rewards"]["reward_candidate_count"].as_i64(), Some(2));
    assert_eq!(body["rewards"]["approved_reward_count"].as_i64(), Some(1));
    assert_eq!(
        body["rewards"]["approved_amount_total"].as_str(),
        Some("40")
    );
    assert_eq!(body["rewards"]["failed_count"].as_i64(), Some(1));
    assert_eq!(body["wallet"]["available"].as_bool(), Some(true));
    assert_eq!(body["wallet"]["wallet_count"].as_i64(), Some(1));
    assert_eq!(body["wallet"]["balance_total"].as_str(), Some("125"));
    assert_eq!(
        body["operator_permissions"]["can_view_reports"].as_bool(),
        Some(true)
    );
    assert!(alert_kind_exists(&body, "teacher_applications_submitted"));
    assert!(alert_kind_exists(&body, "courses_need_changes"));
    assert!(alert_kind_exists(&body, "reward_reconciliation"));
}
