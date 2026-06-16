use crate::content_lifecycle_helpers::*;
use crate::support::*;

#[actix_web::test]
async fn teacher_updates_content_publication_status() {
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
            title: "Lifecycle".to_string(),
            order: 1,
        })
        .get_result::<Chapter>(&mut conn)
        .await
        .expect("chapter should be created");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(content_item_use_cases_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let create_req = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/chapters/{}/contents",
            course.id, chapter.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({
            "content_type": "text",
            "data": "Published lesson",
            "order": 1
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), actix_web::http::StatusCode::CREATED);
    let created: serde_json::Value = test::read_body_json(create_resp).await;
    assert_eq!(created["publication_status"].as_str(), Some("published"));
    let content_id = created["id"].as_i64().expect("content id") as i32;

    let uri = format!(
        "/courses/{}/chapters/{}/contents/{}",
        course.id, chapter.id, content_id
    );
    let unpublish_req = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "publication_status": "unpublished" }))
        .to_request();
    let unpublish_resp = test::call_service(&app, unpublish_req).await;
    assert_eq!(unpublish_resp.status(), actix_web::http::StatusCode::OK);
    let unpublished: serde_json::Value = test::read_body_json(unpublish_resp).await;
    assert_eq!(
        unpublished["publication_status"].as_str(),
        Some("unpublished")
    );

    let invalid_req = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "publication_status": "hidden" }))
        .to_request();
    assert_eq!(
        test::call_service(&app, invalid_req).await.status(),
        actix_web::http::StatusCode::BAD_REQUEST
    );

    let denied_req = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({ "publication_status": "published" }))
        .to_request();
    assert_forbidden_response(app.call(denied_req).await, "Student published content!");
}
