use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    external_transactions, internal_transactions, transactions, transactions_external_transactions,
    transactions_internal_transactions, wallet_token_deposit_intents, wallets,
};
use crate::models::transaction::{
    NewExternalTransaction, NewInternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink, NewTransactionInternalTransactionLink,
};
use crate::models::wallet::{NewWallet, Wallet};
use crate::models::wallet_token_deposit_intent::{
    NewWalletTokenDepositIntent, WalletTokenDepositIntent, WALLET_DEPOSIT_STATUS_AMBIGUOUS,
    WALLET_DEPOSIT_STATUS_CREDITED, WALLET_DEPOSIT_STATUS_PENDING,
};
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::{env, str::FromStr};

pub const TOKEN_TRANSFER_OPERATION_DEPOSIT: &str = "deposit";
pub const TOKEN_TRANSFER_OPERATION_RETIRE: &str = "retire";
pub const TOKEN_TRANSFER_GAS_PAYER_USER: &str = "user";
pub const TOKEN_TRANSFER_GAS_PAYER_PLATFORM: &str = "platform";
pub const TOKEN_DEPOSIT_TRANSACTION_TYPE: &str = "token_deposit";
pub const TOKEN_RETIRE_TRANSACTION_TYPE: &str = "token_retire";
pub const TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK: &str = "metamask";
pub const TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM: &str = "platform";
pub const TOKEN_TRANSFER_EVENT_IMPORT: &str = "import";
pub const TOKEN_TRANSFER_EVENT_TRANSFER: &str = "transfer";

const TOKEN_DEPOSIT_TAX_KEY: &str = "wallet.deposit_tax_tokens";
const TOKEN_RETIRE_TAX_KEY: &str = "wallet.retire_tax_tokens";
const TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER: &str = "metamask_transfer";
const TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE: &str = "metamask_permit_signature";
const TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER: &str = "metamask_presigned_transfer";
const TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER: &str = "platform_transfer";

