#[actix_web::test]
async fn current_session_returns_profile_scopes_and_delegations() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let user = create_test_user(&mut conn, "current_session_user").await;
    let grantor = create_test_user(&mut conn, "current_session_grantor").await;
    let organization = create_organization(&mut conn, &unique_string("CurrentSessionOrg")).await;
    let course = create_course(&mut conn, &unique_string("CurrentSessionCourse")).await;

    assign_platform_role(&mut conn, user.id(), "ADMIN").await;
    assign_organization_role(&mut conn, user.id(), organization.id, "ADMIN").await;
    assign_course_role(&mut conn, user.id(), course.id, "TEACHER").await;

    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: grantor.id(),
            grantee_user_id: user.id(),
            permission: "EXECUTE_REWARD_PAYOUT".to_string(),
            scope_type: "platform".to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("session API test".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create delegated permission");
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

    assert_eq!(body["user"]["id"].as_i64(), Some(user.id() as i64));
    assert_eq!(body["user"]["email_verified"].as_bool(), Some(true));
    assert!(array_contains(&body["platform"]["roles"], "ADMIN"));
    assert!(array_contains(
        &body["platform"]["direct_permissions"],
        "VIEW_REPORT"
    ));
    assert!(array_contains(
        &body["platform"]["delegated_permissions"],
        "EXECUTE_REWARD_PAYOUT"
    ));
    assert!(array_contains(
        &body["platform"]["effective_permissions"],
        "EXECUTE_REWARD_PAYOUT"
    ));

    let organizations = body["organizations"]
        .as_array()
        .expect("organizations array");
    let session_org = organizations
        .iter()
        .find(|item| item["id"].as_i64() == Some(organization.id as i64))
        .expect("session should include organization scope");
    assert_eq!(
        session_org["name"].as_str(),
        Some(organization.name.as_str())
    );
    assert!(array_contains(&session_org["roles"], "ADMIN"));
    assert!(array_contains(
        &session_org["direct_permissions"],
        "GENERATE_REPORT"
    ));

    let courses = body["courses"].as_array().expect("courses array");
    let session_course = courses
        .iter()
        .find(|item| item["id"].as_i64() == Some(course.id as i64))
        .expect("session should include course scope");
    assert_eq!(
        session_course["title"].as_str(),
        Some(course.title.as_str())
    );
    assert!(array_contains(&session_course["roles"], "TEACHER"));
    assert!(array_contains(
        &session_course["direct_permissions"],
        "CREATE_CONTENT"
    ));

    let delegations = body["delegated_permissions"]
        .as_array()
        .expect("delegated permissions array");
    assert!(delegations.iter().any(|delegation| {
        delegation["permission"].as_str() == Some("EXECUTE_REWARD_PAYOUT")
            && delegation["scope_type"].as_str() == Some("platform")
    }));
}

#[actix_web::test]
async fn current_session_requires_authorization() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(current_session_test_app(pool.clone())).await;

    let response =
        test::call_service(&app, test::TestRequest::get().uri("/api/me").to_request()).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["error"]["code"].as_str(), Some("unauthorized"));
}
