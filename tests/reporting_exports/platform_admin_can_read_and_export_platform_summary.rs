use crate::{link_course_to_org::*, reporting_app_data::*, support::*};

#[actix_web::test]
async fn platform_admin_can_read_and_export_platform_summary() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let platform_admin = create_test_user(&mut conn, "report_platform_admin").await;
    let stranger = create_test_user(&mut conn, "report_platform_stranger").await;
    assign_platform_role(&mut conn, platform_admin.id(), "ADMIN").await;
    let org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(platform_summary_use_case(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::reporting::configure_routes),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri("/reports/platform/summary")
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
        .uri("/reports/platform/summary")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["total_users"].as_i64().unwrap_or_default() >= 1);
    assert!(body["total_organizations"].as_i64().unwrap_or_default() >= 1);
    assert!(body["total_courses"].as_i64().unwrap_or_default() >= 1);

    let forbidden_export_req = test::TestRequest::get()
        .uri("/reports/platform/summary.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_export_status = match app.call(forbidden_export_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_export_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/reports/platform/summary.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(platform_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let csv = String::from_utf8(test::read_body(resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("metric,value"));
    assert!(csv.contains("courses,"));
}
