use crate::support::*;

#[actix_web::test]
async fn duplicate_pending_deposit_intents_are_marked_ambiguous_without_crediting() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_test_user(&mut conn, "wallet_ambiguous_learner").await;
    mark_user_kyc_verified(&mut conn, learner.id()).await;

    set_persistent_state(
        &mut conn,
        "platform_importer_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to configure platform importer address");

    let request = WalletTokenTransferCommand {
        amount: BigDecimal::from(20),
        ethereum_address: "0x00000000000000000000000000000000000000aa".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: Some("0x00000000000000000000000000000000000000bb".to_string()),
    };

    let first = deposit_tokens_to_user_wallet(&mut conn, learner.id(), request.clone())
        .await
        .expect("first deposit intent should be created");
    let second = deposit_tokens_to_user_wallet(&mut conn, learner.id(), request)
        .await
        .expect("second deposit intent should be created");
    assert_ne!(first.id, second.id);
    assert_eq!(first.wallet_id, second.wallet_id);

    let tx_hash = unique_string("ambiguous_deposit_tx");
    let result = credit_observed_wallet_deposit(
        &mut conn,
        ObservedWalletDepositEvent {
            chain_id: 31337,
            contract_address: "0x00000000000000000000000000000000000000cc".to_string(),
            transaction_hash: tx_hash.clone(),
            log_index: 7,
            event_type: WalletDepositEventType::Import,
            from_address: "0x00000000000000000000000000000000000000aa".to_string(),
            to_address: "0x00000000000000000000000000000000000000bb".to_string(),
            amount: BigDecimal::from(20),
        },
    )
    .await
    .expect("ambiguous observed deposit should be handled");
    assert!(!result.credited);
    assert_eq!(result.status, WalletDepositStatus::Ambiguous);

    let intent_rows = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::id.eq_any([first.id, second.id]))
        .select((
            wallet_token_deposit_intents::status,
            wallet_token_deposit_intents::transaction_hash,
            wallet_token_deposit_intents::chain_id,
            wallet_token_deposit_intents::log_index,
        ))
        .load::<(String, Option<String>, Option<i64>, Option<i64>)>(&mut conn)
        .await
        .expect("deposit intents should be queryable");
    assert_eq!(intent_rows.len(), 2);
    for (status, stored_hash, chain_id, log_index) in intent_rows {
        assert_eq!(status, WalletDepositStatus::Ambiguous.as_str());
        assert_eq!(
            stored_hash.as_deref(),
            Some(tx_hash.to_ascii_lowercase().as_str())
        );
        assert_eq!(chain_id, Some(31337));
        assert_eq!(log_index, Some(7));
    }

    let wallet_balance: BigDecimal = wallets::table
        .find(first.wallet_id)
        .select(wallets::value)
        .get_result(&mut conn)
        .await
        .expect("wallet balance should be queryable");
    assert_eq!(wallet_balance, BigDecimal::from(0));
}