#[derive(Debug)]
pub struct LinkedWallet {
    pub wallet: Wallet,
    pub created: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetWalletTokenTaxRequest {
    pub tax_amount: BigDecimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTokenTransferRequest {
    pub amount: BigDecimal,
    pub ethereum_address: String,
    pub gas_payer: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub platform_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenTaxResponse {
    pub operation: String,
    pub tax_amount: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenTaxSettingsResponse {
    pub deposit: WalletTokenTaxResponse,
    pub retire: WalletTokenTaxResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenTransferResponse {
    pub operation: String,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub internal_transaction_ids: Vec<i64>,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WalletTokenDepositIntentResponse {
    pub operation: String,
    pub id: i64,
    pub status: String,
    pub wallet_id: i32,
    pub amount: String,
    pub tax_amount: String,
    pub wallet_delta_on_confirmation: String,
    pub gas_payer: String,
    pub ethereum_address: String,
    pub platform_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub log_index: Option<i64>,
    pub wallet_provider: String,
    pub metamask_required: bool,
    pub wallet_action: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObservedWalletDepositEvent {
    pub chain_id: i64,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: i64,
    pub event_type: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletDepositCreditResult {
    pub intent_id: Option<i64>,
    pub wallet_id: Option<i32>,
    pub transaction_id: Option<i64>,
    pub external_transaction_id: Option<i64>,
    pub internal_transaction_ids: Vec<i64>,
    pub credited: bool,
    pub status: String,
}

#[derive(Debug, PartialEq)]
pub enum WalletTokenTransferError {
    PermissionDenied(String),
    InvalidInput(String),
    InsufficientFunds,
    Database(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTokenOperation {
    Deposit,
    Retire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WalletTokenGasPayer {
    User,
    Platform,
}

impl WalletTokenOperation {
    fn as_str(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_TRANSFER_OPERATION_DEPOSIT,
            WalletTokenOperation::Retire => TOKEN_TRANSFER_OPERATION_RETIRE,
        }
    }

    fn tax_key(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_DEPOSIT_TAX_KEY,
            WalletTokenOperation::Retire => TOKEN_RETIRE_TAX_KEY,
        }
    }

    fn set_tax_permission(self) -> Permissions {
        match self {
            WalletTokenOperation::Deposit => Permissions::SET_DEPOSIT_TAX,
            WalletTokenOperation::Retire => Permissions::SET_RETIRE_TAX,
        }
    }

    fn transaction_type(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_DEPOSIT_TRANSACTION_TYPE,
            WalletTokenOperation::Retire => TOKEN_RETIRE_TRANSACTION_TYPE,
        }
    }

    fn external_event_type(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_TRANSFER_EVENT_IMPORT,
            WalletTokenOperation::Retire => TOKEN_TRANSFER_EVENT_TRANSFER,
        }
    }
}

impl WalletTokenGasPayer {
    fn parse(value: &str) -> Result<Self, WalletTokenTransferError> {
        match value.trim().to_ascii_lowercase().as_str() {
            TOKEN_TRANSFER_GAS_PAYER_USER => Ok(WalletTokenGasPayer::User),
            TOKEN_TRANSFER_GAS_PAYER_PLATFORM => Ok(WalletTokenGasPayer::Platform),
            _ => Err(WalletTokenTransferError::InvalidInput(
                "gas_payer must be 'user' or 'platform'".to_string(),
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            WalletTokenGasPayer::User => TOKEN_TRANSFER_GAS_PAYER_USER,
            WalletTokenGasPayer::Platform => TOKEN_TRANSFER_GAS_PAYER_PLATFORM,
        }
    }
}

impl From<DieselError> for WalletTokenTransferError {
    fn from(error: DieselError) -> Self {
        WalletTokenTransferError::Database(error.to_string())
    }
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}

pub async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn find_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .filter(wallets::user_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<LinkedWallet> {
    if let Some(wallet) = find_user_wallet(conn, user_id).await? {
        return Ok(LinkedWallet {
            wallet,
            created: false,
        });
    }

    let new_wallet = NewWallet {
        user_id: Some(user_id),
        organization_id: None,
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
            let wallet = find_user_wallet(conn, user_id)
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
    create_user_wallet_token_deposit_intent(conn, user_id, request).await
}

pub async fn retire_tokens_from_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    request: WalletTokenTransferRequest,
) -> Result<WalletTokenTransferResponse, WalletTokenTransferError> {
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

async fn create_user_wallet_token_deposit_intent(
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

pub async fn credit_observed_wallet_deposit(
    conn: &mut AsyncPgConnection,
    event: ObservedWalletDepositEvent,
) -> Result<WalletDepositCreditResult, WalletTokenTransferError> {
    validate_observed_wallet_deposit_event(&event)?;

    conn.transaction::<_, WalletTokenTransferError, _>(|conn| {
        Box::pin(async move {
            if let Some(existing_intent) =
                find_deposit_intent_by_chain_event(conn, &event, true).await?
            {
                if existing_intent.status == WALLET_DEPOSIT_STATUS_CREDITED {
                    return Ok(WalletDepositCreditResult {
                        intent_id: Some(existing_intent.id),
                        wallet_id: Some(existing_intent.wallet_id),
                        transaction_id: existing_intent.transaction_id,
                        external_transaction_id: existing_intent.external_transaction_id,
                        internal_transaction_ids: Vec::new(),
                        credited: false,
                        status: existing_intent.status,
                    });
                }

                if existing_intent.status != WALLET_DEPOSIT_STATUS_PENDING {
                    return Ok(WalletDepositCreditResult {
                        intent_id: Some(existing_intent.id),
                        wallet_id: Some(existing_intent.wallet_id),
                        transaction_id: existing_intent.transaction_id,
                        external_transaction_id: existing_intent.external_transaction_id,
                        internal_transaction_ids: Vec::new(),
                        credited: false,
                        status: existing_intent.status,
                    });
                }

                if !deposit_intent_matches_observed_event(&existing_intent, &event) {
                    log::warn!(
                        "event=wallet_token_deposit_event_mismatch intent_id={} chain_id={} tx_hash={} log_index={} event_type={}",
                        existing_intent.id,
                        event.chain_id,
                        event.transaction_hash,
                        event.log_index,
                        event.event_type
                    );
                    return Ok(WalletDepositCreditResult {
                        intent_id: Some(existing_intent.id),
                        wallet_id: Some(existing_intent.wallet_id),
                        transaction_id: None,
                        external_transaction_id: None,
                        internal_transaction_ids: Vec::new(),
                        credited: false,
                        status: "mismatched".to_string(),
                    });
                }
            }

            let candidates = load_matching_pending_deposit_intents(conn, &event).await?;
            if candidates.is_empty() {
                log::info!(
                    "event=wallet_token_deposit_event_unmatched chain_id={} tx_hash={} log_index={} from_address={} to_address={} amount={}",
                    event.chain_id,
                    event.transaction_hash,
                    event.log_index,
                    event.from_address,
                    event.to_address,
                    event.amount
                );
                return Ok(WalletDepositCreditResult {
                    intent_id: None,
                    wallet_id: None,
                    transaction_id: None,
                    external_transaction_id: None,
                    internal_transaction_ids: Vec::new(),
                    credited: false,
                    status: "unmatched".to_string(),
                });
            }

            if candidates.len() > 1 {
                mark_deposit_intents_ambiguous(conn, candidates.as_slice(), &event).await?;
                log::warn!(
                    "event=wallet_token_deposit_event_ambiguous chain_id={} tx_hash={} log_index={} candidate_count={}",
                    event.chain_id,
                    event.transaction_hash,
                    event.log_index,
                    candidates.len()
                );
                return Ok(WalletDepositCreditResult {
                    intent_id: None,
                    wallet_id: None,
                    transaction_id: None,
                    external_transaction_id: None,
                    internal_transaction_ids: Vec::new(),
                    credited: false,
                    status: WALLET_DEPOSIT_STATUS_AMBIGUOUS.to_string(),
                });
            }

            let intent = candidates
                .into_iter()
                .next()
                .expect("non-empty candidate list");
            let transaction_id =
                create_wallet_token_transaction(conn, WalletTokenOperation::Deposit).await?;
            let external_transaction_id =
                create_observed_deposit_external_transaction(conn, transaction_id, &event).await?;
            let internal_transaction_ids = apply_wallet_token_ledger_entries(
                conn,
                intent.wallet_id,
                transaction_id,
                WalletTokenOperation::Deposit,
                intent.amount.clone(),
                intent.tax_amount.clone(),
            )
            .await?;

            let credited_intent = mark_deposit_intent_credited(
                conn,
                intent.id,
                transaction_id,
                external_transaction_id,
                &event,
            )
            .await?;

            log::info!(
                "event=wallet_token_deposit_credited intent_id={} user_id={} wallet_id={} transaction_id={} external_transaction_id={} amount={} tax_amount={}",
                credited_intent.id,
                credited_intent.user_id,
                credited_intent.wallet_id,
                transaction_id,
                external_transaction_id,
                credited_intent.amount,
                credited_intent.tax_amount
            );

            Ok(WalletDepositCreditResult {
                intent_id: Some(credited_intent.id),
                wallet_id: Some(credited_intent.wallet_id),
                transaction_id: Some(transaction_id),
                external_transaction_id: Some(external_transaction_id),
                internal_transaction_ids,
                credited: true,
                status: credited_intent.status,
            })
        })
    })
    .await
}

fn validate_positive_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletTokenTransferError> {
    if amount <= &BigDecimal::from(0) {
        return Err(WalletTokenTransferError::InvalidInput(format!(
            "{} must be positive",
            field_name
        )));
    }
    Ok(())
}

fn validate_non_negative_amount(
    amount: &BigDecimal,
    field_name: &str,
) -> Result<(), WalletTokenTransferError> {
    if amount < &BigDecimal::from(0) {
        return Err(WalletTokenTransferError::InvalidInput(format!(
            "{} cannot be negative",
            field_name
        )));
    }
    Ok(())
}

fn validate_transfer_request_addresses(
    request: &WalletTokenTransferRequest,
) -> Result<(), WalletTokenTransferError> {
    if request.ethereum_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "ethereum_address is required".to_string(),
        ));
    }

    if request
        .platform_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "platform_address cannot be empty when provided".to_string(),
        ));
    }

    Ok(())
}

fn validate_external_transaction_fields(
    request: &WalletTokenTransferRequest,
) -> Result<(), WalletTokenTransferError> {
    if request
        .chain_id
        .map(|chain_id| chain_id <= 0)
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "chain_id must be positive when provided".to_string(),
        ));
    }
    if request
        .log_index
        .map(|log_index| log_index < 0)
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "log_index cannot be negative".to_string(),
        ));
    }
    if request
        .contract_address
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "contract_address cannot be empty when provided".to_string(),
        ));
    }
    if request
        .transaction_hash
        .as_ref()
        .map(|value| value.trim().is_empty())
        .unwrap_or(false)
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "transaction_hash cannot be empty when provided".to_string(),
        ));
    }

    Ok(())
}

