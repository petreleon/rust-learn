use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositIntentDraft, WalletDepositIntentError, WalletDepositIntentView,
};
use crate::application::wallet::link_wallet::WalletLinkError;
use crate::db::schema::wallet_token_deposit_intents;
use crate::infra::postgres::wallet::wallet_link_records::link_user_wallet_record;
use crate::models::wallet_token_deposit_intent::{
    NewWalletTokenDepositIntent, WalletTokenDepositIntent, WALLET_DEPOSIT_STATUS_PENDING,
};

impl From<DieselError> for WalletDepositIntentError {
    fn from(error: DieselError) -> Self {
        WalletDepositIntentError::DepositIntentCreate(error.to_string())
    }
}

pub(super) async fn insert_deposit_intent(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    draft: WalletDepositIntentDraft,
) -> Result<WalletDepositIntentView, WalletDepositIntentError> {
    let intent = conn
        .transaction::<_, WalletDepositIntentError, _>(|conn| {
            Box::pin(async move {
                let wallet = link_user_wallet_record(conn, user_id)
                    .await
                    .map_err(map_wallet_link_error)?
                    .wallet;
                diesel::insert_into(wallet_token_deposit_intents::table)
                    .values(NewWalletTokenDepositIntent {
                        user_id,
                        wallet_id: wallet.id,
                        ethereum_address: draft.ethereum_address,
                        platform_address: draft.platform_address,
                        amount: draft.amount,
                        gas_payer: draft.gas_payer,
                        tax_amount: draft.tax_amount,
                        status: WALLET_DEPOSIT_STATUS_PENDING.to_string(),
                        chain_id: draft.chain_id,
                        contract_address: draft.contract_address,
                        transaction_hash: draft.transaction_hash,
                        log_index: draft.log_index,
                        wallet_provider: draft.wallet_provider,
                        metamask_required: draft.metamask_required,
                        wallet_action: draft.wallet_action,
                    })
                    .get_result::<WalletTokenDepositIntent>(conn)
                    .await
                    .map_err(|error| {
                        WalletDepositIntentError::DepositIntentCreate(error.to_string())
                    })
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

    Ok(WalletDepositIntentView::from(intent))
}

fn map_wallet_link_error(error: WalletLinkError) -> WalletDepositIntentError {
    match error {
        WalletLinkError::WalletCreate(message) => WalletDepositIntentError::WalletCreate(message),
        WalletLinkError::WalletLoad(message) => WalletDepositIntentError::WalletLoad(message),
        other => WalletDepositIntentError::WalletLoad(format!("{:?}", other)),
    }
}

impl From<WalletTokenDepositIntent> for WalletDepositIntentView {
    fn from(intent: WalletTokenDepositIntent) -> Self {
        Self {
            operation: "deposit",
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
