#[actix_web::test]
async fn learner_progress_requires_enrollment_and_course_content() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let previewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, previewer.id(), "SUPER_ADMIN").await;

    let course = create_course(&mut conn, &unique_string("ProgressCourse")).await;
    let other_course = create_course(&mut conn, &unique_string("ProgressOther")).await;
    publish_course(&mut conn, course.id).await;
    publish_course(&mut conn, other_course.id).await;
    assign_course_role(&mut conn, learner.id(), course.id, "STUDENT").await;

    let chapter_id = create_chapter(&mut conn, course.id, "Module one", 0).await;
    let first_content_id =
        create_content_with_data(&mut conn, chapter_id, "text", 0, Some("Start here.")).await;
    let second_content_id =
        create_content_with_data(&mut conn, chapter_id, "text", 1, Some("Continue here.")).await;
    let other_chapter_id = create_chapter(&mut conn, other_course.id, "Other", 0).await;
    let other_content_id =
        create_content_with_data(&mut conn, other_chapter_id, "text", 0, Some("Wrong course."))
            .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let preview_save = test::TestRequest::post()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(previewer.id())),
        ))
        .set_json(serde_json::json!({ "content_id": first_content_id }))
        .to_request();
    assert_eq!(
        test::call_service(&app, preview_save).await.status(),
        StatusCode::FORBIDDEN
    );

    let wrong_course_save = test::TestRequest::post()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner.id()))))
        .set_json(serde_json::json!({ "content_id": other_content_id }))
        .to_request();
    assert_eq!(
        test::call_service(&app, wrong_course_save).await.status(),
        StatusCode::NOT_FOUND
    );

    let first_save = test::TestRequest::post()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner.id()))))
        .set_json(serde_json::json!({ "content_id": first_content_id }))
        .to_request();
    let first_resp = test::call_service(&app, first_save).await;
    assert_eq!(first_resp.status(), StatusCode::OK);
    let first_body: Value = test::read_body_json(first_resp).await;
    assert_eq!(
        first_body["content_id"].as_i64(),
        Some(i64::from(first_content_id))
    );

    let second_save = test::TestRequest::post()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner.id()))))
        .set_json(serde_json::json!({ "content_id": second_content_id }))
        .to_request();
    assert_eq!(test::call_service(&app, second_save).await.status(), StatusCode::OK);

    let progress_req = test::TestRequest::get()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner.id()))))
        .to_request();
    let progress_resp = test::call_service(&app, progress_req).await;
    assert_eq!(progress_resp.status(), StatusCode::OK);
    let progress_body: Value = test::read_body_json(progress_resp).await;
    assert_eq!(
        progress_body["content_id"].as_i64(),
        Some(i64::from(second_content_id))
    );
}
