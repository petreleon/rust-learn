use diesel::result::Error as DieselError;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::application::wallet::link_wallet::WalletLinkError;
use crate::application::wallet::retire_tokens::{
    WalletRetirementDraft, WalletRetirementError, WalletRetirementView,
};
use crate::db::schema::{external_transactions, transactions, transactions_external_transactions};
use crate::infra::postgres::wallet::wallet_link_records::link_user_wallet_record;
use crate::infra::postgres::wallet::wallet_retirement_ledger::apply_retirement_ledger_entries;
use crate::models::transaction::{
    NewExternalTransaction, NewTransaction, NewTransactionExternalTransactionLink,
};

const RETIRE_OPERATION: &str = "retire";
const RETIRE_TRANSACTION_TYPE: &str = "token_retire";
const RETIRE_EXTERNAL_EVENT_TYPE: &str = "transfer";

impl From<DieselError> for WalletRetirementError {
    fn from(error: DieselError) -> Self {
        WalletRetirementError::RetirementCreate(error.to_string())
    }
}

pub(super) async fn create_wallet_retirement(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    draft: WalletRetirementDraft,
) -> Result<WalletRetirementView, WalletRetirementError> {
    conn.transaction::<_, WalletRetirementError, _>(|conn| {
        Box::pin(async move {
            let wallet = link_user_wallet_record(conn, user_id)
                .await
                .map_err(map_wallet_link_error)?
                .wallet;
            let transaction_id = create_retirement_transaction(conn).await?;
            let external_transaction_id =
                create_retirement_external_transaction(conn, transaction_id, &draft).await?;
            let internal_transaction_ids = apply_retirement_ledger_entries(
                conn,
                wallet.id,
                transaction_id,
                draft.amount.clone(),
                draft.tax_amount.clone(),
            )
            .await?;
            let wallet_delta = -(draft.amount.clone() + draft.tax_amount.clone());

            log::info!(
                "event=wallet_tokens_retired user_id={} wallet_id={} amount={} tax_amount={} gas_payer={} wallet_provider={} metamask_required={} transaction_id={} external_transaction_id={}",
                user_id,
                wallet.id,
                draft.amount,
                draft.tax_amount,
                draft.gas_payer,
                draft.wallet_provider,
                draft.metamask_required,
                transaction_id,
                external_transaction_id
            );

            Ok(WalletRetirementView {
                operation: RETIRE_OPERATION,
                wallet_id: wallet.id,
                transaction_id,
                external_transaction_id,
                internal_transaction_ids,
                amount: draft.amount.to_string(),
                tax_amount: draft.tax_amount.to_string(),
                wallet_delta: wallet_delta.to_string(),
                gas_payer: draft.gas_payer,
                ethereum_address: draft.ethereum_address,
                wallet_provider: draft.wallet_provider,
                metamask_required: draft.metamask_required,
                wallet_action: draft.wallet_action,
            })
        })
    })
    .await
}

async fn create_retirement_transaction(
    conn: &mut AsyncPgConnection,
) -> Result<i64, WalletRetirementError> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: RETIRE_TRANSACTION_TYPE,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletRetirementError::from)
}

async fn create_retirement_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    draft: &WalletRetirementDraft,
) -> Result<i64, WalletRetirementError> {
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: draft.amount.clone(),
            blockchain_address: &draft.ethereum_address,
            chain_id: draft.chain_id,
            contract_address: draft.contract_address.as_deref(),
            transaction_hash: draft.transaction_hash.as_deref(),
            log_index: draft.log_index,
            event_type: Some(RETIRE_EXTERNAL_EVENT_TYPE),
            from_address: draft.platform_address.as_deref(),
            to_address: Some(&draft.ethereum_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .map_err(WalletRetirementError::from)?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await
        .map_err(WalletRetirementError::from)?;

    Ok(external_transaction_id)
}

fn map_wallet_link_error(error: WalletLinkError) -> WalletRetirementError {
    match error {
        WalletLinkError::WalletCreate(message) => WalletRetirementError::WalletCreate(message),
        WalletLinkError::WalletLoad(message) => WalletRetirementError::WalletLoad(message),
        other => WalletRetirementError::WalletLoad(format!("{:?}", other)),
    }
}
