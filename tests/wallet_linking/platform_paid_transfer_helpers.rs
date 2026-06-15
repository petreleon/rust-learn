use crate::http_support::{token_for, wallet_test_app};
use crate::platform_paid_audit_helpers::{
    assert_platform_paid_audit, assert_wallet_balance_endpoint, assert_wallet_balance_in_db,
};
use crate::support::*;

#[derive(Clone, Copy)]
pub(crate) struct PlatformPaidDeposit {
    pub(crate) wallet_id: i32,
    pub(crate) deposit_transaction_id: i64,
}

pub(crate) async fn create_and_credit_platform_paid_deposit(
    pool: &DbPool,
    learner_id: i32,
) -> PlatformPaidDeposit {
    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let deposit_tx_hash = format!(
        "0x{:064x}",
        ((std::process::id() as u128) << 64)
            | chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u128
    );
    let deposit_req = test::TestRequest::post()
        .uri("/api/wallets/me/deposits")
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner_id))))
        .set_json(json!({
            "amount": "20",
            "ethereum_address": "0x00000000000000000000000000000000000000aa",
            "platform_address": "0x00000000000000000000000000000000000000bb",
            "gas_payer": "platform",
            "chain_id": 31337,
            "contract_address": "0x00000000000000000000000000000000000000cc",
            "transaction_hash": deposit_tx_hash.clone(),
            "log_index": 0
        }))
        .to_request();
    let deposit_resp = test::call_service(&app, deposit_req).await;
    assert_eq!(deposit_resp.status(), StatusCode::CREATED);
    let deposit: Value = test::read_body_json(deposit_resp).await;
    assert_eq!(deposit["operation"], "deposit");
    assert_eq!(deposit["status"], "pending_chain_confirmation");
    assert_eq!(deposit["amount"], "20");
    assert_eq!(deposit["tax_amount"], "2");
    assert_eq!(deposit["wallet_delta_on_confirmation"], "18");
    assert_eq!(deposit["gas_payer"], "platform");
    assert_eq!(deposit["wallet_provider"], "metamask");
    assert_eq!(deposit["metamask_required"], true);
    assert_eq!(deposit["wallet_action"], "metamask_permit_signature");

    let deposit_intent_id = deposit["id"].as_i64().expect("deposit intent id");
    let wallet_id = deposit["wallet_id"].as_i64().expect("wallet id") as i32;
    let mut conn = setup_conn(pool).await;
    let pending_balance: BigDecimal = wallets::table
        .find(wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance query should succeed");
    assert_eq!(pending_balance, BigDecimal::from(0));

    let deposit_credit = credit_observed_wallet_deposit(
        &mut conn,
        ObservedWalletDepositEvent {
            chain_id: 31337,
            contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
            transaction_hash: deposit_tx_hash,
            log_index: 0,
            event_type: "import".to_string(),
            from_address: "0x00000000000000000000000000000000000000aa".to_string(),
            to_address: "0x00000000000000000000000000000000000000bb".to_string(),
            amount: BigDecimal::from(20),
        },
    )
    .await
    .expect("observed deposit should credit");
    assert!(deposit_credit.credited);
    assert_eq!(deposit_credit.intent_id, Some(deposit_intent_id));
    assert_eq!(deposit_credit.wallet_id, Some(wallet_id));
    assert_eq!(deposit_credit.status, "credited");

    PlatformPaidDeposit {
        wallet_id,
        deposit_transaction_id: deposit_credit
            .transaction_id
            .expect("deposit transaction id"),
    }
}

pub(crate) async fn assert_platform_paid_retirement_and_audit(
    pool: &DbPool,
    learner_id: i32,
    deposit: PlatformPaidDeposit,
) {
    let app = test::init_service(wallet_test_app(pool.clone())).await;
    let retire_req = test::TestRequest::post()
        .uri("/api/wallets/me/retirements")
        .insert_header(("Authorization", format!("Bearer {}", token_for(learner_id))))
        .set_json(json!({
            "amount": "5",
            "ethereum_address": "0x00000000000000000000000000000000000000dd",
            "platform_address": "0x00000000000000000000000000000000000000bb",
            "gas_payer": "platform",
            "chain_id": 31337,
            "contract_address": "0x00000000000000000000000000000000000000cc",
            "transaction_hash": unique_string("retire_tx"),
            "log_index": 1
        }))
        .to_request();
    let retire_resp = test::call_service(&app, retire_req).await;
    assert_eq!(retire_resp.status(), StatusCode::CREATED);
    let retire: Value = test::read_body_json(retire_resp).await;
    assert_eq!(retire["operation"], "retire");
    assert_eq!(retire["amount"], "5");
    assert_eq!(retire["tax_amount"], "1");
    assert_eq!(retire["wallet_delta"], "-6");
    assert_eq!(retire["gas_payer"], "platform");
    assert_eq!(retire["wallet_provider"], "platform");
    assert_eq!(retire["metamask_required"], false);
    assert_eq!(retire["wallet_action"], "platform_transfer");
    let retire_transaction_id = retire["transaction_id"]
        .as_i64()
        .expect("retire transaction id");

    assert_wallet_balance_endpoint(pool, learner_id, deposit.wallet_id).await;
    assert_platform_paid_audit(pool, learner_id, deposit, retire_transaction_id).await;
    assert_wallet_balance_in_db(pool, deposit.wallet_id).await;
}
