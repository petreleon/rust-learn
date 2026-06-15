use crate::assign_organization_role::*;
use crate::support::*;

#[actix_web::test]
async fn organization_dashboard_gates_sensitive_sections_for_basic_member() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardBasicOrg")).await;
    let student = create_test_user(&mut conn, "dashboard_basic_student").await;
    assign_organization_role(&mut conn, student.id(), org.id, "STUDENT").await;
    create_course(
        &mut conn,
        org.id,
        &unique_string("DashboardBasicCourse"),
        COURSE_STATUS_PUBLISHED,
    )
    .await;
    create_org_wallet(&mut conn, org.id, BigDecimal::from(50)).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_dashboard_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["members"]["available"].as_bool(), Some(true));
    assert_eq!(body["courses"]["available"].as_bool(), Some(true));
    assert_eq!(body["courses"]["published"].as_i64(), Some(1));
    assert_eq!(
        body["teacher_applications"]["available"].as_bool(),
        Some(false)
    );
    assert_eq!(body["rewards"]["available"].as_bool(), Some(false));
    assert_eq!(body["wallet"]["available"].as_bool(), Some(false));
    assert_eq!(
        body["operator_permissions"]["can_view_reports"].as_bool(),
        Some(false)
    );
    assert!(missing_permission_exists(
        &body,
        "rewards",
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert!(missing_permission_exists(
        &body,
        "teacher_applications",
        "VIEW_ORG_TEACHER_APPLICATIONS"
    ));
    assert!(missing_permission_exists(
        &body,
        "wallet",
        "MANAGE_ORG_WALLETS"
    ));
}

#[actix_web::test]
async fn organization_dashboard_denies_outsiders_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("DashboardDeniedOrg")).await;
    let outsider = create_test_user(&mut conn, "dashboard_outsider").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_dashboard_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/dashboard", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
