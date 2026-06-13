#[actix_web::test]
async fn organization_course_list_uses_org_scope_and_returns_operator_summaries() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let operator = create_test_user(&mut conn).await;
    let teacher = create_test_user(&mut conn).await;
    let student = create_test_user(&mut conn).await;
    let org = create_organization(&mut conn, &unique_string("OrgCourseList")).await;
    let other_org = create_organization(&mut conn, &unique_string("OrgCourseOther")).await;
    let prefix = unique_string("OrgCourse");
    let rewarded = create_course(&mut conn, &format!("{} Rust Rewards", prefix)).await;
    let unrewarded = create_course(&mut conn, &format!("{} Rust Plain", prefix)).await;
    let external = create_course(&mut conn, &format!("{} Rust External", prefix)).await;

    link_course_to_org(&mut conn, rewarded.id, org.id, 0).await;
    link_course_to_org(&mut conn, unrewarded.id, org.id, 1).await;
    link_course_to_org(&mut conn, external.id, other_org.id, 0).await;
    publish_course(&mut conn, rewarded.id).await;
    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_course_role(&mut conn, teacher.id(), rewarded.id, "TEACHER").await;
    assign_course_role(&mut conn, student.id(), rewarded.id, "STUDENT").await;
    let chapter_id = create_chapter(&mut conn, rewarded.id, "Reward basics", 0).await;
    create_content(&mut conn, chapter_id, "article", 0).await;
    create_course_reward_policy(&mut conn, rewarded.id, "manual_completion").await;
    create_pending_join_request(&mut conn, student.id(), rewarded.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/organizations/{}/courses?search=rust&reward_available=true&limit=10",
            org.id
        ))
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
    assert_eq!(body["total"].as_i64(), Some(1));
    assert_eq!(body["courses"].as_array().expect("courses array").len(), 1);
    let course = &body["courses"][0];
    assert_eq!(course["title"].as_str(), Some(rewarded.title.as_str()));
    assert_eq!(course["lifecycle_status"].as_str(), Some("published"));
    assert_eq!(
        course["teachers"][0]["name"].as_str(),
        Some(teacher.name.as_str())
    );
    assert_eq!(course["content"]["chapter_count"].as_u64(), Some(1));
    assert_eq!(course["content"]["content_count"].as_u64(), Some(1));
    assert_eq!(course["roster"]["enrolled_student_count"].as_i64(), Some(1));
    assert_eq!(
        course["roster"]["pending_join_request_count"].as_i64(),
        Some(1)
    );
    assert_eq!(course["rewards"]["available"].as_bool(), Some(true));
    assert_eq!(
        course["rewards"]["event_types"][0].as_str(),
        Some("manual_completion")
    );
    assert_eq!(
        course["permissions"]["can_view_courses"].as_bool(),
        Some(true)
    );
    assert_eq!(
        course["permissions"]["can_submit_reward_events"].as_bool(),
        Some(true)
    );
}

#[actix_web::test]
async fn organization_course_list_denies_users_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn).await;
    let org = create_organization(&mut conn, &unique_string("OrgCourseDenied")).await;
    let course = create_course(&mut conn, &unique_string("OrgCourseDeniedCourse")).await;
    link_course_to_org(&mut conn, course.id, org.id, 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/courses", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
