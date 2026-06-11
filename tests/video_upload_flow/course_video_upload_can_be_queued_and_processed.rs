#[actix_web::test]
async fn course_video_upload_can_be_queued_and_processed() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let s3 = S3State::new_from_env().await.expect("init s3");
    s3.health_check()
        .await
        .expect("S3 must be reachable through S3_INTERNAL_* to run this test");
    let notifications = NotificationsState::new(pool.clone());

    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher_video_upload").await;
    let course = diesel::insert_into(courses::table)
        .values(&NewCourse {
            title: unique_string("VideoUploadCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result::<Course>(&mut conn)
        .await
        .expect("course should be created");
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let teacher_token = create_jwt(teacher.id()).expect("failed to generate token");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(s3.clone()))
            .app_data(web::Data::new(notifications.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/courses/{}/chapters", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "title": "Video chapter",
            "order": 1
        }))
        .to_request();
    let resp = app.call(req).await.expect("chapter request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let chapter: Chapter = test::read_body_json(resp).await;

    let (sample_path, filename) = create_sample_video_file();

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/upload_url",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "filename": filename,
            "content_type": "video/mp4"
        }))
        .to_request();
    let resp = app.call(req).await.expect("upload URL request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let upload: UploadUrlResponse = test::read_body_json(resp).await;

    put_sample_video(&sample_path, &upload.upload_url).await;

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "order": 1,
            "content_type": "video",
            "data": upload.object_key
        }))
        .to_request();
    let resp = app
        .call(req)
        .await
        .expect("content create request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    let content: Content = test::read_body_json(resp).await;
    assert_eq!(content.content_type, "video");
    assert_eq!(content.data.as_deref(), Some(upload.object_key.as_str()));

    let req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents/{}/process",
            course.id, chapter.id, content.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .to_request();
    let resp = app.call(req).await.expect("process request should run");
    assert_eq!(resp.status(), actix_web::http::StatusCode::ACCEPTED);

    let mut conn = setup_conn(&pool).await;
    let job = upload_jobs::table
        .filter(upload_jobs::object.eq(&upload.object_key))
        .order(upload_jobs::id.desc())
        .first::<UploadJob>(&mut conn)
        .await
        .expect("processing endpoint should create an upload job");
    assert_eq!(job.bucket, "course-materials");
    assert_eq!(job.user_id, Some(teacher.id()));

    let use_running_worker = std::env::var("VIDEO_UPLOAD_TEST_USE_RUNNING_WORKER")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if use_running_worker {
        wait_for_running_worker(&mut conn, job.id(), StdDuration::from_secs(60)).await;
    } else {
        assert_eq!(job.status, "queued");
        diesel::update(upload_jobs::table.find(job.id()))
            .set((
                upload_jobs::status.eq("processing"),
                upload_jobs::updated_at.eq(Utc::now()),
            ))
            .execute(&mut conn)
            .await
            .expect("test should mark the job as claimed by the worker");

        s3.process_uploaded_video(
            "course-materials",
            &upload.object_key,
            teacher.id(),
            notifications.clone(),
        )
        .await
        .expect("worker video processing should succeed");
        UploadJob::mark_done(job.id(), &mut conn)
            .await
            .expect("processed job should be marked done");

        let updated_job = upload_jobs::table
            .find(job.id())
            .first::<UploadJob>(&mut conn)
            .await
            .expect("processed job should still exist");
        assert_eq!(updated_job.status, "done");
    }

    let processed_video = format!("processed/{}", upload.object_key);
    let processed_audio = format!(
        "processed/audio/{}.mp3",
        upload.object_key.replace('/', "_")
    );
    assert_s3_object_downloadable(&s3, &processed_video).await;
    assert_s3_object_downloadable(&s3, &processed_audio).await;

    let processed_notification_count = notifications::table
        .filter(notifications::user_id.eq(Some(teacher.id())))
        .filter(notifications::title.eq("video:processed"))
        .filter(notifications::body.like(format!("%{}%", upload.object_key)))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("processed notification query should succeed");
    assert!(processed_notification_count >= 1);

    let _ = tokio::fs::remove_file(sample_path).await;
}
