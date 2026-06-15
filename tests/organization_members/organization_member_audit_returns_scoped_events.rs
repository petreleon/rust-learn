use crate::support::*;

#[actix_web::test]
async fn organization_member_audit_returns_scoped_events() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let operator = create_test_user(&mut conn, "audit_operator").await;
    let target = create_test_user(&mut conn, "audit_target").await;
    let other_target = create_test_user(&mut conn, "audit_other_target").await;
    let org = create_organization(&mut conn, &unique_string("AuditOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherAuditOrg")).await;

    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, target.id(), org.id, "STUDENT").await;
    assign_organization_role(&mut conn, other_target.id(), other_org.id, "STUDENT").await;

    insert_audit_event(
        &mut conn,
        org.id,
        Some(operator.id()),
        target.id(),
        "role_assigned",
        Some("STUDENT"),
        Some("Initial enrollment"),
    )
    .await;
    insert_audit_event(
        &mut conn,
        org.id,
        Some(operator.id()),
        target.id(),
        "member_invited",
        None,
        Some("Manual invitation"),
    )
    .await;
    insert_audit_event(
        &mut conn,
        other_org.id,
        Some(operator.id()),
        other_target.id(),
        "role_assigned",
        Some("STUDENT"),
        Some("Excluded by organization"),
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_member_audit_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/organizations/{}/members/{}/audit",
            org.id,
            target.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(operator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    let events = body.as_array().expect("audit response should be an array");
    assert_eq!(events.len(), 2);
    assert!(events.iter().any(|event| {
        event["event_type"].as_str() == Some("role_assigned")
            && event["role_name"].as_str() == Some("STUDENT")
            && event["target_user_id"].as_i64() == Some(i64::from(target.id()))
    }));
    assert!(events
        .iter()
        .any(|event| event["event_type"].as_str() == Some("member_invited")));
    assert!(!events
        .iter()
        .any(|event| event["reason"].as_str() == Some("Excluded by organization")));
}

#[actix_web::test]
async fn organization_member_audit_denies_users_without_org_scope() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let outsider = create_test_user(&mut conn, "audit_outsider").await;
    let target = create_test_user(&mut conn, "audit_denied_target").await;
    let org = create_organization(&mut conn, &unique_string("AuditDeniedOrg")).await;
    assign_organization_role(&mut conn, target.id(), org.id, "STUDENT").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_member_audit_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/organizations/{}/members/{}/audit",
            org.id,
            target.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

async fn insert_audit_event(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    actor_user_id: Option<i32>,
    target_user_id: i32,
    event_type: &str,
    role_name: Option<&str>,
    reason: Option<&str>,
) -> OrganizationMemberAuditEvent {
    diesel::insert_into(organization_member_audit_events::table)
        .values(NewOrganizationMemberAuditEvent {
            organization_id,
            actor_user_id,
            target_user_id,
            event_type: event_type.to_string(),
            role_name: role_name.map(ToString::to_string),
            reason: reason.map(ToString::to_string),
        })
        .get_result(conn)
        .await
        .expect("failed to insert organization member audit event")
}
