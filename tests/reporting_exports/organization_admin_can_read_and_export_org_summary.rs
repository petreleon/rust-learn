#[actix_web::test]
async fn organization_admin_can_read_and_export_org_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "report_org_admin").await;
    let stranger = create_test_user(&mut conn, "report_stranger").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    create_org_wallet(&mut conn, org.id).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::reports::reports_scope()),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert_eq!(body["course_count"].as_i64(), Some(1));
    assert_eq!(body["member_count"].as_i64(), Some(1));
    assert_eq!(body["wallet_count"].as_i64(), Some(1));

    let req = test::TestRequest::get()
        .uri(&format!("/reports/organizations/{}/summary.csv", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let csv = String::from_utf8(test::read_body(resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("metric,value"));
    assert!(csv.contains("organization_name,"));
    assert!(csv.contains("courses,1"));
}
