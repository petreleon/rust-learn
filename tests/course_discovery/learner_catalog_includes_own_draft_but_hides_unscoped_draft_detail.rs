use crate::{create_organization::*, support::*};

#[actix_web::test]
async fn learner_catalog_includes_own_draft_but_hides_unscoped_draft_detail() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let outsider = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, learner.id(), "USER").await;
    assign_platform_role(&mut conn, outsider.id(), "USER").await;

    let draft = create_course(&mut conn, &unique_string("OwnDraftCourse")).await;
    assign_course_role(&mut conn, learner.id(), draft.id, "STUDENT").await;
    let chapter_id = create_chapter(&mut conn, draft.id, "Draft module", 0).await;
    create_content(&mut conn, chapter_id, "article", 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(learner_course_catalog_use_case_data(&pool))
            .app_data(learner_course_detail_use_case_data(&pool))
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let catalog_req = test::TestRequest::get()
        .uri("/courses/catalog?enrollment_status=enrolled&limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let catalog_resp = test::call_service(&app, catalog_req).await;
    assert_eq!(catalog_resp.status(), StatusCode::OK);
    let catalog_body: Value = test::read_body_json(catalog_resp).await;
    let courses = catalog_body["courses"].as_array().expect("courses array");
    let own_draft = courses
        .iter()
        .find(|course| course["id"].as_i64() == Some(i64::from(draft.id)))
        .expect("own draft should be visible through course role");
    assert_eq!(own_draft["enrollment"]["state"].as_str(), Some("enrolled"));
    assert_eq!(
        own_draft["content"]["content_types"][0].as_str(),
        Some("article")
    );

    let detail_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}", draft.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let detail_resp = test::call_service(&app, detail_req).await;
    assert_eq!(detail_resp.status(), StatusCode::OK);
    let detail_body: Value = test::read_body_json(detail_resp).await;
    assert_eq!(
        detail_body["course"]["id"].as_i64(),
        Some(i64::from(draft.id))
    );
    assert_eq!(
        detail_body["chapters"][0]["contents"][0]["content_type"].as_str(),
        Some("article")
    );

    let outsider_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}", draft.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let outsider_resp = test::call_service(&app, outsider_req).await;
    assert_eq!(outsider_resp.status(), StatusCode::NOT_FOUND);
}
