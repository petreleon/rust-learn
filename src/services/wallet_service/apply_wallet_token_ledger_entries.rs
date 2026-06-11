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

#[cfg(test)]
mod tests;
