async fn assert_teacher_dashboard_list(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/courses/teaching?limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.teacher.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(1));
    let dashboard_course = &body["courses"][0];
    assert_eq!(
        dashboard_course["id"].as_i64(),
        Some(i64::from(fixture.course.id))
    );
    assert_eq!(
        dashboard_course["organizations"][0]["name"].as_str(),
        Some(fixture.org.name.as_str())
    );
    assert_eq!(
        dashboard_course["lifecycle_status"].as_str(),
        Some(COURSE_STATUS_PUBLISHED)
    );
    assert_eq!(
        dashboard_course["content"]["content_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["rewards"]["active_policy_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["enrolled_student_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["pending_join_request_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["roster"]["waitlisted_join_request_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["pending_teacher_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["teacher_approved_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["reward_queue"]["failed_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        dashboard_course["permissions"]["can_manage_settings"].as_bool(),
        Some(true)
    );
    assert_eq!(
        dashboard_course["permissions"]["can_approve_reward_candidates"].as_bool(),
        Some(true)
    );
}
