use crate::support::*;

#[actix_web::test]
async fn role_read_routes_require_view_role_assignments_permission() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "role_reader_admin").await;
    let stranger = create_test_user(&mut conn, "role_reader_stranger").await;

    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;

    let admin_token = generate_token(admin.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(permission_check_use_case_data(&pool))
            .app_data(role_catalog_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::access_control::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/roles")
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/roles")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let response = app.call(req).await.expect("admin role list should run");
    assert_eq!(response.status(), StatusCode::OK);
    let roles: serde_json::Value = test::read_body_json(response).await;
    assert!(role_list_contains(&roles, "SUPER_ADMIN"));

    let req = test::TestRequest::get()
        .uri("/roles/organization")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let response = app
        .call(req)
        .await
        .expect("admin organization role list should run");
    assert_eq!(response.status(), StatusCode::OK);
    let roles: serde_json::Value = test::read_body_json(response).await;
    assert!(role_list_contains(&roles, "ADMIN"));

    let req = test::TestRequest::get()
        .uri("/roles/course")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    let response = app
        .call(req)
        .await
        .expect("admin course role list should run");
    assert_eq!(response.status(), StatusCode::OK);
    let roles: serde_json::Value = test::read_body_json(response).await;
    assert!(role_list_contains(&roles, "TEACHER"));
}

fn role_list_contains(roles: &serde_json::Value, role_name: &str) -> bool {
    roles
        .as_array()
        .map(|items| {
            items
                .iter()
                .any(|role| role["name"].as_str() == Some(role_name))
        })
        .unwrap_or(false)
}
