#[actix_web::test]
async fn organization_member_removal_removes_member_and_logs_audit() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let admin = create_test_user(&mut conn, "member_remove_admin").await;
    let target = create_test_user(&mut conn, "member_remove_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberRemovalOrg")).await;
    assign_organization_role(&mut conn, admin.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, target.id(), org.id, "STUDENT").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(organization_member_removal_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/organizations/{}/users/{}", org.id, target.id()))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 0);
    let event = organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(org.id))
        .filter(organization_member_audit_events::target_user_id.eq(target.id()))
        .filter(organization_member_audit_events::event_type.eq("member_removed"))
        .first::<OrganizationMemberAuditEvent>(&mut conn)
        .await
        .expect("member removal audit event should be recorded");
    assert_eq!(event.actor_user_id, None);
}

#[actix_web::test]
async fn organization_member_removal_denies_users_without_manage_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "member_remove_outsider").await;
    let target = create_test_user(&mut conn, "member_remove_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberRemovalDeniedOrg")).await;
    assign_organization_role(&mut conn, target.id(), org.id, "STUDENT").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(organization_member_removal_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/organizations/{}/users/{}", org.id, target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 1);
}