fn validate_observed_wallet_deposit_event(
    event: &ObservedWalletDepositEvent,
) -> Result<(), WalletTokenTransferError> {
    if event.chain_id <= 0 {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed chain_id must be positive".to_string(),
        ));
    }
    if event.transaction_hash.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed transaction_hash is required".to_string(),
        ));
    }
    if event.log_index < 0 {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed log_index cannot be negative".to_string(),
        ));
    }
    if event.contract_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed contract_address is required".to_string(),
        ));
    }
    if event.from_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed from_address is required".to_string(),
        ));
    }
    if event.to_address.trim().is_empty() {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed to_address is required".to_string(),
        ));
    }
    validate_positive_amount(&event.amount, "observed amount")?;
    if ![TOKEN_TRANSFER_EVENT_IMPORT, TOKEN_TRANSFER_EVENT_TRANSFER]
        .contains(&event.event_type.as_str())
    {
        return Err(WalletTokenTransferError::InvalidInput(
            "observed event_type must be 'import' or 'transfer'".to_string(),
        ));
    }

    Ok(())
}

async fn configured_deposit_platform_address(
    conn: &mut AsyncPgConnection,
    gas_payer: WalletTokenGasPayer,
) -> Result<String, WalletTokenTransferError> {
    let configured = match gas_payer {
        WalletTokenGasPayer::User => env::var("WALLET_DEPOSIT_TREASURY_ADDRESS")
            .ok()
            .or_else(|| env::var("PLATFORM_TREASURY").ok()),
        WalletTokenGasPayer::Platform => get_persistent_state(conn, "platform_importer_address")
            .await?
            .or_else(|| env::var("WALLET_DEPOSIT_IMPORTER_ADDRESS").ok())
            .or_else(|| env::var("PLATFORM_IMPORTER_ADDRESS").ok()),
    };

    configured
        .map(|value| normalize_address(&value))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            WalletTokenTransferError::InvalidInput(
                "platform deposit receiver is not configured".to_string(),
            )
        })
}

