fn validate_compensation_request(
    request: &RewardCompensationRequest,
) -> Result<(), RewardCompensationError> {
    if request.amount == BigDecimal::from(0) {
        return Err(RewardCompensationError::InvalidInput(
            "compensation amount cannot be zero".to_string(),
        ));
    }
    if request.reason.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation reason is required".to_string(),
        ));
    }
    if request.idempotency_key.trim().is_empty() {
        return Err(RewardCompensationError::InvalidInput(
            "compensation idempotency key is required".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests;

async fn apply_wallet_adjustment(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<Wallet, RewardCompensationError> {
    let updated_wallet = diesel::update(
        wallets::table.filter(
            wallets::id
                .eq(wallet_id)
                .and((wallets::value + amount.clone()).ge(BigDecimal::from(0))),
        ),
    )
    .set(wallets::value.eq(wallets::value + amount))
    .get_result::<Wallet>(conn)
    .await;

    match updated_wallet {
        Ok(wallet) => Ok(wallet),
        Err(diesel::result::Error::NotFound) => Err(RewardCompensationError::InsufficientFunds),
        Err(error) => Err(RewardCompensationError::from(error)),
    }
}

async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> Result<i64, RewardCompensationError> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .map_err(RewardCompensationError::from)
}

async fn create_compensation_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> Result<i64, RewardCompensationError> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_COMPENSATION,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(transaction_id)
}
