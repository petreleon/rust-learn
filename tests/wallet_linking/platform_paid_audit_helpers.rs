use crate::http_support::{token_for, wallet_test_app};
use crate::platform_paid_transfer_helpers::PlatformPaidDeposit;
use crate::support::*;

pub(crate) async fn assert_wallet_balance_endpoint(pool: &DbPool, learner_id: i32, wallet_id: i32) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let wallet_req = test::TestRequest::get()
        .uri("/api/wallets/me")
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner_id))))
        .to_request();
    let wallet_resp = test::call_service(&app, wallet_req).await;
    assert_eq!(wallet_resp.status(), StatusCode::OK);
    let wallet: Value = test::read_body_json(wallet_resp).await;
    assert_eq!(wallet["id"], wallet_id);
    assert_eq!(wallet["value"], "12");
}

pub(crate) async fn assert_platform_paid_audit(
    pool: &DbPool,
    learner_id: i32,
    deposit: PlatformPaidDeposit,
    retire_transaction_id: i64,
) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let audit_req = test::TestRequest::get()
        .uri("/api/wallets/me/audit")
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner_id))))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;

    let internal = audit["internal_transactions"]
        .as_array()
        .expect("internal transaction audit rows");
    assert_eq!(internal.len(), 4);
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == deposit.deposit_transaction_id
            && row["transaction_type"] == "token_deposit"
            && row["amount"] == "20"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == deposit.deposit_transaction_id
            && row["transaction_type"] == "token_deposit"
            && row["amount"] == "-2"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["transaction_type"] == "token_retire"
            && row["amount"] == "-5"
    }));
    assert!(internal.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["transaction_type"] == "token_retire"
            && row["amount"] == "-1"
    }));

    let external = audit["external_transactions"]
        .as_array()
        .expect("external transaction audit rows");
    assert_eq!(external.len(), 2);
    assert!(external.iter().any(|row| {
        row["transaction_id"] == deposit.deposit_transaction_id
            && row["event_type"] == "import"
            && row["reward_candidate_id"].is_null()
    }));
    assert!(external.iter().any(|row| {
        row["transaction_id"] == retire_transaction_id
            && row["event_type"] == "transfer"
            && row["reward_candidate_id"].is_null()
    }));
}

pub(crate) async fn assert_wallet_balance_in_db(pool: &DbPool, wallet_id: i32) {
    let mut conn = setup_conn(pool).await;
    let balance: BigDecimal = wallets::table
        .find(wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance query should succeed");
    assert_eq!(balance, BigDecimal::from(12));
}