async fn create_wallet_token_transaction(
    conn: &mut AsyncPgConnection,
    operation: WalletTokenOperation,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: operation.transaction_type(),
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
}

async fn create_wallet_external_transaction(
    conn: &mut AsyncPgConnection,
    operation: WalletTokenOperation,
    transaction_id: i64,
    request: &WalletTokenTransferRequest,
) -> QueryResult<i64> {
    let ethereum_address = request.ethereum_address.trim();
    let platform_address = request.platform_address.as_deref().map(str::trim);
    let (from_address, to_address) = match operation {
        WalletTokenOperation::Deposit => (Some(ethereum_address), platform_address),
        WalletTokenOperation::Retire => (platform_address, Some(ethereum_address)),
    };

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: request.amount.clone(),
            blockchain_address: ethereum_address,
            chain_id: request.chain_id,
            contract_address: request.contract_address.as_deref().map(str::trim),
            transaction_hash: request.transaction_hash.as_deref().map(str::trim),
            log_index: request.log_index,
            event_type: Some(operation.external_event_type()),
            from_address,
            to_address,
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(external_transaction_id)
}

async fn create_observed_deposit_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<i64> {
    let from_address = normalize_address(&event.from_address);
    let to_address = normalize_address(&event.to_address);
    let contract_address = normalize_address(&event.contract_address);
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: event.amount.clone(),
            blockchain_address: &from_address,
            chain_id: Some(event.chain_id),
            contract_address: Some(&contract_address),
            transaction_hash: Some(&transaction_hash),
            log_index: Some(event.log_index),
            event_type: Some(event.event_type.as_str()),
            from_address: Some(&from_address),
            to_address: Some(&to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(external_transaction_id)
}

async fn find_deposit_intent_by_chain_event(
    conn: &mut AsyncPgConnection,
    event: &ObservedWalletDepositEvent,
    for_update: bool,
) -> QueryResult<Option<WalletTokenDepositIntent>> {
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();
    let query = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)))
        .filter(wallet_token_deposit_intents::transaction_hash.eq(Some(transaction_hash)))
        .filter(wallet_token_deposit_intents::log_index.eq(Some(event.log_index)));

    if for_update {
        query.for_update().first(conn).await.optional()
    } else {
        query.first(conn).await.optional()
    }
}

