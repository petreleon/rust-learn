struct PlatformFraudDashboardExpectation {
    admin_id: i32,
    moderator_id: i32,
    stranger_id: i32,
    active_block_ids: [i64; 4],
    expired_block_id: i64,
}

async fn assert_platform_fraud_dashboard(
    pool: &DbPool,
    expected: PlatformFraudDashboardExpectation,
) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(platform_fraud_dashboard_use_case(pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::reporting::configure_routes),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.stranger_id)),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.moderator_id)),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["active_total"].as_i64().unwrap_or_default() >= 4);
    assert!(
        body["active_by_scope"]["teacher"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["organization"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["course"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(
        body["active_by_scope"]["reward_policy"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );

    let active_blocks = body["active_blocks"]
        .as_array()
        .expect("active fraud block rows");
    for block_id in expected.active_block_ids {
        assert!(
            active_blocks
                .iter()
                .any(|row| row["id"].as_i64() == Some(block_id)),
            "active fraud dashboard should include block {block_id}"
        );
    }
    assert!(
        !active_blocks
            .iter()
            .any(|row| row["id"].as_i64() == Some(expected.expired_block_id)),
        "expired fraud block must not appear in active fraud dashboard"
    );

    let denied_export_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.moderator_id)),
        ))
        .to_request();
    let denied_export_status = match app.call(denied_export_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(denied_export_status, StatusCode::FORBIDDEN);

    let export_req = test::TestRequest::get()
        .uri("/reports/platform/fraud-dashboard.csv")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.admin_id)),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("fraud_blocks,active_total,"));
    assert!(csv.contains("active_fraud_blocks,"));
    assert!(csv.contains("teacher approvals paused for review"));
    assert!(!csv.contains("expired teacher block"));
}
