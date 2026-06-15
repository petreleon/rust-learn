use crate::support::*;

#[actix_web::test]
async fn organization_member_invite_adds_member_and_logs_audit() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let admin = create_test_user(&mut conn, "member_invite_admin").await;
    let target = create_test_user(&mut conn, "member_invite_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberInviteOrg")).await;
    assign_organization_role(&mut conn, admin.id(), org.id, "ADMIN").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_member_invite_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/organizations/{}/members", org.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(serde_json::json!({
            "email": format!(" {} ", target.email),
            "role_name": "STUDENT"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["user_id"].as_i64(), Some(i64::from(target.id())));
    assert_eq!(body["email"].as_str(), Some(target.email.as_str()));
    assert_eq!(body["role"].as_str(), Some("STUDENT"));

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 1);
    let event = organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(org.id))
        .filter(organization_member_audit_events::target_user_id.eq(target.id()))
        .filter(organization_member_audit_events::event_type.eq("role_assigned"))
        .first::<OrganizationMemberAuditEvent>(&mut conn)
        .await
        .expect("member invite audit event should be recorded");
    assert_eq!(event.actor_user_id, Some(admin.id()));
    assert_eq!(event.role_name.as_deref(), Some("STUDENT"));
}

#[actix_web::test]
async fn organization_member_invite_denies_users_without_invite_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "member_invite_outsider").await;
    let target = create_test_user(&mut conn, "member_invite_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("MemberInviteDeniedOrg")).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_member_invite_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/organizations/{}/members", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .set_json(serde_json::json!({
            "email": target.email,
            "role_name": "STUDENT"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(member_role_count(&mut conn, org.id, target.id()).await, 0);
}