async fn load_matching_pending_deposit_intents(
    conn: &mut AsyncPgConnection,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<Vec<WalletTokenDepositIntent>> {
    if let Some(intent) = find_deposit_intent_by_chain_event(conn, event, true).await? {
        if deposit_intent_matches_observed_event(&intent, event) {
            return Ok(vec![intent]);
        }
        return Ok(Vec::new());
    }

    let from_address = normalize_address(&event.from_address);
    let to_address = normalize_address(&event.to_address);

    let pending_intents = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_PENDING))
        .filter(wallet_token_deposit_intents::ethereum_address.eq(from_address))
        .filter(wallet_token_deposit_intents::platform_address.eq(to_address))
        .filter(wallet_token_deposit_intents::amount.eq(event.amount.clone()))
        .order(wallet_token_deposit_intents::created_at.asc())
        .for_update()
        .load::<WalletTokenDepositIntent>(conn)
        .await?;

    Ok(pending_intents
        .into_iter()
        .filter(|intent| deposit_intent_matches_observed_event(intent, event))
        .collect())
}

fn deposit_intent_matches_observed_event(
    intent: &WalletTokenDepositIntent,
    event: &ObservedWalletDepositEvent,
) -> bool {
    let expected_event_type = match intent.gas_payer.as_str() {
        TOKEN_TRANSFER_GAS_PAYER_USER => TOKEN_TRANSFER_EVENT_TRANSFER,
        TOKEN_TRANSFER_GAS_PAYER_PLATFORM => TOKEN_TRANSFER_EVENT_IMPORT,
        _ => return false,
    };
    let event_transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();
    let event_contract_address = normalize_address(&event.contract_address);

    intent.status == WALLET_DEPOSIT_STATUS_PENDING
        && event.event_type == expected_event_type
        && addresses_equal(&intent.ethereum_address, &event.from_address)
        && addresses_equal(&intent.platform_address, &event.to_address)
        && intent.amount == event.amount
        && intent
            .chain_id
            .map(|chain_id| chain_id == event.chain_id)
            .unwrap_or(true)
        && intent
            .contract_address
            .as_deref()
            .map(|address| addresses_equal(address, &event_contract_address))
            .unwrap_or(true)
        && intent
            .transaction_hash
            .as_deref()
            .map(|hash| hash.eq_ignore_ascii_case(&event_transaction_hash))
            .unwrap_or(true)
        && intent
            .log_index
            .map(|log_index| log_index == event.log_index)
            .unwrap_or(true)
        && intent
            .event_type
            .as_deref()
            .map(|event_type| event_type == event.event_type.as_str())
            .unwrap_or(true)
}

async fn mark_deposit_intents_ambiguous(
    conn: &mut AsyncPgConnection,
    intents: &[WalletTokenDepositIntent],
    event: &ObservedWalletDepositEvent,
) -> QueryResult<usize> {
    let ids = intents.iter().map(|intent| intent.id).collect::<Vec<_>>();
    diesel::update(
        wallet_token_deposit_intents::table.filter(
            wallet_token_deposit_intents::id
                .eq_any(ids)
                .and(wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_PENDING)),
        ),
    )
    .set((
        wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_AMBIGUOUS),
        wallet_token_deposit_intents::transaction_hash
            .eq(Some(event.transaction_hash.trim().to_ascii_lowercase())),
        wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)),
        wallet_token_deposit_intents::log_index.eq(Some(event.log_index)),
        wallet_token_deposit_intents::last_error.eq(Some(
            "multiple pending deposit intents matched the same chain event".to_string(),
        )),
        wallet_token_deposit_intents::updated_at.eq(chrono::Utc::now()),
    ))
    .execute(conn)
    .await
}

