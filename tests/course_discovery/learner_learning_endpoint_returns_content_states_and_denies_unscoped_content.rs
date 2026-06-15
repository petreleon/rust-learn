use crate::{create_organization::*, support::*};

#[actix_web::test]
async fn learner_learning_endpoint_returns_content_states_and_denies_unscoped_content() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let outsider = create_test_user(&mut conn).await;
    let auditor = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, auditor.id(), "SUPER_ADMIN").await;

    let course = create_course(&mut conn, &unique_string("LearningCourse")).await;
    publish_course(&mut conn, course.id).await;
    assign_course_role(&mut conn, learner.id(), course.id, "STUDENT").await;
    let later_chapter_id = create_chapter(&mut conn, course.id, "Module two", 1).await;
    let later_content_id = create_content_with_data(
        &mut conn,
        later_chapter_id,
        "text",
        0,
        Some("Later module."),
    )
    .await;
    let chapter_id = create_chapter(&mut conn, course.id, "Module one", 0).await;
    let text_content_id = create_content_with_data(
        &mut conn,
        chapter_id,
        "text",
        0,
        Some("Welcome to ownership."),
    )
    .await;
    let object_key = format!("courses/{}/chapters/{}/video.mp4", course.id, chapter_id);
    let video_content_id =
        create_content_with_data(&mut conn, chapter_id, "video", 1, Some(&object_key)).await;
    create_upload_job(
        &mut conn,
        &object_key,
        "failed",
        Some("ffmpeg failed during thumbnail extraction"),
    )
    .await;
    let empty_video_id = create_content_with_data(&mut conn, chapter_id, "video", 2, None).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(learner_course_learning_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["course"]["id"].as_i64(), Some(i64::from(course.id)));
    assert_eq!(
        body["active_content_id"].as_i64(),
        Some(i64::from(text_content_id))
    );
    assert_eq!(
        body["course"]["enrollment"]["state"].as_str(),
        Some("enrolled")
    );
    assert_eq!(body["progress_supported"].as_bool(), Some(true));

    let chapters = body["chapters"]
        .as_array()
        .expect("learning chapters array");
    assert_eq!(chapters[0]["id"].as_i64(), Some(i64::from(chapter_id)));
    assert_eq!(
        chapters[1]["id"].as_i64(),
        Some(i64::from(later_chapter_id))
    );
    assert_eq!(
        chapters[1]["contents"][0]["id"].as_i64(),
        Some(i64::from(later_content_id))
    );

    let contents = chapters[0]["contents"]
        .as_array()
        .expect("learning contents array");
    assert_eq!(contents[0]["id"].as_i64(), Some(i64::from(text_content_id)));
    assert_eq!(contents[0]["display_state"].as_str(), Some("ready"));
    assert_eq!(contents[0]["data"].as_str(), Some("Welcome to ownership."));
    assert_eq!(
        contents[1]["id"].as_i64(),
        Some(i64::from(video_content_id))
    );
    assert_eq!(
        contents[1]["display_state"].as_str(),
        Some("failed_processing")
    );
    assert_eq!(contents[1]["processing_status"].as_str(), Some("failed"));
    assert_eq!(
        contents[1]["processing_error"].as_str(),
        Some("ffmpeg failed during thumbnail extraction")
    );
    assert_eq!(contents[2]["id"].as_i64(), Some(i64::from(empty_video_id)));
    assert_eq!(
        contents[2]["display_state"].as_str(),
        Some("unprocessed_upload")
    );

    let denied_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);

    let preview_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(auditor.id())),
        ))
        .to_request();
    let preview_resp = test::call_service(&app, preview_req).await;
    assert_eq!(preview_resp.status(), StatusCode::OK);
    let preview_body: Value = test::read_body_json(preview_resp).await;
    assert_eq!(
        preview_body["course"]["enrollment"]["state"].as_str(),
        Some("available")
    );
    assert_eq!(preview_body["progress_supported"].as_bool(), Some(false));
}
