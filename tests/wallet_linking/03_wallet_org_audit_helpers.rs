struct OrganizationWalletAuditExpectation {
    org_id: i32,
    wallet_id: i32,
    stranger_id: i32,
    reporter_id: i32,
    internal_transaction_id: i64,
    wallet_transaction_id: i64,
    external_transaction_id: i64,
    payout_transaction_id: i64,
    candidate_id: i64,
    payout_record_id: i64,
}

async fn assert_organization_wallet_audit(
    pool: &DbPool,
    expected: OrganizationWalletAuditExpectation,
) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let forbidden_req = test::TestRequest::get()
        .uri(&format!(
            "/api/wallets/organizations/{}/audit",
            expected.org_id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.stranger_id)),
        ))
        .to_request();
    let forbidden_resp = test::call_service(&app, forbidden_req).await;
    assert_eq!(forbidden_resp.status(), StatusCode::FORBIDDEN);

    let reporter_req = test::TestRequest::get()
        .uri(&format!(
            "/api/wallets/organizations/{}/audit",
            expected.org_id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.reporter_id)),
        ))
        .to_request();
    let reporter_resp = test::call_service(&app, reporter_req).await;
    assert_eq!(reporter_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(reporter_resp).await;

    assert_eq!(audit["wallet"]["id"], expected.wallet_id);
    assert_eq!(audit["wallet"]["owner_type"], "organization");
    assert_eq!(audit["wallet"]["organization_id"], expected.org_id);
    assert_eq!(audit["wallet"]["value"], "75");

    let internal = audit["internal_transactions"]
        .as_array()
        .expect("organization internal audit rows");
    assert_eq!(internal.len(), 1);
    assert_eq!(
        internal[0]["internal_transaction_id"],
        expected.internal_transaction_id
    );
    assert_eq!(
        internal[0]["transaction_id"],
        expected.wallet_transaction_id
    );
    assert_eq!(
        internal[0]["transaction_type"],
        "organization_budget_adjustment"
    );
    assert_eq!(internal[0]["amount"], "75");

    let external = audit["external_transactions"]
        .as_array()
        .expect("organization external audit rows");
    assert_eq!(external.len(), 1);
    assert_eq!(
        external[0]["external_transaction_id"],
        expected.external_transaction_id
    );
    assert_eq!(
        external[0]["transaction_id"],
        expected.payout_transaction_id
    );
    assert_eq!(external[0]["reward_candidate_id"], expected.candidate_id);
    assert_eq!(external[0]["amount"], "75");

    let reward_records = audit["reward_records"]
        .as_array()
        .expect("organization reward audit rows");
    assert_eq!(reward_records.len(), 1);
    assert_eq!(
        reward_records[0]["reward_candidate_id"],
        expected.candidate_id
    );
    assert_eq!(
        reward_records[0]["candidate_status"],
        REWARD_STATUS_TOKEN_CONFIRMED
    );
    assert_eq!(
        reward_records[0]["reconciliation_status"],
        "needs_wallet_credit"
    );
    assert_eq!(reward_records[0]["approved_amount"], "75");
    assert_eq!(
        reward_records[0]["payout_record_id"],
        expected.payout_record_id
    );
    assert_eq!(
        reward_records[0]["payout_transaction_id"],
        expected.payout_transaction_id
    );
    assert_eq!(
        reward_records[0]["external_transaction_id"],
        expected.external_transaction_id
    );
    assert!(reward_records[0]["wallet_credit_record_id"].is_null());
}
