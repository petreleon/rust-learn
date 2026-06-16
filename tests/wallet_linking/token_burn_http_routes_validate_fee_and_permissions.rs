use crate::http_support::{assign_platform_permission_role, token_for, wallet_test_app};
use crate::support::*;

#[actix_web::test]
async fn token_burn_http_routes_validate_fee_paths_and_permissions() {
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "wallet_burn_http").await;
    mark_user_kyc_verified(&mut conn, user.id()).await;
    let org = create_test_organization(&mut conn).await;
    let app = test::init_service(wallet_test_app(pool.clone())).await;

    let invalid_burn = test::TestRequest::post()
        .uri("/api/wallets/me/burns")
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .set_json(json!({
            "amount": "4",
            "source": "decentralized_direct",
            "fee_path": "platform_deposit_fee",
            "idempotency_key": unique_string("invalid_burn"),
            "ethereum_address": "0x00000000000000000000000000000000000000ab",
            "chain_id": 31337,
            "contract_address": "0x00000000000000000000000000000000000000cd",
            "transaction_hash": "0xabc",
            "log_index": 0,
            "leaderboard_visible": true
        }))
        .to_request();
    let invalid_response = test::call_service(&app, invalid_burn).await;
    assert_eq!(invalid_response.status(), StatusCode::BAD_REQUEST);

    let org_permissions = test::TestRequest::get()
        .uri(&format!(
            "/api/wallets/organizations/{}/burns/permissions",
            org.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .to_request();
    let permission_response = test::call_service(&app, org_permissions).await;
    assert_eq!(permission_response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(permission_response).await;
    assert_eq!(body["required_permission"], "BURN_ORGANIZATION_TOKENS");
    assert_eq!(body["can_burn"], false);

    let denied_reconciliation = test::TestRequest::get()
        .uri("/api/wallets/burns/reconciliation")
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .to_request();
    let denied_response = test::call_service(&app, denied_reconciliation).await;
    assert_eq!(denied_response.status(), StatusCode::FORBIDDEN);

    assign_platform_permission_role(&mut conn, user.id(), Permissions::RECONCILE_TOKEN_BURNS).await;
    let allowed_reconciliation = test::TestRequest::get()
        .uri("/api/wallets/burns/reconciliation")
        .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
        .to_request();
    let allowed_response = test::call_service(&app, allowed_reconciliation).await;
    assert_eq!(allowed_response.status(), StatusCode::OK);
}
