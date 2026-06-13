async fn assert_teacher_students(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .app_data(teacher_dashboard_use_case_data(&fixture.pool))
            .app_data(teacher_workspace_use_case_data(&fixture.pool))
            .app_data(teacher_enrollment_workspace_use_case_data(&fixture.pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/teaching/{}/students", fixture.course.id))
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
    assert_eq!(body["total"].as_i64(), Some(1));
    assert_eq!(body["progress_supported"].as_bool(), Some(true));
    assert_eq!(body["reward_eligibility_supported"].as_bool(), Some(true));
    assert!(
        body["reward_eligibility"]["active_policy_count"]
            .as_u64()
            .unwrap_or_default()
            >= 1
    );
    assert_eq!(body["reward_evidence_supported"].as_bool(), Some(true));
    let student_progress = &body["students"][0];
    assert_eq!(
        student_progress["user"]["name"].as_str(),
        Some(fixture.student.name.as_str())
    );
    assert_eq!(
        student_progress["progress"]["supported"].as_bool(),
        Some(true)
    );
    assert_eq!(
        student_progress["progress"]["completed_content_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        student_progress["progress"]["total_content_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        student_progress["progress"]["completion_percentage"].as_f64(),
        Some(100.0)
    );
    assert_eq!(
        student_progress["progress"]["current_content_label"].as_str(),
        Some("Module 1, lesson 1: article")
    );
    assert!(student_progress["progress"]["last_activity_at"].is_string());
    assert_eq!(
        student_progress["rewards"]["reward_candidate_count"].as_i64(),
        Some(3)
    );
    assert_eq!(
        student_progress["reward_eligibility"]["reward_candidate_count"].as_i64(),
        Some(3)
    );
    let event_types = student_progress["reward_eligibility"]["event_types"]
        .as_array()
        .expect("student reward eligibility events");
    assert!(event_types
        .iter()
        .any(|event| event.as_str() == Some("course_completion")));
    assert_eq!(
        student_progress["rewards"]["pending_teacher_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        student_progress["rewards"]["teacher_approved_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        student_progress["rewards"]["failed_count"].as_i64(),
        Some(1)
    );
    assert_eq!(
        student_progress["rewards"]["latest_candidate"]["status"].as_str(),
        Some(REWARD_STATUS_FAILED)
    );
}

async fn assert_outsider_teacher_dashboard_access(fixture: &TeacherDashboardFixture) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(fixture.pool.clone()))
            .app_data(teacher_dashboard_use_case_data(&fixture.pool))
            .app_data(teacher_workspace_use_case_data(&fixture.pool))
            .app_data(teacher_enrollment_workspace_use_case_data(&fixture.pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri("/courses/teaching?limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.outsider.id())),
        ))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let list_body: Value = test::read_body_json(list_resp).await;
    assert_eq!(list_body["total"].as_i64(), Some(0));
    assert!(list_body["courses"]
        .as_array()
        .expect("courses array")
        .is_empty());

    let workspace_req = test::TestRequest::get()
        .uri(&format!("/courses/teaching/{}", fixture.course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.outsider.id())),
        ))
        .to_request();
    let workspace_resp = test::call_service(&app, workspace_req).await;
    assert_eq!(workspace_resp.status(), StatusCode::FORBIDDEN);

    let enrollments_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/teaching/{}/enrollments",
            fixture.course.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.outsider.id())),
        ))
        .to_request();
    let enrollments_resp = test::call_service(&app, enrollments_req).await;
    assert_eq!(enrollments_resp.status(), StatusCode::FORBIDDEN);

    let students_req = test::TestRequest::get()
        .uri(&format!("/courses/teaching/{}/students", fixture.course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(fixture.outsider.id())),
        ))
        .to_request();
    let students_resp = test::call_service(&app, students_req).await;
    assert_eq!(students_resp.status(), StatusCode::FORBIDDEN);
}
