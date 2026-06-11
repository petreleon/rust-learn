async fn execute_user_wallet_token_transfer(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    operation: WalletTokenOperation,
    request: WalletTokenTransferRequest,
) -> Result<WalletTokenTransferResponse, WalletTokenTransferError> {
    validate_positive_amount(&request.amount, "amount")?;
    validate_transfer_request_addresses(&request)?;
    validate_external_transaction_fields(&request)?;

    let gas_payer = WalletTokenGasPayer::parse(&request.gas_payer)?;
    let configured_tax = match gas_payer {
        WalletTokenGasPayer::User => BigDecimal::from(0),
        WalletTokenGasPayer::Platform => get_wallet_token_tax(conn, operation).await?,
    };

    if operation == WalletTokenOperation::Deposit && configured_tax > request.amount {
        return Err(WalletTokenTransferError::InvalidInput(
            "deposit amount must be greater than or equal to the platform-paid gas tax".to_string(),
        ));
    }

    conn.transaction::<_, WalletTokenTransferError, _>(|conn| {
        Box::pin(async move {
            let wallet = link_user_wallet(conn, user_id).await?.wallet;
            let transaction_id = create_wallet_token_transaction(conn, operation).await?;
            let external_transaction_id = create_wallet_external_transaction(
                conn,
                operation,
                transaction_id,
                &request,
            )
            .await?;
            let internal_transaction_ids = apply_wallet_token_ledger_entries(
                conn,
                wallet.id,
                transaction_id,
                operation,
                request.amount.clone(),
                configured_tax.clone(),
            )
            .await?;
            let wallet_delta =
                wallet_delta_for_operation(operation, request.amount.clone(), configured_tax.clone());
            let wallet_interaction = wallet_interaction_for_transfer(operation, gas_payer);

            log::info!(
                "event=wallet_token_transfer operation={} user_id={} wallet_id={} amount={} tax_amount={} gas_payer={} wallet_provider={} metamask_required={} transaction_id={} external_transaction_id={}",
                operation.as_str(),
                user_id,
                wallet.id,
                request.amount,
                configured_tax,
                gas_payer.as_str(),
                wallet_interaction.provider,
                wallet_interaction.metamask_required,
                transaction_id,
                external_transaction_id
            );

            Ok(WalletTokenTransferResponse {
                operation: operation.as_str().to_string(),
                wallet_id: wallet.id,
                transaction_id,
                external_transaction_id,
                internal_transaction_ids,
                amount: request.amount.to_string(),
                tax_amount: configured_tax.to_string(),
                wallet_delta: wallet_delta.to_string(),
                gas_payer: gas_payer.as_str().to_string(),
                ethereum_address: request.ethereum_address.trim().to_string(),
                wallet_provider: wallet_interaction.provider.to_string(),
                metamask_required: wallet_interaction.metamask_required,
                wallet_action: wallet_interaction.action.to_string(),
            })
        })
    })
    .await
}
