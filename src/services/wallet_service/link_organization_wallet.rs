pub async fn link_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<LinkedWallet> {
    if let Some(wallet) = find_organization_wallet(conn, organization_id).await? {
        return Ok(LinkedWallet {
            wallet,
            created: false,
        });
    }

    let new_wallet = NewWallet {
        user_id: None,
        organization_id: Some(organization_id),
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(LinkedWallet {
            wallet,
            created: true,
        }),
        Err(error) if is_unique_violation(&error) => {
            let wallet = find_organization_wallet(conn, organization_id)
                .await?
                .ok_or(DieselError::NotFound)?;
            Ok(LinkedWallet {
                wallet,
                created: false,
            })
        }
        Err(error) => Err(error),
    }
}

pub async fn get_wallet_token_taxes(
    conn: &mut AsyncPgConnection,
) -> Result<WalletTokenTaxSettingsResponse, WalletTokenTransferError> {
    let deposit_tax = get_wallet_token_tax(conn, WalletTokenOperation::Deposit).await?;
    let retire_tax = get_wallet_token_tax(conn, WalletTokenOperation::Retire).await?;

    Ok(WalletTokenTaxSettingsResponse {
        deposit: wallet_token_tax_response(WalletTokenOperation::Deposit, deposit_tax),
        retire: wallet_token_tax_response(WalletTokenOperation::Retire, retire_tax),
    })
}

pub async fn set_wallet_token_tax_for_actor(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    operation: WalletTokenOperation,
    request: SetWalletTokenTaxRequest,
) -> Result<WalletTokenTaxResponse, WalletTokenTransferError> {
    ensure_can_set_wallet_token_tax(conn, actor_user_id, operation).await?;
    validate_non_negative_amount(&request.tax_amount, "tax_amount")?;

    set_persistent_state(conn, operation.tax_key(), &request.tax_amount.to_string()).await?;

    Ok(wallet_token_tax_response(operation, request.tax_amount))
}

pub async fn deposit_tokens_to_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: WalletTokenTransferRequest,
) -> Result<WalletTokenDepositIntentResponse, WalletTokenTransferError> {
    ensure_user_kyc_verified(conn, user_id).await?;
    create_user_wallet_token_deposit_intent(conn, user_id, request).await
}

pub async fn retire_tokens_from_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: WalletTokenTransferRequest,
) -> Result<WalletTokenTransferResponse, WalletTokenTransferError> {
    ensure_user_kyc_verified(conn, user_id).await?;
    execute_user_wallet_token_transfer(conn, user_id, WalletTokenOperation::Retire, request).await
}

async fn ensure_can_set_wallet_token_tax(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    operation: WalletTokenOperation,
) -> Result<(), WalletTokenTransferError> {
    let permission = operation.set_tax_permission().to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission).await? {
        Ok(())
    } else {
        Err(WalletTokenTransferError::PermissionDenied(permission))
    }
}

async fn get_wallet_token_tax(
    conn: &mut AsyncPgConnection,
    operation: WalletTokenOperation,
) -> Result<BigDecimal, WalletTokenTransferError> {
    let Some(value) = get_persistent_state(conn, operation.tax_key()).await? else {
        return Ok(BigDecimal::from(0));
    };

    let amount = BigDecimal::from_str(value.trim()).map_err(|_| {
        WalletTokenTransferError::Database(format!(
            "invalid stored {} token tax amount",
            operation.as_str()
        ))
    })?;
    validate_non_negative_amount(&amount, "stored tax amount")?;
    Ok(amount)
}

fn wallet_token_tax_response(
    operation: WalletTokenOperation,
    tax_amount: BigDecimal,
) -> WalletTokenTaxResponse {
    WalletTokenTaxResponse {
        operation: operation.as_str().to_string(),
        tax_amount: tax_amount.to_string(),
    }
}

impl From<WalletTokenDepositIntent> for WalletTokenDepositIntentResponse {
    fn from(intent: WalletTokenDepositIntent) -> Self {
        WalletTokenDepositIntentResponse {
            operation: TOKEN_TRANSFER_OPERATION_DEPOSIT.to_string(),
            id: intent.id,
            status: intent.status,
            wallet_id: intent.wallet_id,
            amount: intent.amount.to_string(),
            tax_amount: intent.tax_amount.to_string(),
            wallet_delta_on_confirmation: (intent.amount - intent.tax_amount).to_string(),
            gas_payer: intent.gas_payer,
            ethereum_address: intent.ethereum_address,
            platform_address: intent.platform_address,
            chain_id: intent.chain_id,
            contract_address: intent.contract_address,
            transaction_hash: intent.transaction_hash,
            log_index: intent.log_index,
            wallet_provider: intent.wallet_provider,
            metamask_required: intent.metamask_required,
            wallet_action: intent.wallet_action,
        }
    }
}
