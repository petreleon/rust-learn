#[actix_web::test]
async fn test_organization_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    // Setup Data
    let mut conn = setup_conn(&pool).await;
    let owner = create_test_user(&mut conn, "org_superadmin").await;
    let stranger = create_test_user(&mut conn, "stranger").await;

    let new_org = NewOrganization {
        name: unique_string("TestOrg"),
        website_link: None,
        profile_url: None,
    };
    let org = diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result::<Organization>(&mut conn)
        .await
        .unwrap();

    // Assign ADMIN (Org scope) - Note: SUPERADMIN has permission sync issues in current migrations
    force_assign_org_role(&mut conn, owner.id(), org.id, "ADMIN").await;

    let owner_token = generate_token(owner.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    // 1. Stranger (no role) tries to UPDATE organization -> 403
    let req = test::TestRequest::put()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .set_json(serde_json::json!({ "name": "Hacked Org" }))
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN),
        Err(e) => {
            let resp = e.error_response();
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Owner tries to UPDATE organization -> Should pass
    let req = test::TestRequest::put()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(serde_json::json!({ "name": "Updated Org" }))
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => assert!(resp.status().is_success(), "Owner request failed"),
        Err(e) => panic!("Owner request returned error: {}", e),
    }
}
