use crate::support::*;

#[actix_web::test]
async fn test_platform_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    // Setup Users
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "superadmin").await;
    let target_user = create_test_user(&mut conn, "target").await;
    let unprivileged = create_test_user(&mut conn, "unpriv").await;

    // Assign SUPER_ADMIN role (which has all perms)
    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;

    let admin_token = generate_token(admin.id());
    let unprivileged_token = generate_token(unprivileged.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(permission_check_use_case_data(&pool))
            .app_data(platform_role_assignment_use_case_data(&pool))
            .app_data(user_list_use_case_data(&pool))
            .app_data(user_profile_use_case_data(&pool))
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::identity::configure_routes),
    )
    .await;

    // 1. Unprivileged user tries to assign role -> Should Fail
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", unprivileged_token)))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();

    // middleware returns Err, so app.call() returns Err
    let result = app.call(req).await;
    match result {
        Ok(resp) => {
            // It might pass if logic changes, but we expect error or 403
            if resp.status().is_success() {
                panic!("Unprivileged user access succeeded unexpectedly");
            }
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
        Err(e) => {
            // Middleware error is returned here
            let resp = e.error_response();
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Admin user tries to assign role -> Should Succeed or pass middleware
    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", target_user.id()))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(serde_json::json!({ "role_name": "USER" }))
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => {
            // Admin > User, so assignment might succeed (200) or fail on logic details, but OK is expected
            assert!(
                resp.status().is_success(),
                "Admin request failed: status {}",
                resp.status()
            );
        }
        Err(e) => {
            panic!("Admin request returned error: {}", e);
        }
    }
}

#[actix_web::test]
async fn platform_role_assignment_enforces_hierarchy() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let super_admin = create_test_user(&mut conn, "platform_assigner_super").await;
    let equal_target = create_test_user(&mut conn, "platform_equal_target").await;
    let fresh_target = create_test_user(&mut conn, "platform_fresh_target").await;

    force_assign_platform_role(&mut conn, super_admin.id(), "SUPER_ADMIN").await;
    force_assign_platform_role(&mut conn, equal_target.id(), "SUPER_ADMIN").await;

    let super_admin_token = generate_token(super_admin.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(permission_check_use_case_data(&pool))
            .app_data(platform_role_assignment_use_case_data(&pool))
            .app_data(user_list_use_case_data(&pool))
            .app_data(user_profile_use_case_data(&pool))
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::identity::configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", fresh_target.id()))
        .insert_header(("Authorization", format!("Bearer {}", super_admin_token)))
        .set_json(serde_json::json!({ "role_name": "SUPER_ADMIN" }))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::post()
        .uri(&format!("/user/{}/role", equal_target.id()))
        .insert_header(("Authorization", format!("Bearer {}", super_admin_token)))
        .set_json(serde_json::json!({ "role_name": "USER" }))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);
}
