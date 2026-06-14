#[actix_web::test]
async fn organization_member_role_assignment_assigns_role_and_logs_audit() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let admin = create_test_user(&mut conn, "member_role_admin").await;
    let target = create_test_user(&mut conn, "member_role_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberRoleOrg")).await;
    assign_organization_role(&mut conn, admin.id(), org.id, "ADMIN").await;
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: admin.id(),
            grantee_user_id: admin.id(),
            permission: "ASSIGN_ROLES_TO_ORG_USERS".to_string(),
            scope_type: "organization".to_string(),
            organization_id: Some(org.id),
            course_id: None,
            reason: Some("Route role assignment test".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create delegated assignment permission");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(organization_member_role_assignment_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/organizations/{}/users/{}/roles", org.id, target.id()))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 1);
    let event = organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(org.id))
        .filter(organization_member_audit_events::target_user_id.eq(target.id()))
        .filter(organization_member_audit_events::event_type.eq("role_assigned"))
        .first::<OrganizationMemberAuditEvent>(&mut conn)
        .await
        .expect("role assignment audit event should be recorded");
    assert_eq!(event.actor_user_id, Some(admin.id()));
    assert_eq!(event.role_name.as_deref(), Some("STUDENT"));
}

#[actix_web::test]
async fn organization_member_role_assignment_denies_users_without_assign_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "member_role_outsider").await;
    let target = create_test_user(&mut conn, "member_role_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberRoleDeniedOrg")).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(organization_member_role_assignment_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/organizations/{}/users/{}/roles", org.id, target.id()))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 0);
}
