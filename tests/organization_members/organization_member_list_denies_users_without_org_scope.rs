#[actix_web::test]
async fn organization_member_list_denies_users_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "member_outsider").await;
    let member = create_test_user(&mut conn, "member_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberDeniedOrg")).await;
    assign_organization_role(&mut conn, member.id(), org.id, "STUDENT").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(organization_member_list_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}/members", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
