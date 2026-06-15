#[actix_web::test]
async fn learner_catalog_returns_published_course_summaries_and_pending_state() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let teacher = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, learner.id(), "USER").await;

    let org = create_organization(&mut conn, &unique_string("CatalogOrg")).await;
    let prefix = unique_string("CatalogCourse");
    let published = create_course(&mut conn, &format!("{} Published Rust", prefix)).await;
    let hidden_draft = create_course(&mut conn, &format!("{} Hidden Draft", prefix)).await;
    publish_course(&mut conn, published.id).await;
    link_course_to_org(&mut conn, published.id, org.id, 0).await;
    link_course_to_org(&mut conn, hidden_draft.id, org.id, 1).await;
    assign_course_role(&mut conn, teacher.id(), published.id, "TEACHER").await;
    let chapter_id = create_chapter(&mut conn, published.id, "Getting started", 0).await;
    create_content(&mut conn, chapter_id, "video", 0).await;
    create_course_reward_policy(&mut conn, published.id, "course_completion").await;
    let request_id = create_pending_join_request(&mut conn, learner.id(), published.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(learner_course_catalog_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/catalog?search={}&reward_available=true&limit=10",
            prefix
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(1));
    let course = &body["courses"][0];
    assert_eq!(course["id"].as_i64(), Some(i64::from(published.id)));
    assert_eq!(course["title"].as_str(), Some(published.title.as_str()));
    assert_eq!(
        course["organizations"][0]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(
        course["teachers"][0]["name"].as_str(),
        Some(teacher.name.as_str())
    );
    assert_eq!(course["content"]["chapter_count"].as_u64(), Some(1));
    assert_eq!(course["content"]["content_count"].as_u64(), Some(1));
    assert_eq!(
        course["content"]["content_types"][0].as_str(),
        Some("video")
    );
    assert_eq!(course["rewards"]["available"].as_bool(), Some(true));
    assert_eq!(
        course["rewards"]["event_types"][0].as_str(),
        Some("course_completion")
    );
    assert_eq!(course["enrollment"]["state"].as_str(), Some("pending"));
    assert_eq!(
        course["enrollment"]["request_id"].as_i64(),
        Some(request_id)
    );
    assert_eq!(
        course["enrollment"]["can_request_join"].as_bool(),
        Some(false)
    );
    assert_eq!(course["access"]["can_request_join"].as_bool(), Some(true));
}
