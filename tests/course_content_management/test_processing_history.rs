use crate::content_lifecycle_helpers::*;
use crate::support::*;

#[actix_web::test]
async fn teacher_can_inspect_content_processing_history() {
    let fixture = setup_course_content_fixture().await;
    let CourseContentFixture {
        pool,
        course,
        teacher_token,
        student_token,
        ..
    } = fixture;
    let mut conn = setup_conn(&pool).await;
    let chapter = diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id: course.id,
            title: "Video chapter".to_string(),
            order: 1,
        })
        .get_result::<Chapter>(&mut conn)
        .await
        .expect("chapter should be created");
    let object_key = format!("courses/{}/chapters/{}/intro.mp4", course.id, chapter.id);
    let content = diesel::insert_into(rust_learn::infra::postgres::schema::contents::table)
        .values(NewContent {
            chapter_id: chapter.id,
            content_type: "video".to_string(),
            data: Some(object_key.clone()),
            order: 1,
        })
        .get_result::<Content>(&mut conn)
        .await
        .expect("content should be created");
    let job = diesel::insert_into(upload_jobs::table)
        .values(NewUploadJob {
            bucket: "course-materials",
            object: &object_key,
            user_id: None,
        })
        .get_result::<UploadJob>(&mut conn)
        .await
        .expect("job should be created");
    diesel::update(upload_jobs::table.find(job.id))
        .set((
            upload_jobs::status.eq("failed"),
            upload_jobs::attempts.eq(2),
            upload_jobs::last_error.eq(Some("transcode timed out")),
        ))
        .execute(&mut conn)
        .await
        .expect("job should be marked failed");
    drop(conn);

    let app = test::init_service(
        App::new()
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(processing_history_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;
    let uri = format!(
        "/courses/{}/chapters/{}/contents/{}/processing-history",
        course.id, chapter.id, content.id
    );
    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["content_id"], content.id);
    assert_eq!(body["object_key"], object_key);
    assert_eq!(body["jobs"][0]["status"], "failed");
    assert_eq!(body["jobs"][0]["attempts"], 2);
    assert_eq!(body["jobs"][0]["last_error"], "transcode timed out");

    let denied = test::TestRequest::get()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    assert_forbidden_response(
        app.call(denied).await,
        "Student inspected processing history!",
    );
}
