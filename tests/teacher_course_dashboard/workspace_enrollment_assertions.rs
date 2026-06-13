async fn assert_teacher_workspace(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/teaching/{}", fixture.course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.teacher.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["course"]["id"].as_i64(),
        Some(i64::from(fixture.course.id))
    );
    assert_eq!(
        body["publication"]["course_lifecycle_status"].as_str(),
        Some(COURSE_STATUS_PUBLISHED)
    );
    assert_eq!(
        body["publication"]["content_publication_status_supported"].as_bool(),
        Some(false)
    );
    assert_eq!(body["teacher_roles"][0].as_str(), Some("TEACHER"));
    assert_eq!(
        body["chapters"][0]["title"].as_str(),
        Some("Dashboard chapter")
    );
    assert_eq!(
        body["chapters"][0]["contents"][0]["content_type"].as_str(),
        Some("article")
    );
    assert_eq!(
        body["chapters"][0]["contents"][0]["data_present"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["chapters"][0]["contents"][0]["data"].as_str(),
        Some("Workspace lesson text")
    );
    assert_eq!(
        body["chapters"][0]["contents"][0]["publication_status"].as_str(),
        Some("inherits_course_published")
    );
    assert_eq!(
        body["chapters"][0]["contents"][0]["display_state"].as_str(),
        Some("ready")
    );
}

async fn assert_teacher_enrollments(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/teaching/{}/enrollments?limit=10",
            fixture.course.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.teacher.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["course"]["id"].as_i64(),
        Some(i64::from(fixture.course.id))
    );
    assert_eq!(body["join_requests"]["status"].as_str(), Some("open"));
    assert_eq!(body["join_requests"]["total"].as_i64(), Some(2));
    let requests = body["join_requests"]["requests"]
        .as_array()
        .expect("join requests array");
    assert!(requests.iter().any(|request| {
        request["requester"]["name"].as_str() == Some(fixture.pending_learner.name.as_str())
            && request["status"].as_str() == Some(COURSE_JOIN_STATUS_PENDING)
            && request["can_decide"].as_bool() == Some(true)
    }));
    assert!(requests.iter().any(|request| {
        request["requester"]["name"].as_str() == Some(fixture.outsider.name.as_str())
            && request["status"].as_str() == Some(COURSE_JOIN_STATUS_WAITLISTED)
            && request["can_decide"].as_bool() == Some(true)
    }));
    assert_eq!(body["roster"]["total"].as_i64(), Some(1));
    assert_eq!(
        body["roster"]["learners"][0]["user"]["name"].as_str(),
        Some(fixture.student.name.as_str())
    );
    assert_eq!(
        body["roster"]["learners"][0]["latest_join_request_status"].as_str(),
        Some(COURSE_JOIN_STATUS_APPROVED)
    );
    assert_eq!(
        body["roster"]["learners"][0]["access_state"].as_str(),
        Some("enrolled")
    );
    assert_eq!(
        body["roster"]["learners"][0]["can_remove"].as_bool(),
        Some(true)
    );
    assert_eq!(body["progress_supported"].as_bool(), Some(true));
    assert_eq!(body["reward_eligibility_supported"].as_bool(), Some(true));
    assert!(
        body["reward_eligibility"]["active_policy_count"]
            .as_u64()
            .unwrap_or_default()
            >= 1
    );
    let event_types = body["reward_eligibility"]["event_types"]
        .as_array()
        .expect("reward eligibility events");
    assert!(event_types
        .iter()
        .any(|event| event.as_str() == Some("course_completion")));
    assert_eq!(
        body["roster"]["learners"][0]["reward_eligibility"]["reward_candidate_count"].as_i64(),
        Some(3)
    );
    assert_eq!(
        body["roster"]["learners"][0]["reward_eligibility"]["active_policy_count"].as_u64(),
        body["reward_eligibility"]["active_policy_count"].as_u64()
    );
}

async fn assert_teacher_pending_filter(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;
    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/teaching/{}/enrollments?status=pending&limit=10",
            fixture.course.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.teacher.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["join_requests"]["total"].as_i64(), Some(1));
    assert_eq!(
        body["join_requests"]["requests"][0]["status"].as_str(),
        Some(COURSE_JOIN_STATUS_PENDING)
    );
}
