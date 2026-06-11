async fn assert_wallet_tax_permissions_and_update(
    pool: &DbPool,
    tax_admin_id: i32,
    deposit_tax_only_id: i32,
    stranger_id: i32,
    learner_id: i32,
) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_deposit_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/deposit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger_id)),
        ))
        .set_json(json!({ "tax_amount": "2" }))
        .to_request();
    let forbidden_deposit_tax_resp = test::call_service(&app, forbidden_deposit_tax_req).await;
    assert_eq!(forbidden_deposit_tax_resp.status(), StatusCode::FORBIDDEN);

    let forbidden_retire_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/retire")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(deposit_tax_only_id)),
        ))
        .set_json(json!({ "tax_amount": "1" }))
        .to_request();
    let forbidden_retire_tax_resp = test::call_service(&app, forbidden_retire_tax_req).await;
    assert_eq!(forbidden_retire_tax_resp.status(), StatusCode::FORBIDDEN);

    let set_deposit_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/deposit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(tax_admin_id)),
        ))
        .set_json(json!({ "tax_amount": "2" }))
        .to_request();
    let set_deposit_tax_resp = test::call_service(&app, set_deposit_tax_req).await;
    assert_eq!(set_deposit_tax_resp.status(), StatusCode::OK);
    let deposit_tax: Value = test::read_body_json(set_deposit_tax_resp).await;
    assert_eq!(deposit_tax["operation"], "deposit");
    assert_eq!(deposit_tax["tax_amount"], "2");

    let set_retire_tax_req = test::TestRequest::put()
        .uri("/api/wallets/token-taxes/retire")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(tax_admin_id)),
        ))
        .set_json(json!({ "tax_amount": "1" }))
        .to_request();
    let set_retire_tax_resp = test::call_service(&app, set_retire_tax_req).await;
    assert_eq!(set_retire_tax_resp.status(), StatusCode::OK);
    let retire_tax: Value = test::read_body_json(set_retire_tax_resp).await;
    assert_eq!(retire_tax["operation"], "retire");
    assert_eq!(retire_tax["tax_amount"], "1");

    let taxes_req = test::TestRequest::get()
        .uri("/api/wallets/token-taxes")
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner_id))))
        .to_request();
    let taxes_resp = test::call_service(&app, taxes_req).await;
    assert_eq!(taxes_resp.status(), StatusCode::OK);
    let taxes: Value = test::read_body_json(taxes_resp).await;
    assert_eq!(taxes["deposit"]["tax_amount"], "2");
    assert_eq!(taxes["retire"]["tax_amount"], "1");
}
