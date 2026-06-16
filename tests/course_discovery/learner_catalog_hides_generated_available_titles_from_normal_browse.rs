use crate::{create_organization::*, support::*};

#[actix_web::test]
async fn learner_catalog_hides_generated_available_titles_from_normal_browse() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, learner.id(), "USER").await;

    let org = create_organization(&mut conn, &unique_string("CatalogNoiseOrg")).await;
    let visible = create_course(&mut conn, "Rust Ownership Systems").await;
    let generated = create_course(&mut conn, &unique_string("LifecycleCourse")).await;
    publish_course(&mut conn, visible.id).await;
    publish_course(&mut conn, generated.id).await;
    link_course_to_org(&mut conn, visible.id, org.id, 0).await;
    link_course_to_org(&mut conn, generated.id, org.id, 1).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(learner_course_catalog_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/catalog?organization_id={}&limit=10",
            org.id
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
    assert_eq!(
        body["courses"][0]["title"].as_str(),
        Some(visible.title.as_str())
    );

    let search_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/catalog?organization_id={}&search=LifecycleCourse&limit=10",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let search_resp = test::call_service(&app, search_req).await;
    assert_eq!(search_resp.status(), StatusCode::OK);

    let search_body: Value = test::read_body_json(search_resp).await;
    assert_eq!(search_body["total"].as_i64(), Some(1));
    assert_eq!(
        search_body["courses"][0]["title"].as_str(),
        Some(generated.title.as_str())
    );
}
