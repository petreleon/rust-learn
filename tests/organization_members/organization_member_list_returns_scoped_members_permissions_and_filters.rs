#[actix_web::test]
async fn organization_member_list_returns_scoped_members_permissions_and_filters() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let operator = create_test_user(&mut conn, "member_operator").await;
    let delegated_student = create_test_user(&mut conn, "delegated_student").await;
    let teacher = create_test_user(&mut conn, "member_teacher").await;
    let other_member = create_test_user(&mut conn, "other_member").await;
    let org = create_organization(&mut conn, &unique_string("MemberOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherMemberOrg")).await;

    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, delegated_student.id(), org.id, "STUDENT").await;
    assign_organization_role(&mut conn, teacher.id(), org.id, "TEACHER").await;
    assign_organization_role(&mut conn, other_member.id(), other_org.id, "STUDENT").await;
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: operator.id(),
            grantee_user_id: delegated_student.id(),
            permission: "VIEW_ORG_REWARD_REPORTS".to_string(),
            scope_type: "organization".to_string(),
            organization_id: Some(org.id),
            course_id: None,
            reason: Some("Temporary report review".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create delegated permission");
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
        .uri(&format!(
            "/organizations/{}/members?search=delegated&permission=VIEW_ORG_REWARD_REPORTS&limit=10",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(operator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["organization"]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(body["total"].as_i64(), Some(1));
    assert_eq!(body["limit"].as_i64(), Some(10));
    assert_eq!(body["search"].as_str(), Some("delegated"));
    assert_eq!(body["permission"].as_str(), Some("VIEW_ORG_REWARD_REPORTS"));
    assert_eq!(
        body["operator_permissions"]["can_view_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_invite_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_manage_members"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_assign_roles"].as_bool(),
        Some(false)
    );

    let member = &body["members"][0];
    assert_eq!(
        member["name"].as_str(),
        Some(delegated_student.name.as_str())
    );
    assert_eq!(
        member["email"].as_str(),
        Some(delegated_student.email.as_str())
    );
    assert!(array_contains(&member["roles"], "STUDENT"));
    assert!(array_contains(
        &member["direct_permissions"],
        "VIEW_ORGANIZATION"
    ));
    assert!(array_contains(
        &member["delegated_permissions"],
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert!(array_contains(
        &member["effective_permissions"],
        "VIEW_ORG_REWARD_REPORTS"
    ));
    assert_eq!(member["delegated_permission_count"].as_u64(), Some(1));
}
