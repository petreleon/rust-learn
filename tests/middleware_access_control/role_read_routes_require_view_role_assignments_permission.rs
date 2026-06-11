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
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::roles::roles_scope()),
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
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);
}