async fn mark_deposit_intent_credited(
    conn: &mut AsyncPgConnection,
    intent_id: i64,
    transaction_id: i64,
    external_transaction_id: i64,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<WalletTokenDepositIntent> {
    diesel::update(wallet_token_deposit_intents::table.find(intent_id))
        .set((
            wallet_token_deposit_intents::status.eq(WALLET_DEPOSIT_STATUS_CREDITED),
            wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)),
            wallet_token_deposit_intents::contract_address
                .eq(Some(normalize_address(&event.contract_address))),
            wallet_token_deposit_intents::transaction_hash
                .eq(Some(event.transaction_hash.trim().to_ascii_lowercase())),
            wallet_token_deposit_intents::log_index.eq(Some(event.log_index)),
            wallet_token_deposit_intents::event_type.eq(Some(event.event_type.clone())),
            wallet_token_deposit_intents::external_transaction_id.eq(Some(external_transaction_id)),
            wallet_token_deposit_intents::transaction_id.eq(Some(transaction_id)),
            wallet_token_deposit_intents::last_error.eq(None::<String>),
            wallet_token_deposit_intents::updated_at.eq(chrono::Utc::now()),
            wallet_token_deposit_intents::credited_at.eq(Some(chrono::Utc::now())),
        ))
        .get_result(conn)
        .await
}

async fn apply_wallet_token_ledger_entries(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    operation: WalletTokenOperation,
    amount: BigDecimal,
    tax_amount: BigDecimal,
) -> Result<Vec<i64>, WalletTokenTransferError> {
    let mut internal_transaction_ids = Vec::new();

    match operation {
        WalletTokenOperation::Deposit => {
            internal_transaction_ids.push(
                apply_wallet_token_ledger_entry(conn, wallet_id, transaction_id, amount).await?,
            );
            if tax_amount > BigDecimal::from(0) {
                internal_transaction_ids.push(
                    apply_wallet_token_ledger_entry(conn, wallet_id, transaction_id, -tax_amount)
                        .await?,
                );
            }
        }
        WalletTokenOperation::Retire => {
            internal_transaction_ids.push(
                apply_wallet_token_ledger_entry(conn, wallet_id, transaction_id, -amount).await?,
            );
            if tax_amount > BigDecimal::from(0) {
                internal_transaction_ids.push(
                    apply_wallet_token_ledger_entry(conn, wallet_id, transaction_id, -tax_amount)
                        .await?,
                );
            }
        }
    }

    Ok(internal_transaction_ids)
}

async fn apply_wallet_token_ledger_entry(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    transaction_id: i64,
    amount: BigDecimal,
) -> Result<i64, WalletTokenTransferError> {
    let updated = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount.clone()))
    .execute(conn)
    .await?;

    if updated == 0 {
        return Err(WalletTokenTransferError::InsufficientFunds);
    }

    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(internal_transaction_id)
}

fn normalize_address(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn addresses_equal(left: &str, right: &str) -> bool {
    normalize_address(left) == normalize_address(right)
}

fn wallet_delta_for_operation(
    operation: WalletTokenOperation,
    amount: BigDecimal,
    tax_amount: BigDecimal,
) -> BigDecimal {
    match operation {
        WalletTokenOperation::Deposit => amount - tax_amount,
        WalletTokenOperation::Retire => -(amount + tax_amount),
    }
}

#[derive(Debug, Clone, Copy)]
struct WalletTransferInteraction {
    provider: &'static str,
    metamask_required: bool,
    action: &'static str,
}

fn wallet_interaction_for_transfer(
    operation: WalletTokenOperation,
    gas_payer: WalletTokenGasPayer,
) -> WalletTransferInteraction {
    match (operation, gas_payer) {
        (WalletTokenOperation::Deposit, WalletTokenGasPayer::User) => WalletTransferInteraction {
            provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
            metamask_required: true,
            action: TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER,
        },
        (WalletTokenOperation::Deposit, WalletTokenGasPayer::Platform) => {
            WalletTransferInteraction {
                provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
                metamask_required: true,
                action: TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE,
            }
        }
        (WalletTokenOperation::Retire, WalletTokenGasPayer::User) => WalletTransferInteraction {
            provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
            metamask_required: true,
            action: TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER,
        },
        (WalletTokenOperation::Retire, WalletTokenGasPayer::Platform) => {
            WalletTransferInteraction {
                provider: TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM,
                metamask_required: false,
                action: TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER,
            }
        }
    }
}
