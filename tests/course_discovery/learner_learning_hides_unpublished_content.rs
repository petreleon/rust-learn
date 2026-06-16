use crate::{create_organization::*, support::*};

#[actix_web::test]
async fn learner_learning_hides_unpublished_content_and_progress() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_test_user(&mut conn).await;
    let course = create_course(&mut conn, &unique_string("UnpublishedLearning")).await;
    publish_course(&mut conn, course.id).await;
    assign_course_role(&mut conn, learner.id(), course.id, "STUDENT").await;
    let chapter_id = create_chapter(&mut conn, course.id, "Module", 0).await;
    let visible_id =
        create_content_with_data(&mut conn, chapter_id, "text", 0, Some("Visible")).await;
    let hidden_id =
        create_content_with_data(&mut conn, chapter_id, "text", 1, Some("Hidden")).await;
    diesel::update(contents::table.find(hidden_id))
        .set(contents::publication_status.eq("unpublished"))
        .execute(&mut conn)
        .await
        .expect("content should be unpublished");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(learner_course_learning_use_case_data(&pool))
            .app_data(learner_progress_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let token = token_for(learner.id());
    let learning_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let learning_resp = test::call_service(&app, learning_req).await;
    assert_eq!(learning_resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(learning_resp).await;
    assert_eq!(
        body["active_content_id"].as_i64(),
        Some(i64::from(visible_id))
    );
    let contents = body["chapters"][0]["contents"]
        .as_array()
        .expect("contents");
    let content_ids: Vec<i64> = contents
        .iter()
        .filter_map(|content| content["id"].as_i64())
        .collect();
    assert_eq!(content_ids, vec![i64::from(visible_id)]);

    let progress_req = test::TestRequest::post()
        .uri(&format!("/courses/{}/progress", course.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({ "content_id": hidden_id }))
        .to_request();
    assert_eq!(
        test::call_service(&app, progress_req).await.status(),
        StatusCode::NOT_FOUND
    );
}
