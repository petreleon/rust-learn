use crate::http_support::{token_for, wallet_test_app};
use crate::support::*;

pub(crate) struct UserWalletAuditExpectation {
    pub(crate) student_id: i32,
    pub(crate) wallet_id: i32,
    pub(crate) internal_transaction_id: i64,
    pub(crate) wallet_transaction_id: i64,
    pub(crate) external_transaction_id: i64,
    pub(crate) payout_transaction_id: i64,
    pub(crate) candidate_id: i64,
    pub(crate) payout_record_id: i64,
    pub(crate) credit_record_id: i64,
}

pub(crate) async fn assert_user_wallet_audit(pool: &DbPool, expected: UserWalletAuditExpectation) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let audit_req = test::TestRequest::get()
        .uri("/api/wallets/me/audit")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(expected.student_id)),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;

    assert_eq!(audit["wallet"]["id"], expected.wallet_id);
    assert_eq!(audit["wallet"]["owner_type"], "user");
    assert_eq!(audit["wallet"]["value"], "12");

    let internal = audit["internal_transactions"]
        .as_array()
        .expect("internal transaction audit rows");
    assert_eq!(internal.len(), 1);
    assert_eq!(
        internal[0]["internal_transaction_id"],
        expected.internal_transaction_id
    );
    assert_eq!(
        internal[0]["transaction_id"],
        expected.wallet_transaction_id
    );
    assert_eq!(internal[0]["transaction_type"], "reward_wallet_credit");
    assert_eq!(internal[0]["amount"], "12");

    let external = audit["external_transactions"]
        .as_array()
        .expect("external transaction audit rows");
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
    assert_eq!(external[0]["chain_id"], 31337);
    assert_eq!(external[0]["event_type"], "Transfer");

    let reward_records = audit["reward_records"]
        .as_array()
        .expect("reward audit rows");
    assert_eq!(reward_records.len(), 1);
    assert_eq!(
        reward_records[0]["reward_candidate_id"],
        expected.candidate_id
    );
    assert_eq!(
        reward_records[0]["candidate_status"],
        REWARD_STATUS_WALLET_CREDITED
    );
    assert_eq!(
        reward_records[0]["reconciliation_status"],
        "needs_notification"
    );
    assert_eq!(
        reward_records[0]["wallet_credit_record_id"],
        expected.credit_record_id
    );
    assert_eq!(
        reward_records[0]["wallet_credit_transaction_id"],
        expected.wallet_transaction_id
    );
    assert_eq!(
        reward_records[0]["internal_transaction_id"],
        expected.internal_transaction_id
    );
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
    assert!(reward_records[0]["notification_id"].is_null());
}
