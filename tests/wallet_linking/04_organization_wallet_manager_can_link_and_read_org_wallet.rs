#[actix_web::test]
async fn organization_wallet_manager_can_link_and_read_org_wallet() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "wallet_org_admin").await;
    let org_moderator = create_test_user(&mut conn, "wallet_org_moderator").await;
    let org_reporter = create_test_user(&mut conn, "wallet_org_reporter").await;
    let stranger = create_test_user(&mut conn, "wallet_org_stranger").await;
    let org = create_test_organization(&mut conn).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, org_moderator.id(), org.id, "MODERATOR").await;
    assign_organization_permission_role(
        &mut conn,
        org_reporter.id(),
        org.id,
        Permissions::VIEW_ORG_REWARD_REPORTS,
    )
    .await;
    drop(conn);

    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_resp = test::call_service(&app, forbidden_req).await;
    assert_eq!(forbidden_resp.status(), StatusCode::FORBIDDEN);

    let moderator_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_moderator.id())),
        ))
        .to_request();
    let moderator_resp = test::call_service(&app, moderator_req).await;
    assert_eq!(moderator_resp.status(), StatusCode::FORBIDDEN);

    let reporter_link_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_reporter.id())),
        ))
        .to_request();
    let reporter_link_resp = test::call_service(&app, reporter_link_req).await;
    assert_eq!(reporter_link_resp.status(), StatusCode::FORBIDDEN);

    let link_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let link_resp = test::call_service(&app, link_req).await;
    assert_eq!(link_resp.status(), StatusCode::CREATED);
    let first: Value = test::read_body_json(link_resp).await;
    assert_eq!(first["created"], true);
    assert_eq!(first["wallet"]["owner_type"], "organization");
    assert_eq!(first["wallet"]["organization_id"], org.id);
    assert!(first["wallet"]["user_id"].is_null());
    let wallet_id = first["wallet"]["id"].as_i64().expect("wallet id");

    let duplicate_req = test::TestRequest::post()
        .uri(&format!("/api/wallets/organizations/{}/link", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let duplicate_resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(duplicate_resp.status(), StatusCode::OK);
    let duplicate: Value = test::read_body_json(duplicate_resp).await;
    assert_eq!(duplicate["created"], false);
    assert_eq!(duplicate["wallet"]["id"], wallet_id);

    let get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), StatusCode::OK);
    let fetched: Value = test::read_body_json(get_resp).await;
    assert_eq!(fetched["id"], wallet_id);
    assert_eq!(fetched["organization_id"], org.id);

    let reporter_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_reporter.id())),
        ))
        .to_request();
    let reporter_get_resp = test::call_service(&app, reporter_get_req).await;
    assert_eq!(reporter_get_resp.status(), StatusCode::OK);
    let reporter_fetched: Value = test::read_body_json(reporter_get_resp).await;
    assert_eq!(reporter_fetched["id"], wallet_id);

    let moderator_get_req = test::TestRequest::get()
        .uri(&format!("/api/wallets/organizations/{}", org.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_moderator.id())),
        ))
        .to_request();
    let moderator_get_resp = test::call_service(&app, moderator_get_req).await;
    assert_eq!(moderator_get_resp.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    let count: i64 = wallets::table
        .filter(wallets::organization_id.eq(org.id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("wallet count query should succeed");
    assert_eq!(count, 1);
}
