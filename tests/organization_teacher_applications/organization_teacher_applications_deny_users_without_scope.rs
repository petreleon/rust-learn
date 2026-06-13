#[actix_web::test]
async fn organization_teacher_applications_deny_users_without_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("TeacherAppsDeniedOrg")).await;
    let operator = create_test_user(&mut conn, "teacher_apps_denied_operator").await;
    let applicant = create_test_user(&mut conn, "teacher_apps_denied_applicant").await;
    let outsider = create_test_user(&mut conn, "teacher_apps_denied_outsider").await;
    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    nominate_application(
        &mut conn,
        operator.id(),
        org.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Denied fixture candidate.".to_string(),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("org operator should nominate candidate");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/teacher-applications", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
