use crate::db::schema::wallet_token_deposit_intents;
use crate::domain::wallet::deposit::WALLET_DEPOSIT_STATUS_PENDING;
use crate::models::wallet_token_deposit_intent::{
    NewWalletTokenDepositIntent, WalletTokenDepositIntent,
};
use bigdecimal::BigDecimal;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use super::apply_wallet_token_ledger_entries::{
    addresses_equal, normalize_address, wallet_interaction_for_transfer,
};
use super::configured_deposit_platform_address::configured_deposit_platform_address;
use super::link_organization_wallet::get_wallet_token_tax;
use super::support::{
    WalletTokenDepositIntentResponse, WalletTokenGasPayer, WalletTokenOperation,
    WalletTokenTransferError, WalletTokenTransferRequest,
};
use super::validate_positive_amount::{
    validate_external_transaction_fields, validate_positive_amount,
    validate_transfer_request_addresses,
};
use super::wallet_token_helpers::link_user_wallet;

pub(super) async fn create_user_wallet_token_deposit_intent(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: WalletTokenTransferRequest,
) -> Result<WalletTokenDepositIntentResponse, WalletTokenTransferError> {
    validate_positive_amount(&request.amount, "amount")?;
    validate_transfer_request_addresses(&request)?;
    validate_external_transaction_fields(&request)?;

    let gas_payer = WalletTokenGasPayer::parse(&request.gas_payer)?;
    let configured_tax = match gas_payer {
        WalletTokenGasPayer::User => BigDecimal::from(0),
        WalletTokenGasPayer::Platform => {
            get_wallet_token_tax(conn, WalletTokenOperation::Deposit).await?
        }
    };

    if configured_tax > request.amount {
        return Err(WalletTokenTransferError::InvalidInput(
            "deposit amount must be greater than or equal to the platform-paid gas tax".to_string(),
        ));
    }

    let platform_address = configured_deposit_platform_address(conn, gas_payer).await?;
    if let Some(requested_platform_address) = request.platform_address.as_ref() {
        if !addresses_equal(requested_platform_address, &platform_address) {
            return Err(WalletTokenTransferError::InvalidInput(
                "platform_address does not match the configured deposit receiver".to_string(),
            ));
        }
    }
    let wallet_interaction =
        wallet_interaction_for_transfer(WalletTokenOperation::Deposit, gas_payer);

    let intent = conn
        .transaction::<_, WalletTokenTransferError, _>(|conn| {
            let platform_address = platform_address.clone();
            let request = request.clone();
            let configured_tax = configured_tax.clone();
            Box::pin(async move {
                let wallet = link_user_wallet(conn, user_id).await?.wallet;
                diesel::insert_into(wallet_token_deposit_intents::table)
                    .values(NewWalletTokenDepositIntent {
                        user_id,
                        wallet_id: wallet.id,
                        ethereum_address: normalize_address(&request.ethereum_address),
                        platform_address: normalize_address(&platform_address),
                        amount: request.amount.clone(),
                        gas_payer: gas_payer.as_str().to_string(),
                        tax_amount: configured_tax,
                        status: WALLET_DEPOSIT_STATUS_PENDING.to_string(),
                        chain_id: request.chain_id,
                        contract_address: request
                            .contract_address
                            .as_deref()
                            .map(normalize_address),
                        transaction_hash: request
                            .transaction_hash
                            .as_deref()
                            .map(|value| value.trim().to_ascii_lowercase()),
                        log_index: request.log_index,
                        wallet_provider: wallet_interaction.provider.to_string(),
                        metamask_required: wallet_interaction.metamask_required,
                        wallet_action: wallet_interaction.action.to_string(),
                    })
                    .get_result::<WalletTokenDepositIntent>(conn)
                    .await
                    .map_err(WalletTokenTransferError::from)
            })
        })
        .await?;

    log::info!(
        "event=wallet_token_deposit_intent_created intent_id={} user_id={} wallet_id={} amount={} tax_amount={} gas_payer={} platform_address={} metamask_required={}",
        intent.id,
        intent.user_id,
        intent.wallet_id,
        intent.amount,
        intent.tax_amount,
        intent.gas_payer,
        intent.platform_address,
        intent.metamask_required
    );

    Ok(WalletTokenDepositIntentResponse::from(intent))
}
