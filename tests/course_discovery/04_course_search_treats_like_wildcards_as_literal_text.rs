#[actix_web::test]
async fn course_search_treats_like_wildcards_as_literal_text() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let viewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, viewer.id(), "STUDENT").await;

    let org = create_organization(&mut conn, &unique_string("WildcardOrg")).await;
    let prefix = unique_string("WildcardCourse").replace('_', "-");
    let percent_course = create_course(&mut conn, &format!("{} 100% Complete", prefix)).await;
    let underscore_course = create_course(&mut conn, &format!("{} data_set", prefix)).await;
    let plain_course = create_course(&mut conn, &format!("{} Plain Course", prefix)).await;

    link_course_to_org(&mut conn, percent_course.id, org.id, 0).await;
    link_course_to_org(&mut conn, underscore_course.id, org.id, 1).await;
    link_course_to_org(&mut conn, plain_course.id, org.id, 2).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;
    let token = token_for(viewer.id());

    let percent_req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=%25&organization_id={}&limit=10",
            org.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let percent_resp = test::call_service(&app, percent_req).await;
    assert_eq!(percent_resp.status(), StatusCode::OK);

    let percent_body: Value = test::read_body_json(percent_resp).await;
    assert_eq!(percent_body["total"].as_i64(), Some(1));
    assert_eq!(percent_body["search"].as_str(), Some("%"));
    assert_eq!(
        percent_body["courses"][0]["title"].as_str(),
        Some(percent_course.title.as_str())
    );

    let underscore_req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=_&organization_id={}&limit=10",
            org.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let underscore_resp = test::call_service(&app, underscore_req).await;
    assert_eq!(underscore_resp.status(), StatusCode::OK);

    let underscore_body: Value = test::read_body_json(underscore_resp).await;
    assert_eq!(underscore_body["total"].as_i64(), Some(1));
    assert_eq!(underscore_body["search"].as_str(), Some("_"));
    assert_eq!(
        underscore_body["courses"][0]["title"].as_str(),
        Some(underscore_course.title.as_str())
    );
}
