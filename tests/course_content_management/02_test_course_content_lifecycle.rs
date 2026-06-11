#[actix_web::test]
async fn test_course_content_lifecycle() {
    let fixture = setup_course_content_fixture().await;
    let CourseContentFixture {
        pool,
        course,
        other_chapter,
        teacher_token,
        student_token,
        outsider_token,
    } = fixture;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    // 1. Teacher CREATES Chapter (/courses/{id}/chapters)
    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Intro", "order": 1 }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(
        resp.status().is_success(),
        "Create Chapter failed: {}",
        resp.status()
    );

    let chapter: Chapter = test::read_body_json(resp).await;
    assert_eq!(chapter.title, "Intro");

    // 2. Student List Chapters (/courses/{id}/chapters)
    let req = test::TestRequest::get()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let chapters: Vec<Chapter> = test::read_body_json(resp).await;
    assert_eq!(chapters.len(), 1);

    // 3. Teacher CREATES Content (/courses/{id}/chapters/{cid}/contents)
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "text",
            "data": "Welcome to the course"
        }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(
        resp.status().is_success(),
        "Create Content failed: {}",
        resp.status()
    );
    let content: Content = test::read_body_json(resp).await;
    assert_eq!(content.data.unwrap(), "Welcome to the course");

    // 3b. Teacher cannot use their course permission with a chapter from another course.
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, other_chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "text",
            "data": "Cross-course write"
        }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    // 4. Student lists content through VIEW_CONTENT
    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let listed_content: Vec<Content> = test::read_body_json(resp).await;
    assert_eq!(listed_content.len(), 1);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, other_chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);

    // 5. User without course content permission cannot list content
    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", outsider_token)))
        .to_request();
    assert_forbidden_response(app.call(req).await, "Outsider listed content!");

    // 6. Student Cannot Create Content -> 403
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({
            "order": 2,
            "content_type": "video",
            "data": "hack"
        }))
        .to_request();
    // Use try_call check logic manually or app.call
    assert_forbidden_response(app.call(req).await, "Student created content!");

    // 7. Update Content (Teacher)
    let req = test::TestRequest::put()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "data": "Updated Text" }))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert!(resp.status().is_success());
    let updated_content: Content = test::read_body_json(resp).await;
    assert_eq!(updated_content.data.unwrap(), "Updated Text");

    let mut conn = setup_conn(&pool).await;
    let queued_text_jobs_before = upload_jobs::table
        .filter(upload_jobs::object.eq("Updated Text"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("upload job count query should succeed");
    drop(conn);

    // 8. Teacher cannot trigger video processing for text content
    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}/process",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let mut conn = setup_conn(&pool).await;
    let queued_text_jobs_after = upload_jobs::table
        .filter(upload_jobs::object.eq("Updated Text"))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("upload job count query should succeed");
    assert_eq!(queued_text_jobs_after, queued_text_jobs_before);
}
