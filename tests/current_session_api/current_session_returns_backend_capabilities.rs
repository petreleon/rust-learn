#[actix_web::test]
async fn current_session_returns_backend_capabilities() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let user = create_test_user(&mut conn, "current_session_capability_user").await;
    let grantor = create_test_user(&mut conn, "current_session_capability_grantor").await;
    let organization = create_organization(&mut conn, &unique_string("SessionCapabilityOrg")).await;
    let course = create_course(&mut conn, &unique_string("SessionCapabilityCourse")).await;

    assign_platform_role(&mut conn, user.id(), "ADMIN").await;
    assign_organization_role(&mut conn, user.id(), organization.id, "ADMIN").await;
    assign_course_role(&mut conn, user.id(), course.id, "TEACHER").await;
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: grantor.id(),
            grantee_user_id: user.id(),
            permission: "MANAGE_ORG_SETTINGS".to_string(),
            scope_type: "organization".to_string(),
            organization_id: Some(organization.id),
            course_id: None,
            reason: Some("session capability test".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create organization delegation");
    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: grantor.id(),
            grantee_user_id: user.id(),
            permission: "MANAGE_COURSE_SETTINGS".to_string(),
            scope_type: "course".to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("session course capability test".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create course delegation");
    drop(conn);

    let app = test::init_service(current_session_test_app(pool.clone())).await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me")
            .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;

    assert_eq!(body["access"]["learner"].as_bool(), Some(true));
    assert_eq!(body["access"]["teacher"].as_bool(), Some(true));
    assert_eq!(body["access"]["organization"].as_bool(), Some(true));
    assert_eq!(body["access"]["platform_admin"].as_bool(), Some(true));
    assert!(capability_enabled(&body["platform"]["capabilities"], "summary"));

    let session_org = body["organizations"]
        .as_array()
        .expect("organizations array")
        .iter()
        .find(|item| item["id"].as_i64() == Some(organization.id as i64))
        .expect("session should include organization scope");
    assert!(capability_enabled(&session_org["capabilities"], "settings"));
    assert!(capability_contains_permission(
        session_org,
        "reports",
        "GENERATE_REPORT"
    ));

    let session_course = body["courses"]
        .as_array()
        .expect("courses array")
        .iter()
        .find(|item| item["id"].as_i64() == Some(course.id as i64))
        .expect("session should include course scope");
    assert!(capability_enabled(&session_course["capabilities"], "teaching"));
    assert!(capability_contains_permission(
        session_course,
        "reward_status",
        "VIEW_COURSE_REWARD_STATUS"
    ));
}

fn capability_enabled(value: &Value, key: &str) -> bool {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .any(|item| item["key"].as_str() == Some(key) && item["enabled"].as_bool() == Some(true))
        })
        .unwrap_or(false)
}
