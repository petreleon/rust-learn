#[actix_web::test]
async fn delegated_permission_api_grants_lists_and_revokes_reward_permissions() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "delegate_api_admin").await;
    let moderator = create_test_user(&mut conn, "delegate_api_moderator").await;
    let grantee = create_test_user(&mut conn, "delegate_api_grantee").await;
    assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(web::Data::new(delegated_permission_use_case(&pool)))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::access_control::configure_routes),
    )
    .await;

    let grant_body = json!({
        "grantee_user_id": grantee.id(),
        "permission": Permissions::APPROVE_REWARD_AMOUNT.to_string(),
        "scope_type": "platform",
        "reason": "temporary reward amount review"
    });

    let denied_req = test::TestRequest::post()
        .uri("/delegated-permissions")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .set_json(&grant_body)
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);

    let grant_req = test::TestRequest::post()
        .uri("/delegated-permissions")
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(&grant_body)
        .to_request();
    let grant_resp = test::call_service(&app, grant_req).await;
    assert_eq!(grant_resp.status(), StatusCode::CREATED);
    let granted: Value = test::read_body_json(grant_resp).await;
    let delegation_id = granted["id"].as_i64().expect("delegation id");
    assert_eq!(
        granted["grantee_user_id"].as_i64(),
        Some(i64::from(grantee.id()))
    );
    assert_eq!(
        granted["permission"],
        Permissions::APPROVE_REWARD_AMOUNT.to_string()
    );
    assert_eq!(granted["scope_type"], "platform");

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/delegated-permissions?active=true&grantee_user_id={}",
            grantee.id()
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let listed: Value = test::read_body_json(list_resp).await;
    assert!(listed
        .as_array()
        .expect("delegation list")
        .iter()
        .any(|delegation| delegation["id"].as_i64() == Some(delegation_id)));

    let revoke_req = test::TestRequest::put()
        .uri(&format!("/delegated-permissions/{delegation_id}/revoke"))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(json!({ "revoke_reason": "assignment rotated" }))
        .to_request();
    let revoke_resp = test::call_service(&app, revoke_req).await;
    assert_eq!(revoke_resp.status(), StatusCode::OK);
    let revoked: Value = test::read_body_json(revoke_resp).await;
    assert_eq!(
        revoked["revoked_by_user_id"].as_i64(),
        Some(i64::from(admin.id()))
    );
    assert_eq!(revoked["revoke_reason"], "assignment rotated");
}
