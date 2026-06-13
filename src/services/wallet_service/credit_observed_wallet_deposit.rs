pub async fn credit_observed_wallet_deposit(
    conn: &mut AsyncPgConnection,
    event: ObservedWalletDepositEvent,
) -> Result<WalletDepositCreditResult, WalletTokenTransferError> {
    let mut store = PostgresWalletDepositIndexStore::new(conn);
    crate::application::wallet::index_deposit::index_observed_deposit(&mut store, event)
        .await
        .map_err(WalletTokenTransferError::from)
}
